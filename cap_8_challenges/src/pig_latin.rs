pub fn pig_latin(text: String) -> String {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut text_chars: Vec<char> = text.chars().collect();
    if vowels.contains(&text_chars[0]) {
        format!("{}-hay", text)
    } else {
        let letter: char = text_chars[0];
        text_chars.remove(0);
        let new_text: String = text_chars.iter().collect();
        format!("{}-{}ay", new_text, letter)
    }
}
