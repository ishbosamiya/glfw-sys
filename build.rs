extern crate cmake;
use cmake::Config;

fn main() {
    let mut cfg = Config::new("glfw");

    cfg.define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    // requires multithreaded runtime for static library (for
    // windows), otherwise there are linker errors with unresolved
    // external symbols
    //
    // TODO: need to figure out why `cfg.static_crt(true)` does not
    // work but setting `CMAKE_MSVC_RUNTIME_LIBRARY` to
    // `MultiThreaded` or `MultiThreadedDebug`
    // does. `static_crt(true)` causes `cc` (`cmake` internally passes
    // along `static_crt` to `cc`) to emit `-MT` flag, which is the
    // same as `CMAKE_MSVC_RUNTIME_LIBRARY` based on its
    // documentation.
    //
    // reference: <https://github.com/glfw/glfw/issues/1981>
    #[cfg(target_os = "windows")]
    cfg.define(
        "CMAKE_MSVC_RUNTIME_LIBRARY",
        "MultiThreaded$<$<CONFIG:Debug>:Debug>",
    );

    let dst = if cfg!(feature = "wayland") {
        cfg.define("GLFW_USE_WAYLAND", "ON").build()
    } else {
        cfg.define("GLFW_USE_WAYLAND", "OFF").build()
    };

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=glfw3");
}
