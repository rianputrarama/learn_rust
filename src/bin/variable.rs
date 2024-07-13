fn main() {
    let mut x = 5;
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");


    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 *3;
    println!("result 3 hours in second : {}", THREE_HOURS_IN_SECOND);

    // example of shadowing in rust
    let i = 10;

    let i = i + 5;

    {
        let i = i * 2;
        println!("the value of i in the inner scope is: {i}");
    }

    println!("the value of i is: {i}")
}