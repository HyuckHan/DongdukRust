/// Make me compile by only reordering the lines!

fn wait_for_me_to_speak() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    &s[..1]
}

fn main() {
    wait_for_me_to_speak();
}
