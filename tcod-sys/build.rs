use std::io::{fs, Command};
use std::io::process::InheritFd;
use std::os;


fn main() {
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    let target = os::getenv("TARGET").unwrap();
    let makefile = if target.contains("linux") {
        "makefile-linux"
    } else if target.contains("darwin") {
        "makefile-osx"
    } else if target.contains("windows") {
        "makefile-mingw"
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
    assert!(make.stdout(InheritFd(1))
            .stdout(InheritFd(2))
            .status()
            .unwrap()
            .success());

    // TODO(shadower): there's a bunch of name special-casing I came across
    // testing this in my linux and windows setups. I have no idea why these are
    // required, so right now we just produce the compiled binary in all the
    // names necessary. There's probably a better way to deal with this.
    if target.contains("linux") {
        fs::copy(&libtcod_dir.join("libtcod.so"), &dst.join("libtcod.so")).unwrap();
        // NOTE(shadower): this is what the executables seem to expect:
        fs::copy(&libtcod_dir.join("libtcod.so"), &dst.join("libtcod.so.1")).unwrap();
    } else if target.contains("darwin") {
        fs::copy(&libtcod_dir.join("libtcod.dylib"), &dst.join("libtcod.dylib")).unwrap();
    } else if target.contains("windows") {
        // NOTE: this seems to be needed by MinGW's gcc:
        fs::copy(&libtcod_dir.join("libtcod-mingw.dll"), &dst.join("libtcod.dll")).unwrap();
        // NOTE: this seems to be what the executable using tcod seems to expect:
        fs::copy(&libtcod_dir.join("libtcod-mingw.dll"), &dst.join("libtcod-mingw.dll")).unwrap();
        // NOTE: this is shipped with libtcod and Windows executables expect it:
        fs::copy(&libtcod_dir.join("SDL.dll"), &dst.join("SDL.dll")).unwrap();
    }

    // TODO: can we (optionally?) produce a static copy? It'd probably be more
    // work, but would be easier for the end users.
    println!("cargo:rustc-flags=-l tcod:dylib -L {}", dst.display())
}
