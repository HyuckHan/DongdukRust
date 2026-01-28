/// Make me compile!

fn youre_square() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num;
}

fn main() {
    youre_square();
}
