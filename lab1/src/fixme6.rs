/// Make me compile!
///
/// Hint: Destructure the `cat` tuple so that the println will work.
/// This might be [helpful](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

fn old_cats() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}

fn main() {
    old_cats();
}
