use std::path::{Path, PathBuf};

#[cfg(feature = "generate-bindings")]
const XPLANE_SDK_PATH_KEY: &str = "XPLANE_SDK_PATH";
#[cfg(feature = "generate-bindings")]
const XPLANE_SDK_VERSIONS_KEY: &str = "XPLANE_SDK_VERSIONS";
#[cfg(feature = "generate-bindings")]
// Default XPLM version definitions
const DEFAULT_XPLM_VERSION_DEFINITIONS: [&str; 5] = [
    "-DXPLM200",
    "-DXPLM210",
    "-DXPLM300",
    "-DXPLM301",
    "-DXPLM303",
];

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
    println!("cargo:rerun-if-env-changed={XPLANE_SDK_PATH_KEY}");
    println!("cargo:rerun-if-env-changed={XPLANE_SDK_VERSIONS_KEY}");

    let include_path = sdk_path.join("CHeaders");
    let out_path = std::env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("env variable `OUT_DIR` should be defined");

    // Collect headers
    let mut headers = Vec::new();
    headers.extend_from_slice(
        &collect_headers(&include_path.join("Widgets"))
            .expect("failed to gather header files in `Widgets`"),
    );
    headers.extend_from_slice(
        &collect_headers(&include_path.join("XPLM"))
            .expect("failed to gather header files in `XPLM`"),
    );

    // Setup builder
    let mut builder = bindgen::Builder::default()
        // Pass arguments to clang
        .clang_args([
            // Target C++ because some sneaked into the latest SDK version
            "-x",
            "c++",
            // Parse all comments as documentation
            "-fparse-all-comments",
            // Define platform - irrelevant for Rust
            "-DLIN=1",
            // Minimal XPLM version is always defined
            "-DXPLM200",
        ]);

    // Add SDK headers
    for header in headers {
        builder = builder.header(header);
    }

    // Use custom SDK versions if they are specified, otherwise use the default
    match custom_sdk_versions() {
        Some(custom_versions) => builder = builder.clang_args(custom_versions),
        None => builder = builder.clang_args(DEFAULT_XPLM_VERSION_DEFINITIONS),
    };

    // Generate bindings
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    // Write bindings to disk
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings to disk");
}

/// Collects all header files in the directory
#[cfg(feature = "generate-bindings")]
fn collect_headers(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut headers = Vec::new();
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|ext| ext == "h").unwrap_or_default() {
            headers.push(path.to_str().expect("path is not valid UTF-8").to_string());
        }
    }
    Ok(headers)
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

/// Parse the custom version environment variable
///
/// Returns A Vec of version definitions to pass to Clang. Or None if the variable was not defined.
///
/// # Panics
///
/// Panics if the environment variable didn't contain valid UTF-8
#[cfg(feature = "generate-bindings")]
fn custom_sdk_versions() -> Option<Vec<String>> {
    // Semicolon-separated list of SDK versions. Example: "XPLM400;XPLM401"
    let custom_versions = match std::env::var(XPLANE_SDK_VERSIONS_KEY) {
        Ok(versions) => versions,
        Err(std::env::VarError::NotPresent) => return None,
        Err(std::env::VarError::NotUnicode(_)) => panic!(
            "`{}` environment variable has to be valid UTF-8",
            XPLANE_SDK_VERSIONS_KEY
        ),
    };

    Some(
        custom_versions
            .split(';')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(|version| format!("-D{}", version))
            .collect(),
    )
}
