use std::collections::HashMap;
use std::io::{self, stdout, Write};

fn fizzbuzz_memoisation(n: &u64, map: &mut HashMap<u64, String>) {
    let mut i: u64 = 1;

    while 0 < i && i <= *n {
        let key = i % 15;
        // Check if value already exists in hashmap
        match map.get(&key) {
            Some(output) => {
                // If so, output the precalculated value
                print!("{}", output);

                // If string is blank also output number
                if output == "" {
                    print!("{}", i);
                }

                // Output newline
                println!();

                // Increment counter
                i += 1;
            }
            None => {
                // If not calculate result and then store in hashmap
                if key == 0 {
                    map.insert(key, "FizzBuzz".to_string());
                } else if i % 3 == 0 {
                    map.insert(key, "Fizz".to_string());
                } else if i % 5 == 0 {
                    map.insert(key, "Buzz".to_string());
                } else {
                    map.insert(key, "".to_string());
                }
            }
        }
    }
}

fn fizzbuzz_simple(n: &u64) {
    let mut i = 1;
    while 0 < i && i <= *n {
        let mut printed = false;
        if i % 3 == 0 {
            print!("Fizz");
            printed = true;
        }
        if i % 5 == 0 {
            print!("Buzz");
            printed = true;
        }
        if !printed {
            print!("{}", i);
        }
        println!();

        i += 1;
    }
}

fn main() {
    // Get input from user
    print!("Enter a number: ");
    let _ = stdout().flush();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u64 = n.trim().parse().expect("Expected integer input");
    // let mut map: HashMap<u64, String> = HashMap::new();

    println!("\nRunning fizzbuzz...");
    // fizzbuzz_memoisation(&n, &mut map);
    fizzbuzz_simple(&n);
    // println!("\nMap after running:\n{:#?}", map);
}
