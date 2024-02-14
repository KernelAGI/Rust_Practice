// Ask the user for their name, then count how many vowels are in it. Separate the FirstName and LastName and print them together
// Use the string "Hello {FirstName} {LastName}! Your name is {length(FirstName) + length(LastName)} characters long and 
//      has {vowel_count(FirstName) + vowel_count(LastName)}"

use std::io;

fn main() {
    // Ask the user for their name
    println!("Enter your full name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // Separate the first name and last name
    let names: Vec<&str> = name.trim().split_whitespace().collect();
    let first_name = names[0];
    let last_name = names[1];

    // Count the number of vowels in the first name
    let first_name_vowel_count = count_vowels(first_name);

    // Count the number of vowels in the last name
    let last_name_vowel_count = count_vowels(last_name);

    // Calculate the total length of the full name
    let full_name_length = first_name.len() + last_name.len();

    // Print the result
    println!(
        "Hello {} {}! Your name is {} characters long and has {} vowels.",
        first_name,
        last_name,
        full_name_length,
        first_name_vowel_count + last_name_vowel_count
    );
}

fn count_vowels(name: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;

    for c in name.chars() {
        if vowels.contains(&c.to_ascii_lowercase()) {
            count += 1;
        }
    }

    count
}
