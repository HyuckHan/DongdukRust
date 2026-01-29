// Make me compile without changing the function signature!

fn my_favorite_color() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue"
}

fn main() {
    my_favorite_color();
}
