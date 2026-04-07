fn main() {
    // println!("Hello, world!");

    // print if a number is even or not
    println!("{}", is_even(20));
}

// Function to check if a number is even ?
fn is_even(num: i32) -> bool {
    if num % 2 == 0{
        return  true;
    }
    return  false;
}