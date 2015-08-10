use identifier::{Identifier, Class, UniversalTag};
use std;
// use core::array::FixedSizeArray;


#[derive(Debug)]
pub enum Error {
    // InsufficientSpace,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Error::InsufficientSpace => write!(f, "Insufficient")
            &Error::Io(ref e) => e.fmt(f)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
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


pub struct Writer<'w, W: std::io::Write + 'w> {
    w: &'w mut W,
    // pos: usize,
}

impl<'w, W: std::io::Write> Writer<'w, W> {
    pub fn new(w: &'w mut W) -> Self {
        Writer {
            w: w,
            // pos: 0,
        }
    }

    pub fn write_tag_and_data(&mut self, tag: Identifier, data: &[u8]) -> Result<usize> {
        let len = data.len();

        let len: Vec<u8> = if len < 128 {
            let mut len_buf = Vec::with_capacity(1);
            len_buf.push(len as u8);
            len_buf
        } else {
            unimplemented!();
        };

        let tag_bytes = tag.to_bytes();
        try!(self.w.write(&tag_bytes));
        try!(self.w.write(&len));
        try!(self.w.write(data));
        Ok(tag_bytes.len() + len.len() + data.len())
    }

    pub fn write_indefinite_tag<F>(&mut self, tag: Identifier, f: F) -> Result<usize>
        where
            F: FnOnce(&mut IndeterminateLengthContentWriter<W>) -> Result<()>,
    {
        let tag_bytes = tag.to_bytes();
        let tag_size = try!(self.w.write(&tag_bytes));
        let indefinite_len_size = try!(self.w.write(&[0x80u8]));

        let x = |s: &mut Self| -> Result<usize> {
            let mut ilcw = IndeterminateLengthContentWriter {
                w: s.w,
                bytes_written: 0,
            };
            try!(f(&mut ilcw));
            Ok(ilcw.bytes_written)
        };
        let content_len = match x(self) {
            Ok(len) => len,
            Err(e) => return Err(e.into()),
        };

        // let content_len = {
        //     let mut ilcw = IndeterminateLengthContentWriter {
        //         w: self.w,
        //         bytes_written: 0,
        //     };
        //     try!(f(&mut ilcw));
        //     ilcw.bytes_written
        // };

        let eoc_size = try!(self.w.write(&[0u8, 0]));
        Ok(tag_size + indefinite_len_size + content_len + eoc_size)
    }

    pub fn write_boolean(&mut self, v: bool) -> Result<usize> {
        let v: u8 = if v { 0xff } else { 0 };
        self.write_tag_and_data(Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()), &[v])
    }
}

pub struct IndeterminateLengthContentWriter<'w, W: std::io::Write + 'w> {
    w: &'w mut W,
    bytes_written: usize,
}

impl<'w, W: std::io::Write + 'w> IndeterminateLengthContentWriter<'w, W> {
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        match self.w.write(buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

// impl<'w, W: std::io::Write + 'w> std::io::Write for IndeterminateLengthContentWriter<'w, W> {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.w.write(buf)
//     }
//     fn flush(&mut self) -> std::io::Result<()> {
//         self.w.flush()
//     }
// }



#[cfg(test)]
mod test {
    use identifier::{Identifier, Class, UniversalTag};
    use super::*;

    #[test]
    fn smoke_boolean() {
        {
            let mut output = Vec::with_capacity(4);
            {
                let mut w = Writer::new(&mut output);
                w.write_boolean(false).unwrap();
            }
            assert_eq!(output, [0x01u8, 1, 0]);
        }
        {
            let mut output = Vec::with_capacity(4);
            {
                let mut w = Writer::new(&mut output);
                w.write_boolean(true).unwrap();
            }
            assert_eq!(output, [0x01u8, 1, 0xff]);
        }
    }

    #[test]
    fn test_write_indefinite_length() {
        {
            let mut output = Vec::with_capacity(4);
            {
                let mut w = Writer::new(&mut output);
                w.write_indefinite_tag(Identifier::new(Class::Universal, false, UniversalTag::Boolean.into()), |w| {
                    w.write(&[0xffu8])
                }).unwrap();
            }
            assert_eq!(output, [0x01u8, 0x80, 0xff, 0, 0]);
        }
    }
}
