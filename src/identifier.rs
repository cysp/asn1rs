// #![allow(dead_code)]

use std;


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Class {
    Universal, // 0b00
    Application, // 0b01
    ContextSpecific, // 0b10
    Private, // 0b11
}

impl Class {
    pub fn from_u8(v: u8) -> Self {
        match (v >> 6) & 0b11 {
            0b00 => Class::Universal,
            0b01 => Class::Application,
            0b10 => Class::ContextSpecific,
            0b11 => Class::Private,
            _ => unreachable!(),
        }
    }

    fn into_u8(self) -> u8 {
        let v = match self {
            Class::Universal => 0b00,
            Class::Application => 0b01,
            Class::ContextSpecific => 0b10,
            Class::Private => 0b11,
        };
        v << 6
    }
}

impl From<Class> for u8 {
    fn from(v: Class) -> u8 {
        v.into_u8()
    }
}


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum UniversalTag {
    EndOfContent, // 0b00000
    Boolean, // 0b00001
    Integer, // 0b00010
    BitString, // 0b00011
    OctetString, // 0b00100
    Null, // 0b00101
    ObjectIdentifier, // 0b00110
    ObjectDescriptor, // 0b00111
    External, // 0b01000
    Real, // 0b01001
    Enumerated, // 0b01010
    EmbeddedPov, // 0b01011
    Utf8String, // 0b01100
    RelativeOid, // 0b01101
    // 0b01110 reserved
    // 0b01111 reserved
    Sequence, // 0b10000
    Set, // 0b10001
    NumericString, // 0b10010
    PrintableString, // 0b10011
    T61String, // 0b10100
    VideotexString, // 0b10101
    Ia5String, // 0b10110
    UtcTime, // 0b10111
    GeneralizedTime, // 0b11000
    GraphicString, // 0b11001
    VisibleString, // 0b11010
    GeneralString, // 0b11011
    UniversalString, // 0b11100
    CharacterString, // 0b11101
    BmpString, // 0b11110
    // UseLongForm, // 0b11111
}

impl UniversalTag {
    pub fn into_u8(self) -> u8 {
        match self {
            UniversalTag::EndOfContent => 0b00000,
            UniversalTag::Boolean => 0b00001,
            UniversalTag::Integer => 0b00010,
            UniversalTag::BitString => 0b00011,
            UniversalTag::OctetString => 0b00100,
            UniversalTag::Null => 0b00101,
            UniversalTag::ObjectIdentifier => 0b00110,
            UniversalTag::ObjectDescriptor => 0b00111,
            UniversalTag::External => 0b01000,
            UniversalTag::Real => 0b01001,
            UniversalTag::Enumerated => 0b01010,
            UniversalTag::EmbeddedPov => 0b01011,
            UniversalTag::Utf8String => 0b01100,
            UniversalTag::RelativeOid => 0b01101,
            // 0b01110 reserved
            // 0b01111 reserved
            UniversalTag::Sequence => 0b10000,
            UniversalTag::Set => 0b10001,
            UniversalTag::NumericString => 0b10010,
            UniversalTag::PrintableString => 0b10011,
            UniversalTag::T61String => 0b10100,
            UniversalTag::VideotexString => 0b10101,
            UniversalTag::Ia5String => 0b10110,
            UniversalTag::UtcTime => 0b10111,
            UniversalTag::GeneralizedTime => 0b11000,
            UniversalTag::GraphicString => 0b11001,
            UniversalTag::VisibleString => 0b11010,
            UniversalTag::GeneralString => 0b11011,
            UniversalTag::UniversalString => 0b11100,
            UniversalTag::CharacterString => 0b11101,
            UniversalTag::BmpString => 0b11110,
        }
    }

    pub fn from_u64(v: u64) -> Option<Self> {
        Some(match v & 0b11111 {
            0b00000 => UniversalTag::EndOfContent,
            0b00001 => UniversalTag::Boolean,
            0b00010 => UniversalTag::Integer,
            0b00011 => UniversalTag::BitString,
            0b00100 => UniversalTag::OctetString,
            0b00101 => UniversalTag::Null,
            0b00110 => UniversalTag::ObjectIdentifier,
            0b00111 => UniversalTag::ObjectDescriptor,
            0b01000 => UniversalTag::External,
            0b01001 => UniversalTag::Real,
            0b01010 => UniversalTag::Enumerated,
            0b01011 => UniversalTag::EmbeddedPov,
            0b01100 => UniversalTag::Utf8String,
            0b01101 => UniversalTag::RelativeOid,
            0b01110 => return None,
            0b01111 => return None,
            0b10000 => UniversalTag::Sequence,
            0b10001 => UniversalTag::Set,
            0b10010 => UniversalTag::NumericString,
            0b10011 => UniversalTag::PrintableString,
            0b10100 => UniversalTag::T61String,
            0b10101 => UniversalTag::VideotexString,
            0b10110 => UniversalTag::Ia5String,
            0b10111 => UniversalTag::UtcTime,
            0b11000 => UniversalTag::GeneralizedTime,
            0b11001 => UniversalTag::GraphicString,
            0b11010 => UniversalTag::VisibleString,
            0b11011 => UniversalTag::GeneralString,
            0b11100 => UniversalTag::UniversalString,
            0b11101 => UniversalTag::CharacterString,
            0b11110 => UniversalTag::BmpString,
            0b11111 => return None, // use-long-form
            _ => unreachable!(),
        })
    }
}

impl From<UniversalTag> for u8 {
    fn from(tag: UniversalTag) -> u8 {
        match tag {
            UniversalTag::EndOfContent => 0b00000,
            UniversalTag::Boolean => 0b00001,
            UniversalTag::Integer => 0b00010,
            UniversalTag::BitString => 0b00011,
            UniversalTag::OctetString => 0b00100,
            UniversalTag::Null => 0b00101,
            UniversalTag::ObjectIdentifier => 0b00110,
            UniversalTag::ObjectDescriptor => 0b00111,
            UniversalTag::External => 0b01000,
            UniversalTag::Real => 0b01001,
            UniversalTag::Enumerated => 0b01010,
            UniversalTag::EmbeddedPov => 0b01011,
            UniversalTag::Utf8String => 0b01100,
            UniversalTag::RelativeOid => 0b01101,
            // 0b01110 reserved
            // 0b01111 reserved
            UniversalTag::Sequence => 0b10000,
            UniversalTag::Set => 0b10001,
            UniversalTag::NumericString => 0b10010,
            UniversalTag::PrintableString => 0b10011,
            UniversalTag::T61String => 0b10100,
            UniversalTag::VideotexString => 0b10101,
            UniversalTag::Ia5String => 0b10110,
            UniversalTag::UtcTime => 0b10111,
            UniversalTag::GeneralizedTime => 0b11000,
            UniversalTag::GraphicString => 0b11001,
            UniversalTag::VisibleString => 0b11010,
            UniversalTag::GeneralString => 0b11011,
            UniversalTag::UniversalString => 0b11100,
            UniversalTag::CharacterString => 0b11101,
            UniversalTag::BmpString => 0b11110,
        }
    }
}
impl From<UniversalTag> for u32 {
    fn from(tag: UniversalTag) -> u32 {
        u8::from(tag) as Self
    }
}
impl From<UniversalTag> for u64 {
    fn from(tag: UniversalTag) -> u64 {
        u8::from(tag) as Self
    }
}


#[derive(Copy,Clone,PartialEq,Eq)]
pub struct Identifier (Class, bool, u64);

impl Identifier {
    pub fn new(klass: Class, constructed: bool, tag: u64) -> Self {
        Identifier(klass, constructed, tag)
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        let class = Class::from_u8(v & 0xc0);
        let constructed = (v & 0x20) != 0;

        let tag = match v & 0x1f {
            0x1f => return None,
            tag => tag,
        };

        Some(Identifier(class, constructed, tag as u64))
    }

    #[inline]
    pub fn class(&self) -> Class {
        self.0
    }

    #[inline]
    pub fn is_constructed(&self) -> bool {
        self.1
    }

    #[inline]
    pub fn tag(&self) -> u64 {
        self.2
    }

    #[inline]
    pub fn bytes_len(&self) -> usize {
        let tag = self.tag();
        if tag < 31 {
            return 1;
        }
        let mut tag = tag;
        let mut len = 1;
        loop {
            if tag == 0 {
                break;
            }
            len += 1;
            tag = tag / 127;
        }
        len
    }

    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut v = Vec::with_capacity(bytes_len);
        self.write_to(&mut v).unwrap();
        v
    }

    pub fn write_to<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let tag = self.tag();
        if tag < 31 {
            let class = self.0.into_u8();
            let constructed = if self.1 { 0x20 } else { 0 };
            let b: u8 = class | constructed | (tag as u8);
            return w.write(&[b]);
        }
        unimplemented!()
    }
}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let constructed = if self.1 { "constructed" } else { "primitive" };
        match self.0 {
            Class::Universal => {
                match UniversalTag::from_u64(self.2) {
                    Some(tag) => write!(f, "Identifier(Universal, {}, {:?})", constructed, tag),
                    None => write!(f, "Identifier(Universal, {}, {})", constructed, self.2),
                }
            }
            _ => write!(f, "Identifier({:?}, {}, {})", self.0, constructed, self.2),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_roundtrip() {
        for v in 0..254 {
            match v & 0b11111 {
                0b01110 => continue,
                0b01111 => continue,
                0b11111 => continue,
                _ => (),
            }
            let i = Identifier::from_u8(v).unwrap();
            assert_eq!(i.to_bytes(), &[v]);
        }
    }
}
