// Characters (`char`)

fn is_phext_delimiter(letter_in: char) -> bool {
    let letter:u8 = letter_in as u8;
    
    letter == 0x17 || letter == 0x18 || letter == 0x19 ||
    letter == 0x1A || letter == 0x1C || letter == 0x1D ||
    letter == 0x1E || letter == 0x1F || letter == 0x01
}
fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '\x17';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else if is_phext_delimiter(your_character) {
        println!("A Wild Phext Delimiter!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
