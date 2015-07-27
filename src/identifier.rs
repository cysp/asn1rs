// #![allow(dead_code)]

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Class {
    Universal, // 0b00
    Application, // 0b01
    ContextSpecific, // 0b10
    Private, // 0b11
}

impl Class {
    fn from_u8(v: u8) -> Self {
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


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Tag {
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
    UseLongForm, // 0b11111
}

impl Tag {
    fn into_u8(self) -> u8 {
        match self {
            Tag::EndOfContent => 0b00000,
            Tag::Boolean => 0b00001,
            Tag::Integer => 0b00010,
            Tag::BitString => 0b00011,
            Tag::OctetString => 0b00100,
            Tag::Null => 0b00101,
            Tag::ObjectIdentifier => 0b00110,
            Tag::ObjectDescriptor => 0b00111,
            Tag::External => 0b01000,
            Tag::Real => 0b01001,
            Tag::Enumerated => 0b01010,
            Tag::EmbeddedPov => 0b01011,
            Tag::Utf8String => 0b01100,
            Tag::RelativeOid => 0b01101,
            // 0b01110 reserved
            // 0b01111 reserved
            Tag::Sequence => 0b10000,
            Tag::Set => 0b10001,
            Tag::NumericString => 0b10010,
            Tag::PrintableString => 0b10011,
            Tag::T61String => 0b10100,
            Tag::VideotexString => 0b10101,
            Tag::Ia5String => 0b10110,
            Tag::UtcTime => 0b10111,
            Tag::GeneralizedTime => 0b11000,
            Tag::GraphicString => 0b11001,
            Tag::VisibleString => 0b11010,
            Tag::GeneralString => 0b11011,
            Tag::UniversalString => 0b11100,
            Tag::CharacterString => 0b11101,
            Tag::BmpString => 0b11110,
            Tag::UseLongForm => 0b11111,
        }
    }

    fn from_u8(v: u8) -> Option<Self> {
        Some(match v & 0b11111 {
            0b00000 => Tag::EndOfContent,
            0b00001 => Tag::Boolean,
            0b00010 => Tag::Integer,
            0b00011 => Tag::BitString,
            0b00100 => Tag::OctetString,
            0b00101 => Tag::Null,
            0b00110 => Tag::ObjectIdentifier,
            0b00111 => Tag::ObjectDescriptor,
            0b01000 => Tag::External,
            0b01001 => Tag::Real,
            0b01010 => Tag::Enumerated,
            0b01011 => Tag::EmbeddedPov,
            0b01100 => Tag::Utf8String,
            0b01101 => Tag::RelativeOid,
            0b01110 => return None,
            0b01111 => return None,
            0b10000 => Tag::Sequence,
            0b10001 => Tag::Set,
            0b10010 => Tag::NumericString,
            0b10011 => Tag::PrintableString,
            0b10100 => Tag::T61String,
            0b10101 => Tag::VideotexString,
            0b10110 => Tag::Ia5String,
            0b10111 => Tag::UtcTime,
            0b11000 => Tag::GeneralizedTime,
            0b11001 => Tag::GraphicString,
            0b11010 => Tag::VisibleString,
            0b11011 => Tag::GeneralString,
            0b11100 => Tag::UniversalString,
            0b11101 => Tag::CharacterString,
            0b11110 => Tag::BmpString,
            0b11111 => Tag::UseLongForm,
            _ => unreachable!(),
        })
    }
}


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Identifier (Class, bool, Tag);

impl From<u8> for Identifier {
    fn from(v: u8) -> Self {
        let class = Class::from_u8(v & 0xc0);
        let constructed = (v & 0x20) != 0;
        let tag = Tag::from_u8(v & 0x1f);

        Identifier(class, constructed, tag.unwrap())
    }
}

impl From<Identifier> for u8 {
    fn from(v: Identifier) -> Self {
        let class = v.0.into_u8();
        let constructed = if v.1 { 0x20 } else { 0 };
        let tag = v.2.into_u8();
        class | constructed | tag
    }
}

impl Identifier {
    pub fn from_u8(v: u8) -> Option<Self> {
        let class = Class::from_u8(v & 0xc0);
        let constructed = (v & 0x20) != 0;
        let tag = Tag::from_u8(v & 0x1f);

        let tag = match tag {
            Some(tag) => tag,
            None => return None,
        };

        Some(Identifier(class, constructed, tag))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_roundtrip() {
        for v in (0..255) {
            match v & 0b11111 {
                0b01110 => continue,
                0b01111 => continue,
                _ => (),
            }
            let i: Identifier = v.into();
            assert_eq!(v, i.into());
        }
    }
}
