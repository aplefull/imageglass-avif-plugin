#!/bin/sh

set -eu
cd "$(dirname "$0")"

cargo build --release

case "$(uname -s)" in
    Linux)  os=linux; lib=libAvifCodec.so ;;
    Darwin) os=mac;   lib=libAvifCodec.dylib ;;
    *)      os=win;   lib=AvifCodec.dll ;;
esac
case "$(uname -m)" in
    arm64 | aarch64) arch=arm64 ;;
    *)               arch=x64 ;;
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
rm -rf dist/staging

echo "packed dist/$zip"
