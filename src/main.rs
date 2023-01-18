use console::style;
use std::collections::HashMap;
use text_io::read;

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

    print!("{}", style("Enter string: ").italic());
    let mut line: String = read!("{}\n");
    line = line.to_lowercase(); // Lowercase all letters
    let line_vec: Vec<char> = line.chars().collect(); // Get all char

    let line_to_morse: String = (0..line.len())
        .map(|n| {
            // IDK how to fix "temporary dropped"
            // when using line_vec[n].to_string().as_str()
            // If it works, it works ig
            let ch = line_vec[n].to_string();
            let chstr = ch.as_str();

            let idx = match morse_dict.get(&chstr) {
                Some(value) => value.to_string(),
                None => "#".to_string(), // Unknown values will be converted to #
            };

            return idx;
        })
        .collect();

    println!("\n{}{}", style("STRING: ").bold(), line);
    println!("└──  {}[{}]", style("MORSE: ").bold(), line_to_morse);
}
