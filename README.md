# AVIF Codec for ImageGlass

Opens still and animated AVIF files in [ImageGlass](https://imageglass.org) 10, using
[libavif](https://github.com/AOMediaCodec/libavif), [dav1d](https://code.videolan.org/videolan/dav1d)
and [libyuv](https://chromium.googlesource.com/libyuv/libyuv).

## Install

1. Download `avif-codec_<version>_<platform>.igplugin.zip` from
   [Releases](https://github.com/aplefull/imageglass-avif-plugin/releases).
2. ImageGlass → **Settings → Plugins → Add**, pick the zip, then **Trust and enable**.

## How it works

- ImageGlass asks the plugin for one frame at a time. Each frame is decoded with dav1d, converted
  to RGB by libavif (using libyuv where it has a fast path), and rotated or mirrored as the file
  specifies.
- 10- and 12-bit stills are passed on with 16 bits per channel. Animations and thumbnails use
  8 bits.
- HDR files keep their encoding and are tone-mapped by ImageGlass.
- The 4 most recently used files stay open, so repeated requests skip reading and parsing.
- During playback, the next frame is decoded in the background while the current one is on screen.
- An animation whose decoded frames fit in 256 MB keeps them after the
  first loop, so later loops don't decode at all. Larger animations are decoded fresh every loop
  and use roughly constant memory.

## Build

Needs Rust, git, CMake, meson, ninja and nasm (x86 only) on PATH. On Windows, also clang-cl and
Visual Studio with the C++ workload. Visual Studio's bundled ninja is used if none is on PATH.

```sh
cargo build --release   # build.rs fetches and builds libavif, dav1d and libyuv
sh pack.sh              # packs dist/avif-codec_<version>_<platform>.igplugin.zip
```

To update a library, change its pin in `build.rs`. After a libavif update, also run
`cargo build --features regen-bindings` to regenerate `src/sys/bindings.rs`.
