pub mod identifier;
pub mod reading;
pub mod writing;

pub use identifier::Identifier;

pub use reading::Asn1Reader;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indefinite_length_roundtrip() {
        use writing::der::Writer;
        use identifier::{Class, UniversalTag};

        {
            let mut buf = Vec::with_capacity(4);
            {
                let mut w = Writer::new(&mut buf);
                w.write_indefinite_tag(Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()), |w| {
                    w.write(&[0xffu8])
                }).unwrap();
            }
            assert_eq!(buf, [0x01u8, 0x80, 0xff, 0, 0]);

            let (t, v) = {
                let mut r = Asn1Reader::new(&buf);
                let tv = r.next().unwrap();
                assert!(r.is_at_end());
                tv
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0xffu8]);
        }
    }
}
