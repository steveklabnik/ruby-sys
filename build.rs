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

fn use_libdir() {
    println!("cargo:rustc-link-search={}", rbconfig("libdir"));
}

fn transform_lib_args(rbconfig_key: &str, replacement: &str) -> String {
    rbconfig(rbconfig_key).replace("-l", replacement)
}

fn use_static() {
    // LIBRUBY_A returns full library name in the form: `libruby-static.a`
    // or `libruby.static-2.4.2.a`
    // Cargo wants them as: `ruby-static` or `ruby.2.4.2-static`
    let libruby_a = rbconfig("LIBRUBY_A");
    let static_lib_name = libruby_a.trim_left_matches("lib").trim_right_matches(".a");

    println!("cargo:rustc-link-lib=static={}", static_lib_name);

    // Ruby gives back the libs in the form: `-lpthread -lgmp`
    // Cargo wants them as: `-l pthread -l gmp`
    println!("cargo:rustc-flags={}", transform_lib_args("LIBS", "-l "));

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
}

fn use_dylib() {
    println!("cargo:rustc-link-lib=dylib={}", rbconfig("RUBY_SO_NAME"));
}

fn main() {
    if rbconfig("target_os") != "mingw32" && env::var_os("RUBY_STATIC").is_some() {
        use_static()
    } else {
        match rbconfig("ENABLE_SHARED").as_str() {
            "no" => use_static(),
            "yes" => use_dylib(),
            _ => {
                let msg = "Error! Couldn't find a valid value for \
                RbConfig::CONFIG['ENABLE_SHARED']. \
                This may mean that your ruby's build config is corrupted. \
                Possible solution: build a new Ruby with the `--enable-shared` configure opt.";
                panic!(msg)
            }
        }
    }

    use_libdir();
}
