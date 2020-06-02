fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word is: {}",word);
}
fn first_word(s: &String) -> &str {
    let byte_array = s.as_bytes();
    for (i, &element) in byte_array.iter().enumerate() {
        if element == b' ' {
            return &s[..i]
        }
    }
    &s
}
