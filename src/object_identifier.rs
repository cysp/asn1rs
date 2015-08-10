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
}
