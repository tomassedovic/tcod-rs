#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::missing_safety_doc, clippy::redundant_static_lifetimes)]

include!(concat!("./", env!("BINDINGS_TARGET"), "_bindings.rs"));
