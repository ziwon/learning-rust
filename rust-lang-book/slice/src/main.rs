fn main() -> Result<(), std::io::Error> {
    let  s = String::from("hello world");

    let word = first_word(&s[..]);
    println!("{}", word);

    let ss = "hello world";

    let word = first_word(&ss[..]);
    println!("{}", word);

    Ok(())
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
