fn main() {
    // Primitive Data Types
    // Scalar
    // a)integer
    let x: i32 =2;
    println!("x is {}",x);
    let y: u32= 2;
    println!("y is {}",y);

    // b)float
    let f:f32=0.9;
    println!("f is {}",f);

    // c)bool
    let b:bool=false;
    println!("b is {}",b);

    // d)char
    let c:char='j';
    println!("c is {}",c);

    // Compound
    // a)tuple
    let mut tup:(i32,bool,char)=(9,true,'i');
    tup.2='r';
    println!("tup is {}, {}",tup.2,tup.0);

    // b)arrays
    let mut arr: [i32;5]=[1,2,3,4,5];
    arr[2]=15;
    println!("arr is {}",arr[2]);

    // assignment
    let x: u8=4;
    // let y: i32=x;  (cant convert type while assigning)
    let y: i32=4;
    println!("x:{}, y:{}",x,y);

}
