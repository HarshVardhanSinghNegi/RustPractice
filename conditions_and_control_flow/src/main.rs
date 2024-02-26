fn main() {
    /*Condition */
    let cond=(2 as f32)<3.0; //convert to float to prevent mismatch error.
    println!("{}",cond);

    /*Compound Condition */
    let cond2 = true&&cond;
    println!("{}",cond2);

    /*Control Flow */
    let food="cookie";
    if food!="cookie"{
        println!("I like cookies");
    }else if food=="cookie"{
        println!("I don't like cookies!");
    }else{
        println!("I don't like anything else");
    }
}
