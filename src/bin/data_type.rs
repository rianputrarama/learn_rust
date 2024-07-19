use std::io;

#[allow(non_snake_case)]

fn main() {
    let guess: u32="32".parse().expect("not a number!");
    assert_eq!(guess, 32);

    let four: u32 = "4".parse().unwrap();

    assert_eq!(4, four);

    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    println!("{}", x.unwrap());
    
    let z = String::from("Power of Rust!");
    println!("my word is : {}", z);

    // slicing a string
    let slice = &z[0..5];
    // iterating over string
    for c in slice.chars() {
        println!("{}", c);
    }

    // create empty string
    let mut empty_str = String::new();
    empty_str.push_str("Coffee");
    
    println!("{}", empty_str);

    let e: bool = true;
    assert_eq!(e, true);

    let try_float: f32 = 3.42;
    assert_eq!(try_float, 3.42);

    let try_tup: (i32, f32, i8) = (500, 3.14, 1);
    let (_, y, _) = try_tup;
    //access a tuple element
    println!("{}", try_tup.0);
    println!("{y}");
    println!("{}", try_tup.2);

    let try_arr = [1, 2, 3, 4, 5];
    let first = try_arr[0];
    println!("{first}");

    println!("please enter an array index.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = try_arr[index];
    println!("The value of the element at index {index} is: {element}");

}