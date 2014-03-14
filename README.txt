# libtcod bindings for Rust

The raw bindings were generated using rust-bindgen[1]:

    bindgen -l tcod include/libtcod.h -o src/ffi.rs

Licensed under WTFPL v2.


[1]: https://github.com/crabtw/rust-bindgen
