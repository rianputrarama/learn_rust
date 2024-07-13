fn main() {
    let guess: u32="32".parse().expect("not a number!");
    assert_eq!(guess, 32);

    let four: u32 = "4".parse().unwrap();

    assert_eq!(4, four);

    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    println!("{}", x.unwrap());
    
}