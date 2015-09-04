#![cfg(feature = "rustc-serialize")]

use super::Color;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

impl Encodable for Color {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Color", 3, |s| {
            try!{ s.emit_struct_field("r", 0, |s| self.r.encode(s)) };
            try!{ s.emit_struct_field("g", 1, |s| self.g.encode(s)) };
            try!{ s.emit_struct_field("b", 2, |s| self.b.encode(s)) };
            Ok(())
        })
    }
}

#[cfg(feature = "rustc-serialize")]
impl Decodable for Color {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Color", 3, |d| {
            let r = try!(d.read_struct_field("r", 0, |d| d.read_u8()));
            let g = try!(d.read_struct_field("g", 1, |d| d.read_u8()));
            let b = try!(d.read_struct_field("b", 2, |d| d.read_u8()));
            Ok(Color{r: r, g: g, b: b})
        })
    }
}



#[cfg(test)]
mod test {
    use ::Color;
    use ::rustc_serialize::json;

    #[test]
    fn color_encode() {
        let encoded = json::encode(&Color{r: 1, g: 2, b: 3}).unwrap();
        assert_eq!("{\"r\":1,\"g\":2,\"b\":3}", encoded);
    }

    #[test]
    fn color_decode() {
        let decoded: Color = json::decode("{\"r\":1,\"g\":2,\"b\":3}").unwrap();
        assert_eq!(Color{r: 1, g: 2, b: 3}, decoded);
    }
}
