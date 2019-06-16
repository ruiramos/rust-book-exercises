// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let s = convert(input.trim());
            println!("{}", s);
        }
        Err(error) => println!("error: {}", error),
    }
}

fn convert(s: &str) -> String {
    let mut chars = s.chars();
    let mut result = String::new();

    if let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                result.push(c);
                String::from("-hay")
            }
            _ => format!("-{}ay", c),
        };

        let rest: String = chars.collect();
        result.push_str(&rest);

        result += &suffix;
    }

    result
}
