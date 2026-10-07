use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Source {
    url: &'static str,
    rev: &'static str,
}

const LIBAVIF: Source = Source {
    url: "https://github.com/AOMediaCodec/libavif.git",
    rev: "v1.4.2",
};

const DAV1D: Source = Source {
    url: "https://code.videolan.org/videolan/dav1d.git",
    rev: "1.5.4",
};

const LIBYUV: Source = Source {
    url: "https://chromium.googlesource.com/libyuv/libyuv",
    rev: "98697ab033ff9fb56dbe42c6bb214e1d285640c4",
};

const SKIP_NATIVE: &str = "AVIF_PLUGIN_SKIP_NATIVE";
const BINDINGS: &str = "src/sys/bindings.rs";
const MANIFEST: &str = "igplugin.json";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={BINDINGS}");
    println!("cargo:rerun-if-changed={MANIFEST}");
    println!("cargo:rerun-if-env-changed={SKIP_NATIVE}");

    let manifest = fs::read_to_string(MANIFEST).expect("cannot read igplugin.json");
    let cargo_version = env::var("CARGO_PKG_VERSION").unwrap();

    if manifest_version(&manifest) != Some(cargo_version.as_str()) {
        panic!("{MANIFEST} and Cargo.toml disagree on the version");
    }

    let bindings_header = format!("// bindgen output for libavif {}", LIBAVIF.rev);
    let regen = env::var_os("CARGO_FEATURE_REGEN_BINDINGS").is_some();

    if !regen && env::var_os(SKIP_NATIVE).is_some() {
        return;
    }

    if !regen {
        let committed = fs::read_to_string(BINDINGS).unwrap_or_default();
        if committed.lines().next() != Some(bindings_header.as_str()) {
            panic!(
                "{BINDINGS} is not from libavif {}, run cargo build --features regen-bindings",
                LIBAVIF.rev
            );
        }
    }

    if env::var("HOST") != env::var("TARGET") {
        panic!("cross-compiling is not supported, build on the target platform");
    }

    let msvc = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    let x86 = matches!(
        env::var("CARGO_CFG_TARGET_ARCH").as_deref(),
        Ok("x86_64" | "x86")
    );

    let compiler = msvc.then_some("clang-cl");

    let mut path: Vec<PathBuf> =
        env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect();

    if msvc && !on_path(&path, "ninja") {
        path.extend(visual_studio_ninja());
    }

    let mut tools = vec!["git", "cmake", "ninja", "meson"];
    tools.extend(x86.then_some("nasm"));
    tools.extend(compiler);
    for tool in tools {
        if !on_path(&path, tool) {
            panic!("{tool} is not on PATH (see README > Build)");
        }
    }

    let path = env::join_paths(path).unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libavif = out_dir.join("libavif");
    fetch(&LIBAVIF, &libavif);
    fetch(&DAV1D, &libavif.join("ext/dav1d"));
    fetch(&LIBYUV, &libavif.join("ext/libyuv"));

    #[cfg(feature = "regen-bindings")]
    generate_bindings(&libavif.join("include/avif/avif.h"), &bindings_header);

    let build = out_dir.join("build");
    let mut configure = Command::new("cmake");
    configure
        .env("PATH", &path)
        .arg("-S")
        .arg(&libavif)
        .arg("-B")
        .arg(&build)
        .args(["-G", "Ninja"])
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DAVIF_CODEC_DAV1D=LOCAL")
        .arg("-DAVIF_LIBYUV=LOCAL")
        .arg("-DAVIF_LIBSHARPYUV=OFF")
        .arg("-DAVIF_LIBXML2=OFF")
        .arg("-DAVIF_JPEG=OFF")
        .arg("-DAVIF_ZLIBPNG=OFF")
        .arg("-DAVIF_BUILD_APPS=OFF")
        .arg("-DAVIF_BUILD_TESTS=OFF")
        .arg("-DAVIF_ENABLE_WERROR=OFF");

    if msvc {
        configure.arg("-DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDLL");
    }

    let mut build_cmd = Command::new("cmake");
    build_cmd
        .env("PATH", &path)
        .arg("--build")
        .arg(&build)
        .args(["--target", "avif", "yuv", "dav1d"]);

    if let Some(compiler) = compiler {
        configure
            .arg(format!("-DCMAKE_C_COMPILER={compiler}"))
            .arg(format!("-DCMAKE_CXX_COMPILER={compiler}"));

        for cmd in [&mut configure, &mut build_cmd] {
            cmd.env("CC", compiler).env("CXX", compiler);
        }
    }

    run(&mut configure);
    run(&mut build_cmd);

    let libs: [(&str, &str); 3] = if msvc {
        [
            ("avif_internal.lib", "static:+verbatim=avif_internal.lib"),
            ("yuv.lib", "static:+verbatim=yuv.lib"),
            ("libdav1d.a", "static:+verbatim=libdav1d.a"),
        ]
    } else {
        [
            ("libavif_internal.a", "static=avif_internal"),
            ("libyuv.a", "static=yuv"),
            ("libdav1d.a", "static=dav1d"),
        ]
    };

    for (file, link) in libs {
        let path = find_file(&build, file).unwrap_or_else(|| panic!("{file} was not built"));
        println!(
            "cargo:rustc-link-search=native={}",
            path.parent().unwrap().display()
        );
        println!("cargo:rustc-link-lib={link}");
    }

    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("macos") => println!("cargo:rustc-link-lib=dylib=c++"),
        Ok("linux") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => {}
    }
}

/// Writes the libavif bindings, they are committed so the editor and plain checks don't need
/// the native build
#[cfg(feature = "regen-bindings")]
fn generate_bindings(header: &Path, first_line: &str) {
    bindgen::Builder::default()
        .header(header.to_string_lossy())
        .disable_header_comment()
        .raw_line(first_line)
        .allowlist_function("avif.*")
        .allowlist_type("avif.*")
        .allowlist_var("AVIF_.*")
        .prepend_enum_name(false)
        .generate()
        .expect("bindgen could not process avif.h")
        .write_to_file(BINDINGS)
        .expect("cannot write the bindings");
}

/// Shallow-fetches `source.rev` into `dir`, unless it already holds that revision
fn fetch(source: &Source, dir: &Path) {
    let marker = dir.join(".fetched-rev");
    if fs::read_to_string(&marker).is_ok_and(|rev| rev == source.rev) {
        return;
    }

    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }

    fs::create_dir_all(dir).unwrap();

    let git = |args: &[&str]| {
        run(Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "core.longpaths=true"])
            .args(args))
    };
    git(&["init", "--quiet"]);
    git(&["fetch", "--quiet", "--depth", "1", source.url, source.rev]);
    git(&[
        "-c",
        "advice.detachedHead=false",
        "checkout",
        "--quiet",
        "FETCH_HEAD",
    ]);

    fs::write(marker, source.rev).unwrap();
}

fn run(command: &mut Command) {
    let status = command
        .status()
        .unwrap_or_else(|e| panic!("cannot start {command:?}: {e}"));
    if !status.success() {
        panic!("{command:?} failed with {status}");
    }
}

fn on_path(path: &[PathBuf], tool: &str) -> bool {
    let file = format!("{tool}{}", env::consts::EXE_SUFFIX);
    path.iter().any(|dir| dir.join(&file).is_file())
}

/// The ninja bundled with the Visual Studio, found through vswhere
fn visual_studio_ninja() -> Option<PathBuf> {
    let program_files = env::var_os("ProgramFiles(x86)")?;
    let vswhere = Path::new(&program_files).join("Microsoft Visual Studio/Installer/vswhere.exe");
    let output = Command::new(vswhere)
        .args(["-latest", "-products", "*", "-property", "installationPath"])
        .output()
        .ok()?;
    let install = String::from_utf8(output.stdout).ok()?;
    let dir = Path::new(install.trim()).join("Common7/IDE/CommonExtensions/Microsoft/CMake/Ninja");
    dir.join("ninja.exe").is_file().then_some(dir)
}

/// The top-level "version" string of igplugin.json
fn manifest_version(json: &str) -> Option<&str> {
    let rest = &json[json.find("\"version\"")? + "\"version\"".len()..];
    let rest = &rest[rest.find('"')? + 1..];
    Some(&rest[..rest.find('"')?])
}

fn find_file(dir: &Path, name: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_file(&path, name) {
                return Some(found);
            }
        } else if entry.file_name() == name {
            return Some(path);
        }
    }
    None
}
