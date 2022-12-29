fn main() {
    // Only generate bindings if the feature is set
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    use std::path::PathBuf;

    let sdk_path =
        PathBuf::from(std::env::var("XPLANE_SDK").expect("failed to locate the X-Plane SDK"));
    let include_path = sdk_path.join("CHeaders");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Get custom versions environment variable
    let custom_versions = std::env::var("XPLANE_SDK_VERSIONS").unwrap_or_default();

    // Create iterator over custom versions
    // Seperated by : or ;. Example: "XPLM400;XPLM401"
    let custom_versions_iter = custom_versions
        .split(|c| c == ':' || c == ';')
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
