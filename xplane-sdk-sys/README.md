# Rust bindings for the X-Plane SDK

[![crates.io](https://img.shields.io/crates/v/xplane-sdk-sys)](https://crates.io/crates/xplane-sdk-sys)
[![Documentation](https://img.shields.io/docsrs/xplane-sdk-sys)](https://docs.rs/xplane-sdk-sys/)

Low level Rust bindings for the [X-Plane SDK](https://developer.x-plane.com/sdk/).

## Features

* Includes pre-built bindings for version `XPLM303`
* Feature gate allows to switch between pre-built and compile-time generated bindings
* Allows for specification of additional SDK versions for compile-time generated bindings

## Generate your own bindings

To generate your own bindings instead of using the pre-built ones,
activate the `generate-bindings` feature by adding the following to your `Cargo.toml`:

```toml
xplane-sdk-sys = { version = "*", features = ["generate-bindings"] }
```

This will invoke `bindgen` at compile-time in a build script to generate the bindings.
You must specify the location of the SDK in your filesystem by setting the `XPLANE_SDK_PATH`
environment variable.

### Requirements

`xplane-sdk-sys` inherits the `bindgen` requirements. They are
documented [here](https://rust-lang.github.io/rust-bindgen/requirements.html).

### Custom SDK versions

To ensure the library is future-proof, it offers the option to define extra SDK versions for binding generation.

To achieve this, you need to adjust the `XPLANE_SDK_VERSIONS` environment variable accordingly.
It expects a semicolon-separated list of SDK versions. Defining it as `XPLM400;XPLM401`, as an example, will
result in the generation of bindings for the SDK version `4.0.0` and `4.0.1` respectively.

