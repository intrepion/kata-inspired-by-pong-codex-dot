use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo:rerun-if-changed=src/backends/cg_appkit.m");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let object = out.join("cg_appkit.o");
    let library = out.join("libsoftbuffer_native_appkit.a");
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86_64") => "x86_64",
        _ => panic!("unsupported macOS architecture"),
    };
    let mut clang = Command::new("xcrun");
    clang.args([
        "--sdk",
        "macosx",
        "clang",
        "-arch",
        arch,
        "-fobjc-arc",
        "-O2",
    ]);
    if let Ok(v) = env::var("MACOSX_DEPLOYMENT_TARGET") {
        clang.arg(format!("-mmacosx-version-min={v}"));
    } else {
        clang.arg("-mmacosx-version-min=11.0");
    }
    assert!(clang
        .args(["-c", "src/backends/cg_appkit.m", "-o"])
        .arg(&object)
        .status()
        .expect("run Apple clang")
        .success());
    assert!(Command::new("xcrun")
        .args(["ar", "crs"])
        .arg(library)
        .arg(object)
        .status()
        .expect("run Apple ar")
        .success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=softbuffer_native_appkit");
    for fw in ["AppKit", "QuartzCore", "CoreGraphics", "CoreFoundation"] {
        println!("cargo:rustc-link-lib=framework={fw}");
    }
}
