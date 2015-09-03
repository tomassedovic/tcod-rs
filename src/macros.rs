macro_rules! iterable_enum {
    ($Enum: ident => [$First: ident] $($Variant: ident),* [$Last: ident]) => {
        impl $Enum {
            pub fn variants() -> ::std::iter::Cloned<::std::slice::Iter<'static, $Enum>> {
                static VARIANTS: &'static [$Enum] = &[$Enum::$First, $($Enum::$Variant,)+ $Enum::$Last];
                VARIANTS.iter().cloned()
            }

            /// Returns the next variant in an enum. The last variant is infinitely repeated.
            pub fn bounded_next(self) -> $Enum {
                match self {
                    $Enum::$Last => $Enum::$Last,
                    _ => unsafe { ::std::mem::transmute(self as i32 + 1) },
                }
            }

            /// Returns the previous variant in an enum. The first variant is infinitely repeated.
            pub fn bounded_prev(self) -> $Enum {
                match self {
                    $Enum::$First => $Enum::$First,
                    _ => unsafe { ::std::mem::transmute(self as i32 - 1) },
                }
            }

            /// Returns the next variant in an enum. If the last element is reached it wraps around
            /// to the first element.
            pub fn cycled_next(self) -> $Enum {
                match self {
                    $Enum::$Last => $Enum::$First,
                    _ => unsafe { ::std::mem::transmute(self as i32 + 1) },
                }
            }

            /// Returns the previous variant in an enum. If the first element is reached it wraps around
            /// to the last element.
            pub fn cycled_prev(self) -> $Enum {
                match self {
                    $Enum::$First => $Enum::$Last,
                    _ => unsafe { ::std::mem::transmute(self as i32 - 1) },
                }
            }
        }
    };
}

macro_rules! test_iterable_enum {
    ($Test: ident, $Enum: ident => $($Variant: ident),+) => {
        #[test]
        fn $Test() {
            use super::*;

            let variants = [$($Enum::$Variant),+];

            let last = variants.len() - 1;
            assert_eq!(variants[0].bounded_next(), variants[1]);
            assert_eq!(variants[0].bounded_prev(), variants[0]);
            assert_eq!(variants[0].cycled_next(), variants[1]);
            assert_eq!(variants[0].cycled_prev(), variants[last]);

            for i in 1..last {
                assert_eq!(variants[i].bounded_next(), variants[i + 1]);
                assert_eq!(variants[i].bounded_prev(), variants[i - 1]);
                assert_eq!(variants[i].cycled_next(), variants[i + 1]);
                assert_eq!(variants[i].cycled_prev(), variants[i - 1]);
            }

            assert_eq!(variants[last].bounded_next(), variants[last]);
            assert_eq!(variants[last].bounded_prev(), variants[last - 1]);
            assert_eq!(variants[last].cycled_next(), variants[0]);
            assert_eq!(variants[last].cycled_prev(), variants[last - 1]);

            let mut i = 0;
            for variant in $Enum::variants() {
                assert_eq!(variant, variants[i]);
                i += 1;
            }
            assert_eq!(i, variants.len());
        }
    }
}
