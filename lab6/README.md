# Lab 6

[Mini Grep Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## Extra Feature (mini grep)

The extra feature you want to add is **up to you** once you've finished the base `minigrep`.

- A very basic feature that you could add is a "count" flag through `-c` or `--count`, which changes
  the output to show how many lines a pattern is in, rather than printing out all of the lines.
- Another feature you could implement is searching directories as well as specific files.
- You can add regex support by using the [`regex`](https://docs.rs/regex/latest/regex/) crate.
- You can integrate a third-party CLI library such as [`clap`](https://docs.rs/clap/latest/clap/) to
  make the command line interface more user-friendly.
- You can integrate a third-party error-handling library such as
  [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) or
  [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) into your error handling code.

## More Examples (rustlings)

[10\_modules](https://github.com/rust-lang/rustlings/tree/main/exercises/10_modules)

[17\_tests](https://github.com/rust-lang/rustlings/tree/main/exercises/17_tests)
