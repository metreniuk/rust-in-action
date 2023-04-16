const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 0.1 + 0.2;
    let (sign, exponent, mantissa) = to_parts(n);

    // println!(
    //     "{} ----- {:01b} {:08b} {:023b}",
    //     n, sign, exponent, mantissa
    // );
    println!("{}", n);
    println!("{:01b}  |  {:08b}   |   {:23b}", sign, exponent, mantissa);

    let (sign, exponent, mantissa) = decode(sign, exponent, mantissa);

    println!("{}  |  {}   |   {}", sign, exponent, mantissa);

    println!("decode: {}", from_parts(sign, exponent, mantissa));
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = bits & 0x7fffff;

    (sign, exponent, mantissa)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let sign_val = (-1.0_f32).powf(sign as f32);
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let bit_at_i = fraction & mask;
        if bit_at_i != 0 {
            let weight = 2.0_f32.powf((i as f32) - 23.0);
            mantissa += weight;
        }
    }

    (sign_val, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
