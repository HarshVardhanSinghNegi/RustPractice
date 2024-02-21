use std::io;
fn main() {
    // Arithmatic
    
    let x: u8=1;
    let y=255u8;

    let z=x%y;
    println!("{}", z);

    // Type Conversion
    let w=10_u8;
    let v=90 as u16;
    
    let u=(w as u16)+v;
    let t=w%v as u8;
    println!("{}", u);
    println!("{}", t);

    // converting string to int
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("failed to read line");

    println!("string= {}", s);

    let int_i:i64 = s.trim().parse().unwrap();
    println!("integer= {}", int_i+7);
    
}
