use std::collections::HashMap;
use std::io;

fn fizzbuzz(n: &u64, map: &mut HashMap<u64, String>) {
    let mut i: u64 = 1;

    while 0 < i && i <= *n {
        // Check if value already exists in hashmap
        match map.get(&(i % 15)) {
            Some(output) => {
                // If so, output the precalculated value
                println!("{}", output);

                // Increment counter
                i += 1;
            }
            None => {
                // If not calculate result and then store in hashmap
                if i % 15 == 0 {
                    map.insert(i, "FizzBuzz".to_string());
                } else if i % 3 == 0 {
                    map.insert(i, "Fizz".to_string());
                } else if i % 5 == 0 {
                    map.insert(i, "Buzz".to_string());
                } else {
                    map.insert(i, format!("{}", i));
                }
            }
        }
    }
}

fn main() {
    let mut map: HashMap<u64, String> = HashMap::new();

    fizzbuzz(&5, &mut map);
    println!("Map after running:\n{:#?}", map);
}
