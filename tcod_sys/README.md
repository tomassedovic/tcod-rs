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
