build:
	cargo build

test:
	cargo test  --features "rustc-serialize_impls"

package: clean
	cargo package --manifest-path tcod_sys/Cargo.toml
	cargo package

clean:
	git clean -x -f -d

.PHONY: build test package clean
