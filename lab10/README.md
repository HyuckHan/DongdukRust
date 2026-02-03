# Lab 10

The goal of this lab is to give you a better understanding of **lifetimes** in Rust.

So far, we have mostly dealt with owned values. However, there is overhead to dealing with owned
values everywhere, as holding multiply clones of data can be inefficient and expensive. For
performance-critical applications, it is often necessary to use references instead of owned values.
When you use references, you are also using lifetimes, even if you can't see them directly.

In this lab, you will implement functionality similar to that of the [`split`] method on
[`str`] in the Rust standard library. You will need to construct a `Split` struct that implements
the [`Iterator`] trait, where the items it yields are slices of the original string (the
`haystack`), delimited by some `delimiter`.

A correct solution is likely going to contain very few lines of code (~20). However, the interaction
between the lifetimes and your code will probably be tricky to implement on your own.

Instead of trying to figure this out on your own, we ask that you watch the following livestream
called ["Crust of Rust: Lifetime Annotations"](https://youtu.be/rAl-9HwD858?si=VTQfI8Re7DvrtDqy).
This is a live-coding stream on YouTube where Jon Gjengset (a very notable educator in the Rust
community) explains exactly how to implement this iterator.

[`split`]: https://doc.rust-lang.org/std/primitive.str.html#method.split
[`str`]: https://doc.rust-lang.org/std/primitive.str.html

## More Examples (rustlings)

[16\_lifetimes](https://github.com/rust-lang/rustlings/tree/main/exercises/16_lifetimes)
