# An expression evaluator, in Rust

This is a simple command line application that evaluates expressions. It is written in Rust, as my attempt to learn the language, and I'd like to share the sources for educational purposes. Feel free to browse through the code, there's lot of informative comments about how the code is structured and why I did that.

## Usage
Run using `cargo run`, then enter an expression as soon as the `>>>` appears. Expressions can be made out of basic arithmetic operations and parentheses with the formal operator precedence. The following operators are allowed:
- `+x` gives just `x`
- `-x` gives the negated value of `x`
- `|x|` gives the absolute value of `x`
- `x + y` gives the sum of `x` and `y`
- `x - y` gives the difference of `x` and `y`
- `x * y` gives the product of `x` and `y`
- `x / y` gives the quotient of `x` and `y`
- `x % y` gives the remainder of `x` and `y`

Entering nothing but spaces will terminate the program. Entering an invalid expression will print an error.

## The expression syntax

Given the program ignores any space and tab outside of numbers, and treats any newline as an EOF symbol, the formal syntax is as follows:
```
expr:
    add <EOF>

add:
    mul
    mul '+' add
    mul '-' add

mul:
    base
    base '*' add
    base '/' add
    base '%' add

base:
    number
    '-' base
    '+' base
    '(' add ')'
    '|' add '|'

number:
    /[0-9.]+/
```

