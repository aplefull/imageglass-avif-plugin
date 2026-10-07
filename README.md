# AVIF Codec for ImageGlass

Opens still and animated AVIF files in [ImageGlass](https://imageglass.org) 10, using
[libavif](https://github.com/AOMediaCodec/libavif), [dav1d](https://code.videolan.org/videolan/dav1d)
and [libyuv](https://chromium.googlesource.com/libyuv/libyuv).

## Install

1. Download `avif-codec_<version>_<platform>.igplugin.zip` from
   [Releases](https://github.com/aplefull/imageglass-avif-plugin/releases).
2. ImageGlass → **Settings → Plugins → Add**, pick the zip, then **Trust and enable**.

## Build

Needs Rust, git, CMake, meson, ninja and nasm (x86 only) on PATH. On Windows, also clang-cl and
Visual Studio with the C++ workload. Visual Studio's bundled ninja is used if none is on PATH.

```sh
cargo build --release   # build.rs fetches and builds libavif, dav1d and libyuv
sh pack.sh              # packs dist/avif-codec_<version>_<platform>.igplugin.zip
```

To update a library, change its pin in `build.rs`. After a libavif update, also run
`cargo build --features regen-bindings` to regenerate `src/sys/bindings.rs`.
