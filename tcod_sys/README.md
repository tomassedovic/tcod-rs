Bindings
------------

Bindings are generated for the given target during build time when the feature
`generate_bindings` is enabled:
```sh
cargo build --features generate_bindings
```

This will create or update the file `$TARGET_bindings.rs` in the root of
tcod_sys. This file is included by `lib.rs`.

If the bindings are up to date, doing a build without the `generate_bindings`
feature will simply use the existing bindings, or if none exist for the given
target, will panic during the build with a message about the bindings missing:
```
thread 'main' panicked at 'No bindings found for $TARGET', build.rs:148:9
```

The current raw bindings were generated using
[rust-bindgen](https://github.com/rust-lang/rust-bindgen) (v0.54) and are located at
`src/ffi.rs`. The safe (hopefully?) wrapper was built on top of them at
`src/lib.rs`.

You can regenerate the raw bindings by running:

```sh
bindgen --builtins --default-enum-style rust --with-derive-default --raw-line "#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]" libtcod/include/libtcod.h -o lib.rs
```
