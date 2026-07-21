use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.ancestors().nth(3).unwrap();
    
    // Perform symlinking for the examples output directory
    let examples_dir = target_dir.join("examples");
    let _ = std::fs::create_dir_all(&examples_dir);
    let so_path = examples_dir.join("libk4rust_usage.so");
    if so_path.exists() || so_path.symlink_metadata().is_ok() {
        let _ = std::fs::remove_file(&so_path);
    }
    
    let test_so_path = examples_dir.join("libk4rust_test_ffi.so");
    if test_so_path.exists() || test_so_path.symlink_metadata().is_ok() {
        let _ = std::fs::remove_file(&test_so_path);
    }
    
    #[cfg(unix)]
    {
        let _ = std::os::unix::fs::symlink("libk4rust_usage.dylib", &so_path);
        let _ = std::os::unix::fs::symlink("libk4rust_test_ffi.dylib", &test_so_path);
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&manifest_dir).join("lib");
    let _ = std::fs::create_dir_all(&lib_dir);
    let lib_path = lib_dir.join("libkdb_client.a");

    if !lib_path.exists() {
        // Detect OS and Architecture to determine the download path
        let target = env::var("TARGET").unwrap(); // e.g. "aarch64-apple-darwin" or "x86_64-unknown-linux-gnu"
        let (folder, filename) = if target.contains("apple-darwin") {
            // macOS uses universal binary in m64/
            ("m64", "c.o")
        } else if target.contains("linux") {
            if target.contains("aarch64") {
                ("l64arm", "c.o")
            } else {
                ("l64", "c.o")
            }
        } else {
            panic!("Unsupported compile target: {}", target);
        };

        let url = format!("https://github.com/KxSystems/kdb/raw/master/{}/{}", folder, filename);
        let c_o_path = lib_dir.join("c.o");

        println!("cargo:warning=Downloading c.o from {}...", url);
        let status = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&c_o_path)
            .arg(&url)
            .status()
            .expect("Failed to execute curl");

        if !status.success() {
            panic!("Failed to download c.o from {}", url);
        }

        // Package c.o into libkdb_client.a
        println!("cargo:warning=Packaging c.o into libkdb_client.a...");
        let archive_status = if target.contains("apple-darwin") {
            // Use libtool on macOS
            Command::new("libtool")
                .arg("-static")
                .arg("-o")
                .arg(&lib_path)
                .arg(&c_o_path)
                .status()
                .expect("Failed to execute libtool")
        } else {
            // Use ar on Linux
            Command::new("ar")
                .arg("rcs")
                .arg(&lib_path)
                .arg(&c_o_path)
                .status()
                .expect("Failed to execute ar")
        };

        if !archive_status.success() {
            panic!("Failed to package c.o into libkdb_client.a");
        }

        // Clean up temporary c.o
        let _ = std::fs::remove_file(c_o_path);
    }

    println!("cargo:rustc-link-search=native={}/lib", manifest_dir);
}
