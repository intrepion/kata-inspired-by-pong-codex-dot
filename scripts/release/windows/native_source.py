"""Prepare a pinned local source override; does not install or build anything."""
import argparse
import hashlib
import pathlib
import tarfile
import tomllib

CONFIG = {
    "sdl3": (
        "sdl3-src", "3.4.20", "experiments/sdl3/Cargo.lock",
        "dc5181b85760f18170dbc15b27b00d5aaaa003058bb14333ec7dd280665ef3a4",
        "SDL/src/video/windows/SDL_windowswindow.c",
        "6bbd1337837b84d1215563433a82130de2d7d24660d29fc04588d90b03629c5a",
        "c1a38342d9f92881d55c02c77820604f4cc3664b0510506221e0a5cef8ef1847",
    ),
    "raylib": (
        "raylib-sys", "6.0.0", "experiments/raylib-rs/Cargo.lock",
        "36be0f95d9ea818847eedf4a6a73cbd5331db5a558069747dae827ac3c12fdf8",
        "raylib/src/platforms/rcore_desktop_glfw.c",
        "c2a2ea9a4ade6ca80ad3fcf266c7d370353910d94b15336e8cf5954a3fc70379",
        "e7105440feec4df9d3a146b7f114541b8e8637fc9f80b766e131e93e43b05486",
    ),
}
sha = lambda b: hashlib.sha256(b).hexdigest()

def transform(kind, data):
    if kind == "sdl3":
        spans = ((1098, 1106), (1108, 1111), (1115, 1125), (1133, 1137))
        return b"".join(row for i, row in enumerate(data.splitlines(keepends=True), 1)
                        if not any(lo <= i <= hi for lo, hi in spans))
    decl = b'    #include "../external/win32_clipboard.h" // Clipboard image copy-paste'
    implementation = b"#ifdef _WIN32\n#   define WIN32_CLIPBOARD_IMPLEMENTATION"
    if data.count(decl) != 1 or data.count(implementation) != 1:
        raise ValueError("raylib include anchors differ")
    disabled = (
        b"    // Pong Windows release intentionally omits clipboard images, not clipboard text.\n"
        b"    #undef SUPPORT_CLIPBOARD_IMAGE\n"
        b"    #define SUPPORT_CLIPBOARD_IMAGE 0\n"
        b"#if SUPPORT_CLIPBOARD_IMAGE && SUPPORT_MODULE_RTEXTURES\n"
        + decl + b"\n#endif"
    )
    return data.replace(decl, disabled).replace(
        implementation,
        b"#if defined(_WIN32) && SUPPORT_CLIPBOARD_IMAGE && SUPPORT_MODULE_RTEXTURES\n"
        b"#   define WIN32_CLIPBOARD_IMPLEMENTATION",
    )

def prepare(kind, clean_repo, archive, out):
    name, version, lock, archive_sha, member, original_sha, changed_sha = CONFIG[kind]
    packages = tomllib.loads((clean_repo / lock).read_text(encoding="utf-8"))["package"]
    matching = [p for p in packages if p["name"] == name and p["version"] == version]
    if len(matching) != 1 or matching[0].get("checksum") != archive_sha:
        raise ValueError("exact pristine Cargo lock required")
    with archive.open("rb") as src:
        if hashlib.file_digest(src, "sha256").hexdigest() != archive_sha:
            raise ValueError("locked archive digest mismatch")
        src.seek(0)
        with tarfile.open(fileobj=src) as tar:
            root = f"{name}-{version}"
            files = tar.getmembers()
            seen = set()
            for entry in files:
                p = pathlib.PurePosixPath(entry.name)
                key = p.as_posix().casefold()
                if (p.is_absolute() or ".." in p.parts or not p.parts or p.parts[0] != root
                    or "\\" in entry.name or ":" in entry.name or key in seen
                    or not (entry.isfile() or entry.isdir())):
                    raise ValueError("unexpected archive member")
                seen.add(key)
            pick = tar.getmember(root + "/" + member)
            handle = tar.extractfile(pick)
            if handle is None:
                raise ValueError("missing original source")
            source = handle.read()
            if sha(source) != original_sha:
                raise ValueError("original source digest mismatch")
            changed = transform(kind, source)
            if sha(changed) != changed_sha:
                raise ValueError("transformed source digest mismatch")
            out.mkdir(exist_ok=False)
            tar.extractall(out, members=files, filter="data")
            dest = out / root / member
            if sha(dest.read_bytes()) != original_sha:
                raise ValueError("extracted source differs")
            dest.write_bytes(changed)
            print(f"{name} {version} {changed_sha}")

if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("kind", choices=CONFIG)
    for arg in ("pristine_repo", "crate_archive", "fresh_output"):
        p.add_argument(arg, type=pathlib.Path)
    args = p.parse_args()
    prepare(args.kind, args.pristine_repo, args.crate_archive, args.fresh_output)
