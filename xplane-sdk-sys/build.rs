use std::path::{Path, PathBuf};

#[cfg(feature = "generate-bindings")]
const XPLANE_SDK_PATH_KEY: &str = "XPLANE_SDK_PATH";
#[cfg(feature = "generate-bindings")]
const XPLANE_SDK_VERSIONS_KEY: &str = "XPLANE_SDK_VERSIONS";

fn main() {
    // Pre-built SDK path
    #[cfg(not(feature = "generate-bindings"))]
    let sdk_path = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("failed to locate packaged SDK, env variable `CARGO_MANIFEST_DIR` should be set")
        .join("SDK");

    // Retrieve SKD path from environment variable
    #[cfg(feature = "generate-bindings")]
    let sdk_path = std::env::var(XPLANE_SDK_PATH_KEY)
        .map(PathBuf::from)
        .expect("failed to locate the SDK, env variable `XPLANE_SDK_PATH` should point to the SDK");

    // Only generate bindings if the feature is set
    #[cfg(feature = "generate-bindings")]
    generate_bindings(&sdk_path);

    // Add libraries for linking
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    link_libraries(&sdk_path);
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(sdk_path: &Path) {
    // Re-run the build script if any SDK change is detected
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed={XPLANE_SDK_PATH_KEY}");
    println!("cargo:rerun-if-env-changed={XPLANE_SDK_VERSIONS_KEY}");

    let include_path = sdk_path.join("CHeaders");
    let out_path = std::env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("env variable `OUT_DIR` should be defined");

    // Get custom versions environment variable
    let custom_versions = std::env::var(XPLANE_SDK_VERSIONS_KEY).unwrap_or_default();

    // Create iterator over custom versions
    // Semicolon-separated list of SDK versions. Example: "XPLM400;XPLM401"
    let custom_versions_iter = custom_versions
        .split(|c| c == ';')
        .map(|version| version.trim())
        .filter(|version| !version.is_empty())
        .map(|version| format!("-D{}", version));

    // Setup builder
    let builder = bindgen::Builder::default()
        // Pass arguments to clang
        .clang_args([
            // Parse all comments as documentation
            "-fparse-all-comments",
            // Specify XPLM versions
            "-DXPLM200",
            "-DXPLM210",
            "-DXPLM300",
            "-DXPLM301",
            "-DXPLM303",
            // Define platform - irrelevant for Rust
            "-DLIN=1",
            // Include directories
            &format!("-I{}", include_path.join("XPLM").display()),
            &format!("-I{}", include_path.join("Widgets").display()),
        ])
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // Add custom version definitions
    let builder = builder.clang_args(custom_versions_iter);

    // Generate bindings
    let bindings = builder.generate().expect("failed to generate bindings");

    // Write bindings to disk
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings to disk");
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn link_libraries(sdk_path: &Path) {
    let library_path = sdk_path.join("Libraries");

    // Can only ever be Windows or MacOS
    if cfg!(windows) {
        println!(
            "cargo:rustc-link-search={}",
            library_path.join("Win").display()
        );
        println!("cargo:rustc-link-lib=XPLM_64");
        println!("cargo:rustc-link-lib=XPWidgets_64");
    } else {
        // MacOS
        println!(
            "cargo:rustc-link-search=framework={}",
            library_path.join("Mac").display()
        );
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    }
}
