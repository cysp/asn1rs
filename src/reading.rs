use identifier::{self, Identifier};
use std;


#[derive(Debug)]
enum InternalReadU8Error {
    UnexpectedEndOfData,
}

#[derive(Debug)]
enum InternalReadMultibyteError {
    UnexpectedEndOfData,
    MultibyteOverflow,
}


#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfData,
    InvalidTag,
    LengthOverflow,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Error::UnexpectedEndOfData => write!(f, "unexpected endofdata"),
            &Error::InvalidTag => write!(f, "invalid tag"),
            &Error::LengthOverflow => write!(f, "length overflow"),
            &Error::Io(ref e) => e.fmt(f)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::UnexpectedEndOfData => "unexpected endofdata",
            &Error::InvalidTag => "invalid tag",
            &Error::LengthOverflow => "length overflow",
            &Error::Io(ref e) => e.description()
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;


enum Asn1Length {
    Indefinite,
    Definite(usize),
}


pub struct Asn1Reader<'b> {
    buf: &'b [u8],
    pos: usize,
}

impl<'b> Asn1Reader<'b> {
    pub fn new(b: &'b [u8]) -> Self {
        Asn1Reader {
            buf: b,
            pos: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn read_u8(&mut self) -> std::result::Result<u8, InternalReadU8Error> {
        if self.pos >= self.buf.len() {
            return Err(InternalReadU8Error::UnexpectedEndOfData);
        }
        let v = self.buf[self.pos];
        self.pos += 1;
        Ok(v)
    }

    fn read_identifier(&mut self) -> Result<Identifier> {
        let b = match self.read_u8() {
            Ok(b) => b,
            Err(InternalReadU8Error::UnexpectedEndOfData) => return Err(Error::UnexpectedEndOfData),
        };
        let klass = identifier::Class::from_u8(b);
        let constructed = (b & 0x20) != 0;
        let tag = b & 0x1f;
        if tag != 0x1f {
            return Ok(Identifier::new(klass, constructed, tag as u64));
        }
        let tag = match self.read_multibyte_u64() {
            Ok(tag) => tag,
            Err(InternalReadMultibyteError::UnexpectedEndOfData) => return Err(Error::UnexpectedEndOfData),
            Err(InternalReadMultibyteError::MultibyteOverflow) => return Err(Error::InvalidTag),
        };
        Ok(Identifier::new(klass, constructed, tag))
    }

    fn read_multibyte_u64(&mut self) -> std::result::Result<u64, InternalReadMultibyteError> {
        let mut v = 0u64;
        loop {
            let b = match self.read_u8() {
                Ok(b) => b,
                Err(InternalReadU8Error::UnexpectedEndOfData) => return Err(InternalReadMultibyteError::UnexpectedEndOfData),
            };
            let has_continuation = b & 0x80 != 0;
            let b = b & 0x7f;
            v = match v.checked_mul(127) {
                Some(v) => v,
                None => return Err(InternalReadMultibyteError::MultibyteOverflow),
            };
            v += b as u64;
            if !has_continuation {
                break;
            }
        }
        Ok(v)
    }

    fn read_length(&mut self) -> Result<Asn1Length> {
        let b = match self.read_u8() {
            Ok(b) => b,
            Err(InternalReadU8Error::UnexpectedEndOfData) => return Err(Error::UnexpectedEndOfData),
        };
        if b == 0b10000000 {
            return Ok(Asn1Length::Indefinite);
        }

        if b < 128 {
            return Ok(Asn1Length::Definite(b as usize));
        }

        let len_len = b - 128;

        let mut len = 0usize;
        for _ in 0..len_len {
            let b = match self.read_u8() {
                Ok(b) => b,
                Err(InternalReadU8Error::UnexpectedEndOfData) => return Err(Error::UnexpectedEndOfData),
            };
            len = match len.checked_mul(256) {
                Some(len) => len + b as usize,
                None => return Err(Error::LengthOverflow),
            };
        }
        Ok(Asn1Length::Definite(len))
    }

    fn read_contents(&mut self, len: Asn1Length) -> Result<&'b [u8]> {
        match len {
            Asn1Length::Indefinite => self.read_indefinite_length_contents(),
            Asn1Length::Definite(len) => self.read_definite_length_contents(len),
        }
    }

    fn read_definite_length_contents(&mut self, len: usize) -> Result<&'b [u8]> {
        if (self.pos + len) > self.buf.len() {
            return Err(Error::UnexpectedEndOfData);
        }
        let contents = &self.buf[self.pos..self.pos+len];
        self.pos += len;
        Ok(contents)
    }

    fn read_indefinite_length_contents(&mut self) -> Result<&'b [u8]> {
        let pos = self.pos;

        #[derive(Copy,Clone,Debug,PartialEq,Eq)]
        enum State {
            Scanning,
            FoundOneNul,
        }

        let mut s = State::Scanning;
        let mut len = 0usize;
        loop {
            let b = match self.read_u8() {
                Ok(b) => b,
                Err(InternalReadU8Error::UnexpectedEndOfData) => return Err(Error::UnexpectedEndOfData),
            };

            len = match len.checked_add(1) {
                Some(len) => len,
                None => return Err(Error::LengthOverflow),
            };

            let b_is_nul = b == 0u8;
            match s {
                State::Scanning => {
                    if b_is_nul {
                        s = State::FoundOneNul;
                    }
                }
                State::FoundOneNul => {
                    if b_is_nul {
                        break;
                    }
                    s = State::Scanning;
                }
            }
        }
        let contents = &self.buf[pos..self.pos - 2];
        Ok(contents)
    }

    pub fn next(&mut self) -> Result<Option<(Identifier, &'b [u8])>> {
        if self.is_at_end() {
            return Ok(None);
        }
        let identifier = try!(self.read_identifier());
        let len = try!(self.read_length());
        let value = try!(self.read_contents(len));
        Ok(Some((identifier, value)))
    }

    pub fn iter(&'b mut self) -> Asn1ReaderRefIter<'b> {
        Asn1ReaderRefIter {
            r: self,
            have_vended_none_or_error: false,
        }
    }
}

impl<'b> IntoIterator for Asn1Reader<'b> {
    type Item = <Asn1ReaderIter<'b> as Iterator>::Item;
    type IntoIter = Asn1ReaderIter<'b>;

    fn into_iter(self) -> Self::IntoIter {
        Asn1ReaderIter {
            r: self,
            have_vended_none_or_error: false,
        }
    }
}

pub struct Asn1ReaderIter<'b> {
    r: Asn1Reader<'b>,
    have_vended_none_or_error: bool,
}

impl<'b> Iterator for Asn1ReaderIter<'b> {
    type Item = Result<(Identifier, &'b [u8])>;

    fn next(&mut self) -> Option<Result<(Identifier, &'b [u8])>> {
        if self.have_vended_none_or_error {
            return None;
        }

        let item = match self.r.next() {
            Ok(item) => item,
            Err(e) => {
                self.have_vended_none_or_error = true;
                return Some(Err(e));
            }
        };
        if let Some(i) = item {
            return Some(Ok(i));
        }
        self.have_vended_none_or_error = true;
        None
    }
}

pub struct Asn1ReaderRefIter<'b> {
    r: &'b mut Asn1Reader<'b>,
    have_vended_none_or_error: bool,
}

impl<'b> Iterator for Asn1ReaderRefIter<'b> {
    type Item = Result<(Identifier, &'b [u8])>;

    fn next(&mut self) -> Option<Result<(Identifier, &'b [u8])>> {
        if self.have_vended_none_or_error {
            return None;
        }

        let item = match self.r.next() {
            Ok(item) => item,
            Err(e) => {
                self.have_vended_none_or_error = true;
                return Some(Err(e));
            }
        };
        if let Some(i) = item {
            return Some(Ok(i));
        }
        self.have_vended_none_or_error = true;
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std;
    use identifier::{Identifier, Class, UniversalTag};

    #[test]
    fn smoke_boolean() {
        {
            let input = [0x01u8, 1, 0];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                r.next().unwrap()
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0u8]);
        }
        {
            let input = [0x01u8, 1, 0xff];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                r.next().unwrap()
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0xffu8]);
        }
    }

    #[test]
    fn test_long_form_tag() {
        {
            let input = [0x1fu8, 0x01, 1, 0];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                r.next().unwrap()
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0u8]);
        }

        {
            let input = [0x1fu8, 0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80];

            let mut r = Asn1Reader::new(&input);
            let e = r.next().unwrap_err();

            assert_eq!(std::error::Error::description(&e), "invalid tag");
        }
    }

    #[test]
    fn test_long_form_length() {
        {
            let input = [0x01u8, 0x81, 1, 0];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                r.next().unwrap()
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0u8]);
        }

        {
            let input = [0x01u8, 0x89, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                r.next().unwrap()
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [0u8]);
        }

        {
            let input = [0x01u8, 0x89, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

            let mut r = Asn1Reader::new(&input);
            let e = r.next().unwrap_err();

            assert_eq!(std::error::Error::description(&e), "length overflow");
        }
    }

    #[test]
    fn test_indefinite_length() {
        {
            let input = [0x01u8, 0x80, 1, 0, 0];

            let (t, v) = {
                let mut r = Asn1Reader::new(&input);
                let tv = r.next().unwrap();
                assert!(r.is_at_end());
                tv
            }.unwrap();
            assert_eq!(t, Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()));
            assert_eq!(v, [1u8]);
        }
    }

    #[test]
    fn test_iter() {
        {
            let input = [0x30u8, 0x06, 0x01, 0x01, 0x00, 0x01, 0x01, 0xff];

            let mut r = Asn1Reader::new(&input);
            let (seq_i, seq_d) = r.next().unwrap().unwrap();

            assert_eq!(seq_i, Identifier::new(Class::Universal, true, UniversalTag::Sequence.into()));

            let mut r = Asn1Reader::new(&seq_d);
            let seq_vals: Vec<_> = r.iter().map(Result::unwrap).collect();

            assert_eq!(seq_vals, &[
                (Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()), &[0u8][..]),
                (Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()), &[0xffu8][..]),
            ]);
        }
    }
}
