fn main() {

    let mut user_input = String::new();
    println!("Enter a number to calculate its factorial:");
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let number: u32 = user_input.trim().parse().expect("Please enter a valid number");
    let factorial = |n: u32| -> u32 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    };

 
    println!("The factorial of {} is {}", number, factorial(number));
}
