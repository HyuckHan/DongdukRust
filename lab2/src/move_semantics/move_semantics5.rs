/// Make me compile!
/// You can't change anything except adding or removing references (`&`).
///
/// Hint:
/// The first function SHOULD NOT take ownership of `data`.
/// The second function SHOULD take ownership of `data`.

fn did_you_get_that_reference() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}

fn main() {
    did_you_get_that_reference();
}
