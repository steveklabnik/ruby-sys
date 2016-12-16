use std::env;
use std::process::Command;

fn rbconfig(key: &str) -> Vec<u8> {
    let ruby = Command::new("ruby")
                   .arg("-e")
                   .arg(format!("print RbConfig::CONFIG['{}']", key))
                   .output()
                   .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    ruby.stdout
}

fn main() {
    let libdir = rbconfig("libdir");
    println!("cargo:rustc-link-search={}", String::from_utf8_lossy(&libdir));

    if env::var_os("USE_LIBRUBY_A").is_some() {
        println!("cargo:rustc-link-lib=static=ruby-static");
    } else {
        let soname = rbconfig("RUBY_SO_NAME");
        println!("cargo:rustc-link-lib=dylib={}", String::from_utf8_lossy(&soname));
    }
}
