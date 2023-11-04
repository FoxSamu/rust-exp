// Note that Expression instances are put in Boxes. This is because the Expression trait
// is a trait. A trait is not a concrete type that the compiler can determine the size of,
// it's a dynamically sized type. Boxes allocate this size at runtime, on the heap, and drop
// it when the box gets dropped. If we were to deal with expressions directly or as references,
// the compiler will complain because it cannot determine a compile-time size of the expression
// type.
//
// For this very reason, you cannot have trait types directly. You can, however, add a generic
// type parameter to your function, with a Sized + Expression bound. The compiler will then
// duplicate your function for any separate type you use it on, and it can ensure that those
// types have a compile time size due to the Sized trait, and that it are Expression types due
// to the Expression trait. That's what the val function does.

/// Anything that can evaluate as an expression. Usually, expressions are dealt with
/// in [Box]es.
pub trait Expression {
    /// Evaluates the expression.
    fn eval(&self) -> f64;
}

/// Creates a boxed expression that's a single value. Any sized value that implements
/// the [Expression] trait is valid.
pub fn val<T: Sized + Expression + 'static>(l: T) -> Box<dyn Expression>  {
    Box::new(l)
}

/// Creates a boxed expression that's the sum of two inner expressions.
pub fn add(l: Box<dyn Expression>, r: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Add(l, r))
}

/// Creates a boxed expression that's the difference of two inner expressions.
pub fn sub(l: Box<dyn Expression>, r: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Sub(l, r))
}

/// Creates a boxed expression that's the product of two inner expressions.
pub fn mul(l: Box<dyn Expression>, r: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Mul(l, r))
}

/// Creates a boxed expression that's the quotient of two inner expressions.
pub fn div(l: Box<dyn Expression>, r: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Div(l, r))
}

/// Creates a boxed expression that's the remainder of two inner expressions.
pub fn rem(l: Box<dyn Expression>, r: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Rem(l, r))
}

/// Creates a boxed expression that's the negation of an inner expression.
pub fn neg(e: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Neg(e))
}

/// Creates a boxed expression that's the absolute of an inner expression.
pub fn abs(e: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Operator::Abs(e))
}

/// An operator expression, which joins two expressions.
pub enum Operator {
    /// The sum of two expressions.
    Add(Box<dyn Expression>, Box<dyn Expression>),

    // The difference of two expressions.
    Sub(Box<dyn Expression>, Box<dyn Expression>),

    /// The product of two expressions.
    Mul(Box<dyn Expression>, Box<dyn Expression>),

    /// The quotient of two expressions.
    Div(Box<dyn Expression>, Box<dyn Expression>),

    /// The remainder of two expressions.
    Rem(Box<dyn Expression>, Box<dyn Expression>),

    /// The negation of two expressions.
    Neg(Box<dyn Expression>),

    /// The absolute of two expressions.
    Abs(Box<dyn Expression>),
}


impl Expression for Operator {
    fn eval(&self) -> f64 {
        match self {
            Operator::Add(left, right) => left.eval() + right.eval(),
            Operator::Sub(left, right) => left.eval() - right.eval(),
            Operator::Mul(left, right) => left.eval() * right.eval(),
            Operator::Div(left, right) => left.eval() / right.eval(),
            Operator::Rem(left, right) => left.eval() % right.eval(),
            Operator::Neg(exp) => -exp.eval(),
            Operator::Abs(exp) => _abs(exp.eval())
        }
    }
}


/// Absolute value function.
fn _abs(n: f64) -> f64 {
    if n < 0.0 {
        -n
    } else {
        n
    }
}



// All numeric values are expressions that evaluate to themselves as f64. Rust
// allows you to add traits to existing types. Since numbers are 'Sized', the
// compiler now allows you to call the 'val' function on any of the types
// implementing below.

// Floats
impl Expression for f64 {
    fn eval(&self) -> f64 {
        *self
    }
}

impl Expression for f32 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}


// Signed integers
impl Expression for i8 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for i16 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for i32 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for i64 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for i128 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}


// Unsigned integers
impl Expression for u8 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for u16 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for u32 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for u64 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}

impl Expression for u128 {
    fn eval(&self) -> f64 {
        *self as f64
    }
}
