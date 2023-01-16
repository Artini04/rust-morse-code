use std::collections::HashMap;
use std::io;

fn main() {
    // Morse Code Table
    // From https://morsecode.world/international/morse2.html
    let morse_dict: HashMap<&str, &str> = HashMap::from([
        ("a", ".-"),
        ("b", "-..."),
        ("c", "-.-."),
        ("d", "-.."),
        ("e", "."),
        ("f", "..-."),
        ("g", "--."),
        ("h", "...."),
        ("i", ".."),
        ("j", ".---"),
        ("k", "-.-"),
        ("l", ".-.."),
        ("m", "--"),
        ("n", "-."),
        ("o", "---"),
        ("p", ".--."),
        ("q", "--.-"),
        ("r", ".-."),
        ("s", "..."),
        ("t", "-"),
        ("u", "..-"),
        ("v", "...-"),
        ("w", ".--"),
        ("x", "-..-"),
        ("y", "-.--"),
        ("z", "--.."),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        ("0", "-----"),
        (" ", "/"), // space
        ("&", ".-..."),
        ("'", ".----."),
        ("@", ".--.-."),
        ("(", "-.--.-"),
        (")", "-.--."),
        (")", "-.--."),
        (":", "---..."),
        (",", "--..--"),
        ("=", "-...-"),
        ("!", "-.-.--"),
        (".", ".-.-.-"),
        ("-", "-....-"),
        ("-", "-....-"),
        ("%", "------..-.-----"), // % => 0 / 0
        ("+", ".-.-."),
        ("\"", ".-..-."),
        ("?", "..--.."),
        ("/", "-..-."),
    ]);

    let mut line = read_line();
    let line_len = line.len() * 3;
    line.pop(); // Remove \n at the end of the string
    line = line.to_lowercase(); // Lowercase all letters
    let line_vec: Vec<char> = line.chars().collect();

    let line_to_morse: String = (0..line.len())
        .map(|n| {
            // IDK how to fix "temporary dropped"
            // when using line_vec[n].to_string().as_str()
            // If it works, it works ig
            let ch = line_vec[n].to_string();
            let chstr = ch.as_str();

            let idx = match morse_dict.get(&chstr) {
                Some(value) => value.to_string(),
                None => "#".to_string(),
            };

            return idx;
        })
        .collect();

    println!("\nCODE: [{: ^line_len$}]", line_to_morse);
}

fn read_line() -> String {
    let mut line = String::new();
    println!("Enter string:");
    let stdinput = io::stdin();
    stdinput.read_line(&mut line).unwrap();
    return line.to_string();
}
