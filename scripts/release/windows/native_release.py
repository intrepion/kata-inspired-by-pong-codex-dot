"""Prepare or build the Windows x64 SDL3/raylib packaging inputs, not a public ZIP."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import struct
import stat
import subprocess
import sys
import tomllib

from native_source import CONFIG, prepare, sha
from package import TARGET, TOOLCHAIN, reviewed_manifest, verified_inputs, verify_compiler, verify_pe_x64

HEX = re.compile(r"[0-9a-f]{40}")
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
ENV_BLOCK = {"CL", "_CL_", "CFLAGS", "CXXFLAGS", "CMAKE_TOOLCHAIN_FILE",
             "CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS", "RUSTC", "RUSTC_WRAPPER",
             "RUSTC_WORKSPACE_WRAPPER", "LIBCLANG_PATH", "BINDGEN_EXTRA_CLANG_ARGS"}


def git(repo, *args):
    return subprocess.check_output(["git", "-C", str(repo), *args], stderr=subprocess.PIPE)


def check_checkout(repo, commit, tree):
    if not (HEX.fullmatch(commit) and HEX.fullmatch(tree)):
        raise ValueError("provide full independently reviewed commit and tree")
    if (git(repo, "rev-parse", "--show-toplevel").decode().strip().replace("\\", "/").lower()
            != repo.as_posix().lower()
            or git(repo, "rev-parse", "HEAD").decode().strip() != commit
            or git(repo, "rev-parse", "HEAD^{tree}").decode().strip() != tree
            or git(repo, "status", "--porcelain", "--untracked-files=all", "--ignored")):
        raise ValueError("not the exact clean reviewed checkout")
    if any(not row.startswith(b"H ") for row in git(repo, "ls-files", "-v").splitlines()):
        raise ValueError("special index flags are not valid release inputs")


def isolated_lock(data, name, version, digest):
    lf = data.replace(b"\r\n", b"\n")
    expected = f'name = "{name}"\nversion = "{version}"\nsource = "{REGISTRY}"\nchecksum = "{digest}"\n'.encode()
    new = expected.split(b"source = ", 1)[0]
    if lf.count(expected) != 1:
        raise ValueError("expected one exact registry lock identity")
    transform = lf.replace(expected, new)
    a, b = (tomllib.loads(x.decode("utf-8")) for x in (lf, transform))
    rows = [p for p in a["package"] if (p["name"], p["version"], p.get("source")) == (name, version, REGISTRY)]
    if len(rows) != 1 or rows[0].pop("checksum") != digest:
        raise ValueError("unverified original lock identity")
    del rows[0]["source"]
    if a != b:
        raise ValueError("isolated lock contains other changes")
    return transform


def privacy(data, roots):
    for marker in {str(x) for x in roots} | {x.as_posix() for x in roots} | {"C:\\Users\\", "C:/Users/"}:
        if any(marker.lower().encode(enc) in data.lower() for enc in ("utf-8", "utf-16le")):
            raise ValueError("executable contains an unredacted local path")


def run(args):
    if any(k.startswith("GIT_") and k not in {"GIT_PAGER", "GIT_TERMINAL_PROMPT"} for k in os.environ):
        raise ValueError("unexpected ambient Git configuration")
    repo, archive = args.repo.resolve(strict=True), args.crate.resolve(strict=True)
    out = args.output.parent.resolve(strict=True) / args.output.name
    roots = [repo, Path.home().resolve() / ".cargo", Path.home().resolve() / ".rustup"]
    if out.exists() or out.is_symlink() or any(out.is_relative_to(root) for root in roots):
        raise ValueError("use a fresh scratch output outside the repository and global cache")
    check_checkout(repo, args.commit, args.tree)
    if args.build and (os.name != "nt" or not args.cargo_home):
        raise ValueError("Windows build requires an explicitly prepared isolated offline CARGO_HOME")
    env = os.environ.copy()
    bad = {k for k in env if k in ENV_BLOCK or k == "CC" or k == "CXX" or k.startswith(
        ("CARGO_BUILD_", "CARGO_TARGET_", "CC_", "CXX_", "CFLAGS_", "CXXFLAGS_", "CMAKE_")
    )}
    if args.build and bad:
        raise ValueError("unexpected ambient compiler configuration: " + ", ".join(sorted(bad)))
    kind = args.kind
    name, version, lock_rel, digest, _, _, result_sha = CONFIG[kind]
    out.mkdir()
    success = False
    try:
        prepare(kind, repo, archive, out / "vendor")
        scratch = out / "checkout"
        subprocess.run(["git", "clone", "--no-hardlinks", "--no-local", "--no-checkout", str(repo), str(scratch)], check=True)
        subprocess.run(["git", "-C", str(scratch), "checkout", "--detach", args.commit], check=True)
        check_checkout(scratch, args.commit, args.tree)
        crate = (out / "vendor" / (name + "-" + version)).resolve(strict=True)
        orig = (scratch / lock_rel).read_bytes()
        if orig.replace(b"\r\n", b"\n") != git(repo, "show", "HEAD:" + lock_rel):
            raise ValueError("working lock differs from exact Git object")
        modified = isolated_lock(orig, name, version, digest)
        (scratch / lock_rel).write_bytes(modified)
        status = git(scratch, "status", "--porcelain", "--untracked-files=all")
        if status != f" M {lock_rel}\n".encode():
            raise ValueError("unexpected change in isolated checkout")
        info = {"base_commit": args.commit, "base_tree": args.tree, "target": TARGET,
                "archive_sha256": digest, "transformed_source_sha256": result_sha,
                "original_lock_sha256": sha(orig.replace(b"\r\n", b"\n")), "isolated_lock_sha256": sha(modified),
                "scope": "Windows release packaging inputs; native notices, runtime and benchmarks require separate qualification."}
        if args.build:
            cargo_home = args.cargo_home.resolve(strict=True)
            if not cargo_home.is_dir() or any(cargo_home.is_relative_to(x) or x.is_relative_to(cargo_home) for x in roots + [out]):
                raise ValueError("CARGO_HOME must be separate from all source, output and global caches")
            for location in [scratch, *scratch.parents, cargo_home]:
                config_root = location if location == cargo_home else location / ".cargo"
                if any((config_root / n).exists() for n in ("config.toml", "config")):
                    raise ValueError("unexpected ambient Cargo configuration")
            if any(c in str(crate) for c in ' ="'):
                raise ValueError("use a short scratch path without spaces, quotes or equals")
            env.update(CARGO_HOME=str(cargo_home), RUSTUP_TOOLCHAIN=TOOLCHAIN,
                       CARGO_NET_OFFLINE="true", CARGO_INCREMENTAL="0")
            identity = subprocess.check_output(["rustc", "-Vv"], env=env, cwd=scratch, text=True)
            inputs = scratch / "scripts/release/windows"
            reviewed = reviewed_manifest((inputs / "inputs-manifest.json").read_bytes())
            verify_compiler(identity, reviewed)
            verified_inputs(reviewed, inputs / "inputs", args.kind)
            env["CL"] = f"/experimental:deterministic /pathmap:{crate}=/pong-{kind}-source"
            rust_roots = [out, cargo_home]
            env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(
                f"--remap-path-prefix={form}=/pong-build"
                for root in rust_roots for form in (str(root), root.as_posix())
            )
            if kind == "sdl3":
                env["CMAKE_TOOLCHAIN_FILE"] = str(scratch / "scripts/release/windows/sdl3-keyboard-only.cmake")
            else:
                if not args.clang:
                    raise ValueError("raylib needs path to an existing clang.exe alongside libclang.dll")
                clang = args.clang.resolve(strict=True)
                if not (clang.parent / "libclang.dll").is_file():
                    raise ValueError("matching libclang.dll missing")
                resource = Path(subprocess.check_output([str(clang), "-print-resource-dir"], text=True, env=env).strip()).resolve(strict=True)
                if not (resource / "include/stdbool.h").is_file():
                    raise ValueError("clang resource headers missing")
                env["LIBCLANG_PATH"] = str(clang.parent)
                env["BINDGEN_EXTRA_CLANG_ARGS"] = f'-resource-dir "{resource.as_posix()}"'
            mapfile = out / "native.link.map"
            target = out / "target"
            manifest = str(Path(lock_rel).with_name("Cargo.toml"))
            exe = "pong-" + ("sdl3" if kind == "sdl3" else "raylib-rs")
            config = f"patch.crates-io.{name}.path=" + json.dumps(crate.as_posix())
            with (out / "build.jsonl").open("xb") as log:
                subprocess.run(["cargo", "--config", config, "rustc", "--locked", "--offline",
                                "--release", "--target", TARGET, "--target-dir", str(target), "--manifest-path",
                                manifest, "--bin", exe, "--message-format=json", "--", f"-Clink-arg=/MAP:{mapfile}"],
                               env=env, cwd=scratch, check=True, stdout=log)
            binary = (target / TARGET / "release" / (exe + ".exe")).read_bytes()
            verify_pe_x64(binary)
            privacy(binary, [out, cargo_home, Path.home()])
            mapped = mapfile.read_bytes()
            offset = struct.unpack_from("<I", binary, 60)[0]
            stamp = re.search(br"Timestamp is ([0-9a-fA-F]+)", mapped)
            if not stamp or int(stamp[1], 16) != struct.unpack_from("<I", binary, offset + 8)[0]:
                raise ValueError("linker map timestamp does not match PE")
            prohibited = (b"SDL_c_dfDIJoystick2", b"Lg4ff", b"__imp_AttachThreadInput") if kind == "sdl3" else (
                b"Win32GetClipboardImageData", b"GetPixelDataOffset", b"OpenClipboardRetrying")
            if any(x.lower() in mapped.lower() for x in prohibited):
                raise ValueError("a specifically excluded native symbol survived")
            if git(scratch, "status", "--porcelain", "--untracked-files=all") != status or (scratch / lock_rel).read_bytes() != modified:
                raise ValueError("unexpected checkout drift after the build")
            info.update(binary_sha256=sha(binary), map_sha256=sha(mapped), compiler=identity)
        (out / "receipt.json").write_text(json.dumps(info, indent=2) + "\n", encoding="utf8")
        success = True
    finally:
        if not success:
            if out.resolve(strict=True) == out and not out.is_symlink() and not out.is_junction() and out.is_dir():
                remove_scratch(out)
            else:
                raise RuntimeError("scratch changed identity; refusing cleanup")



def remove_scratch(out):
    def on_readonly(func, path, exc):
        p = Path(path)
        attrs = getattr(p.lstat(), "st_file_attributes", 0)
        if (not isinstance(exc, PermissionError) or not p.is_file() or p.is_symlink()
                or getattr(p, "is_junction")() or not p.resolve(strict=True).is_relative_to(out)
                or not attrs & stat.FILE_ATTRIBUTE_READONLY
                or attrs & stat.FILE_ATTRIBUTE_REPARSE_POINT):
            raise exc
        # Git marks packs in our clone read-only. Clear only this file attribute
        # in our own scratch clone, never change ACLs or retry a policy denial.
        p.chmod(stat.S_IWRITE)
        func(path)
    shutil.rmtree(out, onexc=on_readonly)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("kind", choices=CONFIG)
    for n in ("repo", "crate", "output"): p.add_argument(n, type=Path)
    p.add_argument("--commit", required=True)
    p.add_argument("--tree", required=True)
    p.add_argument("--build", action="store_true")
    p.add_argument("--cargo-home", type=Path)
    p.add_argument("--clang", type=Path)
    args = p.parse_args()
    try:
        run(args)
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        p.exit(1, f"Native preparation failed: {exc}\n")


if __name__ == "__main__":
    main()
