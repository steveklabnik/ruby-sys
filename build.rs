use std::env;
use std::ffi::OsStr;
use std::process::Command;

fn rbconfig(key: &str) -> Vec<u8> {
    let ruby = match env::var_os("RUBY") {
        Some(val) => val.to_os_string(),
        None => OsStr::new("ruby").to_os_string(),
    };
    let config = Command::new(ruby)
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    config.stdout
}

fn main() {
    let libdir = rbconfig("libdir");
    let soname = rbconfig("RUBY_SO_NAME");

    println!("cargo:rustc-link-search={}",
             String::from_utf8_lossy(&libdir));
    println!("cargo:rustc-link-lib=dylib={}",
             String::from_utf8_lossy(&soname));
}
