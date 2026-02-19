fn main() {
    // Create a vector with numbers from 1 to 20
    let numbers: Vec<u32> = (1..=20).collect();

    // Define a closure that checks if a number is even
    let is_even = |n: &u32| n % 2 == 0;

    // Use the closure to filter even numbers
    let even_numbers: Vec<u32> = numbers
        .iter()
        .filter(is_even)
        .cloned() // turn &u32 into u32
        .collect();

    // Print the even numbers
    println!("Even numbers: {:?}", even_numbers);
}
