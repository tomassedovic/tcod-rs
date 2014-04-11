CARGO_OUT_DIR?=.

all:
	rustc --crate-type lib src/lib.rs --out-dir ${CARGO_OUT_DIR}
