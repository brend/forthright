# forthright

A tidy interpreter for a tiny subset of *FORTH* in under 200 lines of code.

## Building and Usage

*forthright* is a pure Rust project you can easily build using cargo:

``` sh
$ cargo build --release
```

Install the [Rust toolchain](https://rust-lang.org/learn/get-started/) first if you need to.

And run the built program. Command line arguments are interpreted as a *FORTH* program:

``` sh 
$ target/release/forthright 3 4 + .
```

This will run the program `3 4 + .` and thus print `7`.

If no arguments are give, standard input is read and evaluated:

``` sh
$ echo "5 dup * ." | target/release/forthright
```

This will print `25`.

You can run the tests also using cargo:

``` sh
$ cargo test
```
