# libtcod bindings for Rust


TODO:
* link to libtcod
* document the tcod version
* link to wtfpl
* link to libtcod docs
* minimal example
* document how to build executables with it
* declare we're using semver (plus link)
* document what's implemented


The raw bindings were generated using rust-bindgen[1]:

    bindgen -builtins -l tcod include/libtcod.h -o src/ffi.rs

Licensed under WTFPL v2.


[1]: https://github.com/crabtw/rust-bindgen
