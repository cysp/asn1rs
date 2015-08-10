use std;

use std::io::Read;


#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfData,
    ComponentOverflow,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::UnexpectedEndOfData => "unexpected endofdata",
            &Error::ComponentOverflow => "oid component overflow",
        }
    }
}


pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub struct ObjectIdentifier {
    components: Vec<u64>,
}

impl ObjectIdentifier {
    pub fn from_bytes(b: &[u8]) -> Result<ObjectIdentifier> {
        if b.len() < 1 {
            return Err(Error::UnexpectedEndOfData);
        }

        let mut components = Vec::<u64>::with_capacity(b.len() + 1);
        let c = std::io::Cursor::new(b);
        let mut cb = c.bytes();
        let b = match cb.next() {
            Some(Ok(b)) => b,
            Some(Err(_)) => return Err(Error::UnexpectedEndOfData),
            None => return Err(Error::UnexpectedEndOfData),
        };

        components.push((b / 40) as u64);
        components.push((b % 40) as u64);

        let mut expect_continuation = false;
        let mut accumulator = 0u64;
        for b in cb { match b {
            Ok(b) => {
                accumulator |= (b & 0x7f) as u64;
                if b & 0x80 != 0 {
                    if accumulator > u64::max_value() / 128 {
                         return Err(Error::ComponentOverflow);
                    }
                    accumulator *= 128;
                    expect_continuation = true;
                } else {
                    components.push(accumulator);
                    accumulator = 0;
                    expect_continuation = false;
                }
            },
            Err(_) => return Err(Error::UnexpectedEndOfData),
        }}

        if expect_continuation {
            return Err(Error::UnexpectedEndOfData);
        }

        Ok(ObjectIdentifier {
            components: components,
        })
    }

    pub fn components(&self) -> &[u64] {
        self.components.as_ref()
    }

    #[inline]
    pub fn bytes_len(&self) -> usize {
        if self.components.len() < 2 {
            panic!();
        }

        let mut it = self.components.iter();

        let first_byte_hi = it.next().unwrap();
        if *first_byte_hi > 256/40 {
            panic!();
        }

        let first_byte_lo = it.next().unwrap();
        if *first_byte_lo >= 40 {
            panic!();
        }

        let mut len = 1;

        for component in it {
            len += byte_len_for_component(*component);
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
        if self.components.len() < 2 {
            panic!();
        }

        let mut len = 0;

        let mut it = self.components.iter();

        let first_byte_hi = it.next().unwrap();
        if *first_byte_hi > 256/40 {
            panic!();
        }

        let first_byte_lo = it.next().unwrap();
        if *first_byte_lo >= 40 {
            panic!();
        }

        len += try!(w.write(&[ (first_byte_hi * 40 + first_byte_lo) as u8 ]));

        for component in it {
            let remainder_bytes_len = 9;
            let mut remainder_bytes_used = 0;
            let mut remainder_bytes = [0u8; 9];
            let mut is_last_byte = true;

            let mut component = *component;
            while component > 0 {
                let component_sevenbits: u8 = (component % 128) as u8;
                component /= 128;
                let continuation_overlay: u8 = if is_last_byte {
                    0
                } else {
                    0x80
                };

                remainder_bytes[remainder_bytes_len - remainder_bytes_used - 1] = component_sevenbits | continuation_overlay;
                remainder_bytes_used += 1;
                is_last_byte = false;
            }

            len += try!(w.write(&remainder_bytes[(remainder_bytes_len - remainder_bytes_used)..remainder_bytes_len]));
        }

        Ok(len)
    }
}

#[inline]
fn byte_len_for_component(mut c: u64) -> usize {
    let mut len = 0;
    while c > 0 {
        len += 1;
        c /= 128;
    }
    len
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oid_decode1() {
        {
            let input = [0x2au8, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02];

            let oid = ObjectIdentifier::from_bytes(&input).unwrap();

            assert_eq!(oid.components(), &[ 1, 2, 840, 113549, 1, 7, 2]);
        }
    }

    #[test]
    fn test_oid_roundtrip1() {
        {
            let input = [0x2au8, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02];

            let oid = ObjectIdentifier::from_bytes(&input).unwrap();

            assert_eq!(oid.components(), &[ 1, 2, 840, 113549, 1, 7, 2]);

            let oid_bytes = oid.to_bytes();

            assert_eq!(oid_bytes.len(), oid.bytes_len());
            assert_eq!(oid_bytes, input);
        }
    }
}
