macro_rules! native_enum_convert {
    ($From: ident, $To: ident) => {
        impl From<$From> for $To {
            fn from(other: $From) -> $To {
                unsafe { ::std::mem::transmute::<$From, $To>(other) }
            }
        }
    };
}
