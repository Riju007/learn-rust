use std::io;
fn main() {
    println!("Enter a number: ");
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read");
    println!("Input was: {user_input}");
    // shadowing a variable.
    let user_input: i32 = user_input
        .trim()
        .parse()
        .expect("Please enter a valid number.");
    if user_input < 2 {
        println!("A prime number must be greater than 1.")
    }
    else {
        let user_input_sqrt = (user_input as f32).sqrt().floor() as i32;
        println!("User Input Sqrt: {}", user_input_sqrt);
        let mut is_prime: bool = true;
        for i in 2..=user_input_sqrt {
            if user_input % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("Prime Number")
        }
        else {
            println!("Not a prime number")
        }
    }
}
