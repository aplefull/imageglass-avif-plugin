#!/bin/sh

set -eu
cd "$(dirname "$0")"

deploy=
case "${1:-}" in
    --deploy | -deploy) deploy=1 ;;
    "") ;;
    *) echo "usage: sh pack.sh [--deploy]" >&2; exit 2 ;;
esac

cargo build --release

case "$(uname -s)" in
    Linux)  os=linux; lib=libAvifCodec.so;    data=${XDG_DATA_HOME:-$HOME/.local/share} ;;
    Darwin) os=mac;   lib=libAvifCodec.dylib; data="$HOME/Library/Application Support" ;;
    *)      os=win;   lib=AvifCodec.dll;      data=$(cygpath -u "$LOCALAPPDATA") ;;
esac
case "$(rustc -vV | sed -n 's/^host: //p')" in
    aarch64-*) arch=arm64 ;;
    x86_64-*)  arch=x64 ;;
    *)         echo "unsupported architecture" >&2; exit 1 ;;
esac
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
zip="avif-codec_${version}_$os-$arch.igplugin.zip"

rm -rf dist/staging
mkdir -p dist/staging/Plugin_AvifCodec
cp "target/release/$lib" dist/staging/Plugin_AvifCodec/
sed "s/\"AvifCodec\.dll\"/\"$lib\"/" igplugin.json > dist/staging/Plugin_AvifCodec/igplugin.json

for py in python3 python; do "$py" -c '' 2>/dev/null && break; done
rm -f "dist/$zip"
(cd dist/staging && "$py" -m zipfile -c "../$zip" Plugin_AvifCodec/)
echo "packed dist/$zip"

if [ -n "$deploy" ]; then
    target="$data/ImageGlass/_plugins/Plugin_AvifCodec"

    if [ -e "$target" ]; then
        rm -rf "$target.old"
        mv "$target" "$target.old" 2>/dev/null || { echo "cannot replace $target, close ImageGlass first" >&2; exit 1; }
        rm -rf "$target.old"
    fi

    mkdir -p "$(dirname "$target")"
    cp -R dist/staging/Plugin_AvifCodec "$target"
    echo "deployed to $target"
fi

rm -rf dist/staging
