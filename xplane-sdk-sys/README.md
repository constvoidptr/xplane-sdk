# Rust bindings for the X-Plane SDK

[![crates.io](https://img.shields.io/crates/v/xplane-sdk-sys)](https://crates.io/crates/xplane-sdk-sys)
[![Documentation](https://img.shields.io/docsrs/xplane-sdk-sys)](https://docs.rs/xplane-sdk-sys/)

Low level Rust bindings for the [X-Plane SDK](https://developer.x-plane.com/sdk/).

## Features

* Includes pre-built bindings for version `XPLM303` (X-Plane 11.50 and newer)
* Option to generate bindings at compile-time, using your own SDK version
* Allows for precise version specification when using compile-time generated bindings

## Generate your own bindings

To generate your own bindings instead of using the pre-built ones, activate the `generate-bindings` feature by adding
the following to your `Cargo.toml`:

```toml
xplane-sdk-sys = { version = "*", features = ["generate-bindings"] }
```

This will invoke `bindgen` at compile-time in a build script to generate the bindings for you. You must specify the
location of the SDK in your filesystem by setting the `XPLANE_SDK_PATH` environment variable.

### Requirements

`xplane-sdk-sys` inherits the `bindgen` requirements. They are
documented [here](https://rust-lang.github.io/rust-bindgen/requirements.html).

### SDK version selection

When generating bindings yourself you can specify what XPLM versions you would like to use. This not only enables you to
target earlier X-Plane versions but also ensures that the library is compatible with future versions.

To achieve this, you need to set the `XPLANE_SDK_VERSIONS` environment variable. It expects a semicolon-separated list
of SDK versions. Defining at as `XPLM303;XPLM400`, for example, will result in the generation of bindings for SDK
versions `3.0.3` and `4.0.0` respectively.

Please note that SDK versions do not automatically imply older versions. To ensure that the latest version with all
features is targeted, it is necessary to specify every version.

If you leave the `XPLANE_SDK_VERSIONS` environment variable unspecified, the default set of versions used for generating
the pre-built binding will be applied.