<#
.SYNOPSIS
    Builds the AVIF codec plugin and packs it as avif-codec_<version>_<arch>.igplugin.zip.

.DESCRIPTION
    Produces a single "Plugin_AvifCodec" folder holding the native library and its manifest.
    ImageGlass accepts that either through Settings > Plugins > Add or as a manual copy into
    the _plugins directory.

    libavif and dav1d are built from source and linked statically, which needs CMake, meson,
    ninja and nasm. On Windows dav1d must be compiled by MSVC, so this script loads the
    Visual Studio x64 build environment first.
#>

$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$pluginFolder = 'Plugin_AvifCodec'
$runtime = 'win-x64'
$libName = 'AvifCodec.dll'


# 1. tools the dav1d and libavif builds call by name
foreach ($tool in 'cargo', 'cmake', 'meson', 'ninja', 'nasm') {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        throw "$tool is not on PATH (see README > Build)."
    }
}


# 2. the MSVC x64 environment, so meson picks cl.exe
$installer = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer"
$vswhere = Join-Path $installer 'vswhere.exe'
if (-not (Test-Path $vswhere)) { throw 'Visual Studio with the C++ workload is required.' }

$vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vsPath) { throw 'Visual Studio C++ x64 build tools are not installed.' }

# vcvarsall calls vswhere by bare name, so it must be reachable from inside cmd too
$env:PATH = "$installer;$env:PATH"
$vcvars = Join-Path $vsPath 'VC\Auxiliary\Build\vcvars64.bat'
cmd /c "`"$vcvars`" >nul 2>nul && set" | ForEach-Object {
    if ($_ -match '^([^=]+)=(.*)$') { Set-Item "env:$($Matches[1])" $Matches[2] }
}
$env:CC = 'cl'
$env:CXX = 'cl'


# 3. versions must agree: ImageGlass shows the manifest's, the library reports Cargo's
$manifestPath = Join-Path $root 'igplugin.json'
$version = (Get-Content $manifestPath -Raw | ConvertFrom-Json).version
$cargoVersion = (Select-String -Path (Join-Path $root 'Cargo.toml') -Pattern '^version\s*=\s*"(.+)"').Matches[0].Groups[1].Value
if ($version -ne $cargoVersion) {
    throw "igplugin.json says $version but Cargo.toml says $cargoVersion; keep them in step."
}


# 4. build
Write-Host "`n=== building $runtime ===" -ForegroundColor Cyan
cargo build --release --manifest-path (Join-Path $root 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }


# 5. pack archive
$staged = Join-Path $root "dist/staging/$runtime/$pluginFolder"
if (Test-Path $staged) { Remove-Item $staged -Recurse -Force }
New-Item -ItemType Directory -Force -Path $staged | Out-Null
Copy-Item (Join-Path $root "target/release/$libName"), $manifestPath -Destination $staged

$zip = Join-Path $root "dist/avif-codec_${version}_$runtime.igplugin.zip"
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path $staged -DestinationPath $zip

if (Test-Path $staged) { Remove-Item $staged -Recurse -Force }

$size = [math]::Round((Get-Item $zip).Length / 1KB)
Write-Host "packed $zip ($size KB)" -ForegroundColor Green

