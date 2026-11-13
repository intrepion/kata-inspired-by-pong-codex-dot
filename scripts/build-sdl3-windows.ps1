param(
    [ValidateSet('build', 'test')][string]$Action = 'build',
    [string]$TargetDir = 'target-sdl3-msvc-runtime'
)
$ErrorActionPreference = 'Stop'
if ($env:CMAKE_TOOLCHAIN_FILE) {
    throw 'CMAKE_TOOLCHAIN_FILE is already set; refusing to override an existing toolchain.'
}
$root = Split-Path $PSScriptRoot -Parent
$previousToolchain = $env:CMAKE_TOOLCHAIN_FILE
try {
    $env:CMAKE_TOOLCHAIN_FILE = Join-Path $PSScriptRoot 'sdl3-msvc-runtime.cmake'
    & cargo +stable-x86_64-pc-windows-msvc $Action --locked --manifest-path (Join-Path $root 'experiments/sdl3/Cargo.toml') --target-dir (Join-Path $root $TargetDir) -j 2
    if ($LASTEXITCODE -ne 0) { throw "SDL3 cargo $Action failed with exit code $LASTEXITCODE" }
} finally {
    $env:CMAKE_TOOLCHAIN_FILE = $previousToolchain
}
