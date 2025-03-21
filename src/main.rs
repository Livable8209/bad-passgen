use rand::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();

    let words = include_str!("../wordlist.txt"); //fs::read_to_string("wordlist.txt")?;
    let words: Vec<&str> = words.split("\n").collect();

    for _ in 0..5 {
        let chosen_words = words.choose_multiple(&mut rng, 8);
        let mut used_number = false;

        for word in chosen_words {
            print!("{}", word);
            if !&used_number && rng.gen_bool(0.5) {
                used_number = true;

                print!("{}", rng.gen_range(1..=9));
            }
            print!(" ");
        }
        println!();
    }

    print!("{}", rng.gen_range(1..=9));

    println!("Done");
    Ok(())
}
