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
    base '*' mul
    base '/' mul
    base '%' mul

base:
    number
    '-' base
    '+' base
    '(' add ')'
    '|' add '|'

number:
    /[0-9.]+/
```

## License

Copyright 2023 Runefox

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
