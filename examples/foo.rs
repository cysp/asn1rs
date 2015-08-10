extern crate asn1;

// use std::io::Stdin;
// use std::prelude::*;


fn main() {
    let mut stdin = std::io::stdin();
    let mut stdin_data = Vec::new();
    std::io::Read::read_to_end(&mut stdin, &mut stdin_data).unwrap();
    let r = asn1::Asn1Reader::new(&stdin_data);
    for item in r { match item {
        Ok((t, v)) => {
            println!("item: {:?}", t);
            match t.class() {
                asn1::identifier::Class::Universal => {
                    match (t.is_constructed(), asn1::identifier::UniversalTag::from_u64(t.tag())) {
                        (true, Some(asn1::identifier::UniversalTag::Sequence)) => {
                            let r = asn1::Asn1Reader::new(v);
                            for item in r { match item {
                                Ok((t, v)) => {
                                    match t.class() {
                                        asn1::identifier::Class::Universal => {
                                            println!("  item: {:?}", t);
                                        }
                                        _ => {
                                            print!("  ");
                                            println!("{:?}", v);
                                        }
                                    }
                                },
                                Err(_) => (),
                            }}
                        }
                        _ => {
                            print!("  ");
                            println!("{:?}", v);
                        }
                    }
                }
                _ => {
                    print!("  ");
                    println!("{:?}", v);
                }
            }
            let _ = v;
        },
        Err(_) => (),
    }}
}
