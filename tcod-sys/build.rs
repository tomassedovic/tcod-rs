use std::io::{fs, Command};
use std::io::process::InheritFd;
use std::os;


fn main() {
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    let target = os::getenv("TARGET").unwrap();
    let (makefile, dylib_ext) = if target.contains("linux") {
        ("makefile-linux", "so")
    } else if target.contains("darwin") {
        ("makefile-osx", "dylib")
    } else {
        panic!("Don't know how to handle target {} yet.", target);
    };

    let libtcod_dir = src.join("libtcod");

    let mut make = Command::new("make");
    make.arg("-f")
        .arg(Path::new("makefiles").join(makefile))
        .arg("clean")
        .arg("all")
        .cwd(&libtcod_dir);
    make.stdout(InheritFd(1))
        .stdout(InheritFd(2))
        .status()
        .unwrap()
        .success();


    let lib_name = Path::new("libtcod").with_extension(dylib_ext);
    match fs::copy(&libtcod_dir.join(&lib_name), &dst.join(&lib_name)) {
        Ok(_) => {
            if target.contains("linux") {
                // TODO(tsedovic): code using this seems to expect
                // `libtcod.so.1` instead of `libtcod.so` so just copy that,
                // too. No idea why?
                fs::copy(&libtcod_dir.join(&lib_name),
                         &dst.join(&lib_name.with_extension("so.1"))).unwrap();
            }
            println!("cargo:rustc-flags=-l tcod:dylib -L {}", dst.display())
        }
        Err(_) => panic!("Copy failed!"),
    }
}
