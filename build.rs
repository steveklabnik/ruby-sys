use std::env;
use std::ffi::OsStr;
use std::process::Command;

fn rbconfig(key: &str) -> String {
    let ruby = match env::var_os("RUBY") {
        Some(val) => val.to_os_string(),
        None => OsStr::new("ruby").to_os_string(),
    };
    let config = Command::new(ruby)
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    String::from_utf8(config.stdout).expect("RbConfig value not UTF-8!")
}

fn main() {
    if cfg!(feature="test") {
        println!("cargo:rustc-link-search={}", rbconfig("libdir"));
        println!("cargo:rustc-link-lib=dylib={}", rbconfig("RUBY_SO_NAME"));
    }
}
