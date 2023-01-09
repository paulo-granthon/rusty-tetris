#[allow(dead_code)]
pub enum Align { 
    Start,
    Fraction(i32),
    End,
    Custom(i32),
    Relative(Box<Align>, Op<i32>)
}

impl Align {

    // fast constructors
    pub fn center   () -> Self { Align::Fraction(2) }                           // short for Fraction(2)
    pub fn center2  () -> (Self, Self) { (Align::center(), Align::center()) }   // short for (Fraction(2), Fraction(2))
    pub fn start2   () -> (Self, Self) { (Align::Start, Align::Start) }         // short for (Start, Start)

    // returns the i32 value of the Align for the given range 
    pub fn value (&self, size: i32) -> i32 {
        match &self {
            Align::Start => 0,
            Align::Fraction(x) => if x.to_owned() == 0 {0} else {-size / x},
            Align::End => - size,
            Align::Custom(x) => x.to_owned(),
            Align::Relative(x, op) => op.calc(x.value(size)),
        }
    }
}

#[allow(dead_code)]
pub enum Op<T> {
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
}

impl Op<i32> {
    pub fn calc (&self, a: i32) -> i32 {
        match self {
            Op::Add(b) => a + b,
            Op::Sub(b) => a - b,
            Op::Mul(b) => a * b,
            Op::Div(b) => a / b,
        }
    } 
}