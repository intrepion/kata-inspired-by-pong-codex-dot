# Match Rust's default dynamically linked MSVC runtime in SDL debug builds.
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL" CACHE STRING "MSVC runtime" FORCE)
