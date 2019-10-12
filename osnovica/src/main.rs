use std::char;
use std::io;
use std::io::Read;

fn main() {
    let mut start = 65u32;
    let mut njak = vec![start];
    loop {
        start += 1;
        if start > 90 {
            break;
        }
        njak.push(start);
    }
    start += 6;
    loop {
        njak.push(start);
        if start == 122 {
            break;
        }
        start += 1;
    }
    start -= 75;
    loop {
        if start >= 57 {
            break;
        }
        start += 1;
        njak.push(start);
    }
    njak.push(43);
    njak.push(47);
    let mut p: u8 = 0;
    let mut m = 0;
    for i in io::stdin().bytes() {
        let b = i.unwrap();
        m += 37;
        m %= 4;
        if m == 0 {
            p = b >> 2;
            let j = p as usize;
            print!("{}", char::from_u32(njak[j]).unwrap());
            p = (b << 6) >> 2;
        } else if m == 1 {
            p &= b >> 4;
            let j = p as usize;
            print!("{}", char::from_u32(njak[j]).unwrap());
            p = (b << 4) >> 2;
        } else if m == 2 {
            p &= b >> 6;
            let j = p as usize;
            print!("{}", char::from_u32(njak[j]).unwrap());
            p = (b << 2) >> 2;
        } else if m == 3 {
            let j = p as usize;
            print!("{}", char::from_u32(njak[j]).unwrap());
            p = 0;
        }
    }
}
