fn first(){
    println!("first");// macro
}
// main function
fn main() {
    sum(6,9);//function call
    hello();
    first();
    // statement
    let a=7;
    println!("expression {}", a);
    // expressions function calls, macros, 7 in the above statement is an expression 
    let number={
        let t:u8=9;
        let a:u8=5;
        t+1-a// no semi colon means return here
    };
    println!("number is: {}",number);

    let add=add(7,8);
    println!("add is: {}",add);
}
// functions
fn hello(){
    println!("Hello");
}

fn sum(x:i64, y:i64){
    println!("sum is {}",x+y);
}

// return
fn add(x:i64,y:i64)->i64{
    x+y
}