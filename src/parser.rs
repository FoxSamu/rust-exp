use std::str::Chars;

use crate::expression::*;

// Use enums defined in this file so we don't have to prefix them
// every time.
use ParseResult::*;
use MulOp::*;
use AddOp::*;

/// A parser, which keeps track of the parsing position in the string.

struct Parser<'str> {
    /// The current index in the string.
    idx: usize,

    /// The string being parsed.

    // The 'str is a lifetime modifier, which basically tells that the
    // values referenced here must live at least as long as this
    // Parser value does. The Chars instance internally references a
    // string, so the rust compiler must be able to verify that we can
    // safely reference that string as long as the Parser instance lives.
    str: Chars<'str>,

    /// The current character.
    cur: Option<char>
}

/// A parse result.
pub enum ParseResult {
    /// The parse result that indicates that an expression has been successfully parsed.
    Present(Box<dyn Expression>),
    
    /// The parse result that indicates that there is nothing found in the input that could be parsed.
    Absent,

    /// The parse result that indicates that the input has an incorrect syntax.
    Error(String, usize)
}


/// Parses an expression from a string.
pub fn parse(s: &String) -> ParseResult {
    // About the lifetime of the parser and the string, Rust can verify, because we received
    // the string as a parameter, that it lives as long as the parser value lives. This makes
    // perfect sense, the parser value only lives within this method, the string value lives
    // probably much longer, in the calling method, so the lifetime is much longer than that
    // of the parser.

    // Say we were to add an if statement in this function, where we assign the str field to
    // the chars of a string we created right in that statement, it would not compile, because
    // the string value lives shorter than the parser does.
    let mut parser: Parser = Parser {
        idx: 0,
        str: s.chars(),
        cur: None
    };

    parser.cur = parser.str.next();

    // Match the parse result, note how 'return Error ...', instead of
    // assigning the value to the 'res' variable, instead immediately
    // returns from the function.
    let res = match parser.parse_add() {
        Present(x) => Present(x),
        Absent => Absent,
        Error(x, i) => {
            return Error(x, i);
        }
    };

    // So here we only have Present or Absent
    parser.skip_space();

    // No remaining input? Fine.
    if parser.peek() == None {
        return res;
    }

    // Remaining input is a syntax error.
    return Error(String::from("Extra input"), parser.idx);
}


// Note the difference between 'self', '&self' and '&mut self':
// - 'self' moves the ownership of the instance from the calling function to the called function.
//   This makes that the calling function loses access to this instance and we have to return it
//   again, do we want to give it back to the calling function.
// - '&self' borrows the instance from the calling function for read-only operations. This makes
//   it so that the calling function retains access to this instance, but we cannot dereference
//   it without any issues.
// - '&mut self' is like '&self', but it also allows for modification.
//
// Note how match on a reference, will make all matched variables be the same type of reference as
// well. If we match Present(x) on self, we get x to be Box<...>. If we match it on &self, we get
// x to be &Box<...>. And if we match it on &mut self, we get x to be &mut Box<...>.
//
// See how below functions use these different ways of referencing self.

#[allow(dead_code)] // Some functions here are not used, but we want them anyway
impl ParseResult {
    /// If the result is [Present], applies the function on the resulting expression, which may return
    /// a new [ParseResult]. Otherwise, it returns itself.
    fn monad<F, G>(self, user_data: G, func: F) -> ParseResult
    where
        // F is any function taking user data and a boxed expression, and returning ParseResult
        F : Fn(G, Box<dyn Expression>) -> ParseResult 
    {
        match self {
            Present(x) => func(user_data, x),
            x => x,
        }
    }

    /// If the result is [Present], applies the function on the resulting expression and wraps it back
    /// as [Present]. Otherwise, it returns itself.
    fn map<F, G>(self, parser: G, func: F) -> ParseResult
    where
        // F is any function taking user data and a boxed expression, and returning another
        // boxed expression
        F : Fn(G, Box<dyn Expression>) -> Box<dyn Expression>
    {
        match self {
            Present(x) => Present(func(parser, x)),
            x => x,
        }
    }

    /// Returns true when the result is present.
    pub fn is_present(&self) -> bool {
        match self {
            Present(_) => true,
            _ => false
        }
    }

    /// Returns true when the result is absent.
    pub fn is_absent(&self) -> bool {
        match self {
            Absent => true,
            _ => false
        }
    }

    /// Returns true when the result is an error.
    pub fn is_error(&self) -> bool {
        match self {
            Error(_, _) => true,
            _ => false
        }
    }

    /// Returns an [Option] with the parsed expression, if it is [Present].
    pub fn present(self) -> Option<Box<dyn Expression>> {
        match self {
            Present(x) => Some(x),
            _ => None
        }
    }

    /// Returns an [Option] with the error message, if it is an [Error] result.
    pub fn error(self) -> Option<String> {
        match self {
            Error(x, _) => Some(x),
            _ => None
        }
    }

    /// Returns an [Option] with the error index, if it is an [Error] result.
    pub fn error_index(self) -> Option<usize> {
        match self {
            Error(_, x) => Some(x),
            _ => None
        }
    }
}


/// A multiplication operator.
enum MulOp {
    Mul,
    Div,
    Rem
}


/// An addition operator.
enum AddOp {
    Add,
    Sub
}


// Implementation of the parser.
// Note how we have to specify the lifetime specifier again.
// We have to specify that this all works for any lifetime of
// the parser.
impl<'str> Parser<'str> {

    /// Peeks one character ahead, returns [None] if the end was reached.
    fn peek(&self) -> Option<char> {
        // Interpret newlines as None, since we want them to be the end of input.
        self.cur.and_then(|ch| {
            if ch == '\n' || ch == '\r' {
                None
            } else {
                Some(ch)
            }
        })
    }

    /// Skips a character.
    fn skip(&mut self) -> &mut Self { // Returns itself, the Self type ensures that
        self.cur = self.str.next();
        self.idx += 1;

        self
    }

    /// Skips any spaces and then peeks the next character.
    fn symbol(&mut self) -> Option<char> {
        self.skip_space();
        self.peek()
    }

    /// Skips any spaces in the input.
    fn skip_space(&mut self) {
        while _is_space(self.peek()) {
            self.skip();
        }
    }

    /// Parses a number in the input.
    /// A number has the syntax:
    /// 
    /// ```txt
    /// number:
    /// 1.  /[0-9.]+/
    /// ```
    fn parse_number(&mut self) -> ParseResult {
        self.skip_space();

        let c = self.peek();
        let s = self.idx;

        // If no number present, return Absent
        if !_is_number_char(c) {
            return Absent;
        }

        // Keep reading digits and periods until there are no more
        let mut st = String::new();
        let mut c = self.peek();
        while _is_number_char(c) {
            st.push(c.unwrap());
            c = self.skip().peek();
        }

        // Parse the number as float, if it fails the syntax is
        // incorrect and we give an Error result
        match st.parse::<f64>() {
            Ok(v) => Present(val(v)),
            Err(_) => Error(String::from("Incorrect number"), s)
        }
    }

    /// Parses a base expression in the input.
    /// A base expression has the syntax:
    /// 
    /// ```txt
    /// base:
    /// 1.  number
    /// 2.  '-' base
    /// 3.  '+' base
    /// 4.  '(' add ')'
    /// 5.  '|' add '|'
    /// ```
    fn parse_base(&mut self) -> ParseResult {
        match self.symbol() {
            // Rule 2
            Some('-') => {
                // The - operator negates the expression
                self.skip().parse_base().map(self, |_, exp| {
                    neg(exp)
                })
            },

            // Rule 3
            Some('+') => {
                // The - operator negates the expression
                self.skip().parse_base().map(self, |_, exp| {
                    neg(exp)
                })
            },

            // Rule 4
            Some('(') => {
                self.skip().parse_add().monad(self, |p, exp| {
                    // Expect a closing ')'
                    p.skip_space();
                    if p.peek().map_or(true, |ch| ch != ')') {
                        Error(String::from("Expected ')'"), p.idx)
                    } else {
                        p.skip();
    
                        // Semantically, brackets do nothing, they
                        // just direct the parser in which order to parse.
                        // Just return the expression that we got.
                        Present(exp)
                    }
                })
            },

            // Rule 5
            Some('|') => {
                self.skip().parse_add().monad(self, |p, exp| {
                    // Expect a closint '|'
                    p.skip_space();
                    if p.peek().map_or(true, |ch| ch != '|') {
                        Error(String::from("Expected '|'"), p.idx)
                    } else {
                        p.skip();
    
                        // Between vertical bars, we do the abs operator.
                        Present(abs(exp))
                    }
                })
            },

            // Rule 1
            _ => self.parse_number()
        }
    }

    /// Parses a multiplication expression in the input.
    /// A multiplication expression has the syntax:
    /// 
    /// ```txt
    /// mul:
    /// 1.  base
    /// 2.  base '*' mul
    /// 3.  base '/' mul
    /// 4.  base '%' mul
    /// ```
    fn parse_mul(&mut self) -> ParseResult {
        // Parse left hand side, returning error or absent results
        // immediately
        let lhs = match self.parse_base() {
            Present(x) => x,
            other => return other
        };


        // Determine which operator was used to determine
        // which syntax rule to apply
        let op = match self.symbol() {
            // Rule 2
            Some('*') => Mul,

            // Rule 3
            Some('/') => Div,

            // Rule 4
            Some('%') => Rem,

            // Rule 1; in that case, just return
            // from the function already
            _ => return Present(lhs)
        };

        // Now we have eliminated rule 1, all other rules
        // are the same logic, just different operators:
        let right = self.skip().parse_mul();

        right.map(lhs, |p, rhs| {
            match op {
                Mul => mul(p, rhs),
                Div => div(p, rhs),
                Rem => rem(p, rhs)
            }
        })
    }

    /// Parses a addition expression in the input.
    /// A addition expression has the syntax:
    /// 
    /// ```txt
    /// add:
    /// 1.  mul
    /// 2.  mul '+' add
    /// 3.  mul '-' add
    /// ```
    fn parse_add(&mut self) -> ParseResult {
        // Parse left hand side, returning error or absent results
        // immediately
        let lhs = match self.parse_mul() {
            Present(x) => x,
            other => return other
        };


        // Determine which operator was used to determine
        // which syntax rule to apply
        let op = match self.symbol() {
            // Rule 2
            Some('+') => Add,

            // Rule 3
            Some('-') => Sub,

            // Rule 1; in that case, just return
            // from the function already
            _ => return Present(lhs)
        };

        // Now we have eliminated rule 1, all other rules
        // are the same logic, just different operators:
        let right = self.skip().parse_add();

        right.map(lhs, |p, rhs| {
            match op {
                Add => add(p, rhs),
                Sub => sub(p, rhs)
            }
        })
    }
}

/// Returns true if the given [Option] holds a space character, either a tab or a space.
fn _is_space(c: Option<char>) -> bool {
    match c {
        None => false,
        Some(ch) => ch == ' ' || ch == '\t'
    }
}

/// Returns true if the given [Option] holds a digit or a period.
fn _is_number_char(c: Option<char>) -> bool {
    match c {
        None => false,
        Some(ch) => ch.is_numeric() || ch == '.'
    }
}