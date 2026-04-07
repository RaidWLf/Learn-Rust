fn main() {
    // println!("Hello, world!");

    // print if a number is even or not
    println!("{}", fibbonacci(5));
}

// Function to check if a number is even ?
// fn is_even(num: i32) -> bool {
//     if num % 2 == 0{
//         return  true;
//     }
//     return  false;
// }

// function to get fibbonacci of a number
fn fibbonacci(num: i32) -> i32 {
    // Mutable variables
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }
    // for loop
    for _i in 0..num-1 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}