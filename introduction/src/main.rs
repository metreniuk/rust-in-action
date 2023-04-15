use std::mem::transmute;

#[allow(arithmetic_overflow)]

fn main() {
    let a: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let b: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(a) };
    let b: i32 = unsafe { transmute(b) };

    print!("{} {}", a, b)
}
