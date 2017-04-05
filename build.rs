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

fn use_static() {
    let ruby_libs = rbconfig("LIBS");
    let libs = String::from_utf8_lossy(&ruby_libs);

    // Ruby gives back the libs in the form: `-lpthread -lgmp`
    // Cargo wants them as: `-l pthread -l gmp`
    let transformed_lib_args = libs.replace("-l", "-l ");

    println!("cargo:rustc-link-lib=static=ruby-static");
    println!("cargo:rustc-flags={}", transformed_lib_args);
}

fn use_dylib(lib: Vec<u8>) {
    println!("cargo:rustc-link-lib=dylib={}",
             String::from_utf8_lossy(&lib));
}

fn main() {
    let libdir = rbconfig("libdir");

    let libruby_static = rbconfig("LIBRUBY_A");
    let libruby_so = rbconfig("RUBY_SO_NAME");

    match (libruby_static.is_empty(), libruby_so.is_empty()) {
        (false, true) => use_static(),
        (true, false) => use_dylib(libruby_so),
        (false, false) => {
            if env::var_os("RUBY_STATIC").is_some() {
                use_static()
            } else {
                use_dylib(libruby_so)
            }
        },
        _ => {
            let msg = "Error! Could not find LIBRUBY_A or RUBY_SO_NAME. \
            This means that no static, or dynamic libruby was found. \
            Possible solution: build a new Ruby with the `--enable-shared` configure opt.";
            panic!(msg)
        }
    }

    println!("cargo:rustc-link-search={}",
             String::from_utf8_lossy(&libdir));
}
