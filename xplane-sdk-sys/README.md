# Rust bindings for the X-Plane SDK

[![crates.io](https://img.shields.io/crates/v/xplane-sdk-sys)](https://crates.io/crates/xplane-sdk-sys)
[![crates.io](https://img.shields.io/readthedocs/xplane-sdk-sys)](https://docs.rs/xplane-sdk-sys/)

This library uses `bindgen` to generate the bindings.

## Features

* Extracts the documentation out of original the C SDK
* Includes pre-built bindings for version `XPLM303`
* Feature gate allows to switch between pre-built and compile time generated bindings
* Allows to specify additional SDK versions when using compile time generated bindings

## Generate your own bindings

To generate the bindings yourself, rather than using the pre-built once,
activate the `generate-bindings` feature.

```toml
xplane-sdk-sys = { version = "*", features = ["generate-bindings"] }
```

This will invoke `bindgen` at compile time in a build script to generate the bindings.
You'll have to specify the location of the SDK in your filesystem by setting the `XPLANE_SDK`
environment variable. `xplane-sdk-sys` expects to find the `CHeaders` directory in the specified folder.

### Custom SDK versions

To future prove this library, it allows you to specify additional SDK version to generator bindings for.

To do this you'll have to set the `XPLANE_SDK_VERSIONS` environment variable accordingly.
Setting it to `XPLM400;XPLM401` will, for example, generate bindings for SDK version `4.0.0` and `4.0.1`.

