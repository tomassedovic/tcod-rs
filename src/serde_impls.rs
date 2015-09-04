#![cfg(feature = "serde")]

use super::Color;

use serde::ser::{self, Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer};

impl Serialize for Color {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.visit_struct("Color", ColorMapVisitor {
            value: self,
            state: 0,
        })
    }
}

struct ColorMapVisitor<'a> {
    value: &'a Color,
    state: u8,
}

impl<'a> ser::MapVisitor for ColorMapVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("r", &self.value.r))))
            }
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("g", &self.value.g))))
            }
            2 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("b", &self.value.b))))
            }
            _ => {
                Ok(None)
            }
        }
    }
}

impl Deserialize for Color {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer {
        static FIELDS: &'static [&'static str] = &["r", "g", "b"];
        deserializer.visit_struct("Color", FIELDS, ColorVisitor)
    }
}

struct ColorVisitor;

impl de::Visitor for ColorVisitor {
    type Value = Color;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Color, V::Error>
        where V: de::MapVisitor
    {
        let mut r = None;
        let mut g = None;
        let mut b = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(ColorField::R) => { r = Some(try!(visitor.visit_value())); }
                Some(ColorField::G) => { g = Some(try!(visitor.visit_value())); }
                Some(ColorField::B) => { b = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let r = match r {
            Some(r) => r,
            None => try!(visitor.missing_field("r")),
        };

        let g = match g {
            Some(g) => g,
            None => try!(visitor.missing_field("g")),
        };

        let b = match b {
            Some(b) => b,
            None => try!(visitor.missing_field("b")),
        };

        try!(visitor.end());

        Ok(Color{r: r, g: g, b: b})
    }
}

enum ColorField {R, G, B}

impl Deserialize for ColorField {
    fn deserialize<D>(deserializer: &mut D) -> Result<ColorField, D::Error>
        where D: Deserializer
    {
        struct ColorFieldVisitor;

        impl de::Visitor for ColorFieldVisitor {
            type Value = ColorField;

            fn visit_str<E>(&mut self, value: &str) -> Result<ColorField, E>
                where E: de::Error
            {
                match value {
                    "r" => Ok(ColorField::R),
                    "g" => Ok(ColorField::G),
                    "b" => Ok(ColorField::B),
                    _ => Err(de::Error::syntax("expected r, g or b")),
                }
            }
        }

        deserializer.visit(ColorFieldVisitor)
    }
}

#[cfg(test)]
mod test {
    use ::Color;
    use serde_json;

    #[test]
    fn color_encode() {
        let encoded = serde_json::to_string(&Color{r: 1, g: 2, b: 3}).unwrap();
        assert_eq!("{\"r\":1,\"g\":2,\"b\":3}", encoded);
    }

    #[test]
    fn color_decode() {
        let decoded: Color = serde_json::from_str("{\"r\":1,\"g\":2,\"b\":3}").unwrap();
        assert_eq!(Color{r: 1, g: 2, b: 3}, decoded);
    }
}
