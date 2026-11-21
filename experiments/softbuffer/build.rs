use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo:rerun-if-changed=src/appkit_host.m");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let obj = out.join("appkit_host.o");
    let lib = out.join("libpong_appkit_host.a");
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86_64") => "x86_64",
        _ => panic!("unsupported macOS architecture"),
    };
    let deployment = env::var("MACOSX_DEPLOYMENT_TARGET").unwrap_or_else(|_| "11.0".into());
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
    if env::var_os("CARGO_FEATURE_APPKIT_NATIVE_QA").is_some() {
        clang.arg("-DPONG_APPKIT_QA");
    }
    assert!(clang
        .arg(format!("-mmacosx-version-min={deployment}"))
        .args(["-c", "src/appkit_host.m", "-o"])
        .arg(&obj)
        .status()
        .expect("run Apple clang")
        .success());
    assert!(Command::new("xcrun")
        .args(["ar", "crs"])
        .arg(lib)
        .arg(obj)
        .status()
        .expect("run Apple ar")
        .success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=pong_appkit_host");
    println!("cargo:rustc-link-lib=framework=AppKit");
}
