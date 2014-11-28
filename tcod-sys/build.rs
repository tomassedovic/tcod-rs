use std::io::fs;
use std::os;


fn main() {
    panic!("cannot compile");
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    // TODO: this just copies the precompiled linux lib to the right place. We
    // want to build it and provide support for other plattforms:
    let linux_src = src.join("precompiled/libtcod.so");
    let linux_dst = dst.join("libtcod.so");

    match fs::copy(&linux_src, &linux_dst) {
        Ok(_) => {
            println!("cargo:rustc-flags=-l tcod:dylib -L {}", dst.display())
        }
        Err(_) => panic!("Copy failed!"),
    }
}
