use std::str;
use std::u8;
use std::fmt::Write;
                            //https://doc.rust-lang.org/beta/std/fmt/#formatting-traits

                            //TODO - wymienić implementację na prostszą
pub fn to_hex(input: &[u8]) -> String {
    
    let mut out = String::new();

    for &byte in input {
        write!(&mut out, "{:02x}", byte).unwrap();
    }

    out
}
                                                        //TODO - uogólnić tą funkcję
pub fn convert_from_hex(hash: &[u8]) -> [u8; 20] {
    
    let mut out = [0; 20];
    
    for index in 0..20 {
        let (_, tail) = hash.split_at(2 * index);
        let (range, _) = tail.split_at(2);
        
        out[index] = from_hex(range);
    }
    
    out
}

fn from_hex(slice: &[u8]) -> u8 {
    
    let slice_str = str::from_utf8(&slice).unwrap();
    u8::from_str_radix(slice_str, 16).unwrap()
}