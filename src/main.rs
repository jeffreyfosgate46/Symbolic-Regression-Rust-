// SYMBOLIC REGRESSION GENETIC ALGORITHM IN RUST
// By Jeffrey Fosgate
// Last updated September 21, 2026

// Symbolic regression is a problem that lends itself well to genetic
// programming. Given a set of x- and y-values, one must discover an equation
// (i.e., y = 2x^2 + 3x + 4) that corresponds perfectly to all given (x, y)
// pairings.

// Please note that this is NOT the best way to solve this problem. A tree
// would be far nicer here than a bunch of structs and enumerators, but I
// want to practice using structs and enums, so structs and enums it shall be!

// TODO: Many symbolic regression implementations do not restrict themselves
// to polynomials, and introduce trig functions and other math concepts into
// the frey. Consider working with this Trig_Func enum below:

/*
enum Trig_Func {
    Sine,
    Cosine,
    Tangent,
}
*/

// What has Jeffrey tried so far in vain to get everything here working?

/*
1.) Define one struct (Term) and one enum (Operation), where Term represents
a single polynomial term (i.e., 3x^2) and Operation represents an operation
performed upon two of these terms (i.e., the "+" in 3x^2 + 2x^3).

PROBLEMS:
    * This setup required defining Operation in terms of generics restricted
    by a certain trait (IsEqn), which presented a problem, as THE COMPILER
    MUST KNOW EVERYTHING'S DATA TYPE AT COMPILE-TIME. For this reason, a recurring issue
    was "Expected T, got <some other type>", even when <some other type> matched
    T's specifications. T was not restricted to just one data type, hence the error!

CONCEPTS LEARNED:
    * The reason for the double <L, R> in generic implementations like
    "impl <L, R> Operation <L, R>" is to tell Rust that the following functions
    are being implemented with respect to some generic-typed thing.
    * Again, THE TYPES OF EVERYTHING MUST BE KNOWN AT COMPILE-TIME. This is
    another safety feature of Rust: there should be no circumstance where a
    variable can end up as a string in one iteration, and a float in another!
    * .concat() is AWESOME! Just give the function any vector or array of
    disparate strings, and it'll make one string out of those pieces!!
    * Traits in general: how to define 'em ("trait MyTrait { ... }"), how to
    define their associated functions ("fn MyFn (&self) -> String;") or ("fn
    MyFn(&self) -> String { ... }" for default behavior), and how to use them
    in restricting generics ("impl<T> MyStruct<T> where T: MyTrait").

2.) Define one enum (Equation), where Equation can represent either a single
term with "x" in it (i.e., 3x^2), or multiple such terms composited together
by some operation (::Add, ::Subtract, etc.) This resolves the previous issue
of an Equation's data type being unknown at compile-time.

PROBLEMS:
    * So many issues related to "recursive types". Since an Equation can
    technically be comprised of multiple smaller Equations, one needs to
    introduce indirection in Equation's definition (namely, through fixed-
    sized pointers, like Box<Equation> or &Equation). From rustc --explain
    E0072:
        @ "When defining a recursive struct or enum, any use of the type 
        being defined from inside the definition must occur behind a pointer
        (like `Box`, `&` or `Rc`). This is because structs and enums must 
        have a well-defined size, and without the pointer, the size of the
        type would need to be unbounded."

    * 

CONCEPTS LEARNED:
    * There are crucial differences between String, str, and &str!
        @ A STRING (i.e., String::new()) is a struct that kind of acts like
        a fat pointer, in that it consists of a pointer to the beginning of
        the string in the heap (represented by a Vec<u8>), a string length,
        and a string capacity. 
        Its size IS fixed: its overall size is simply
        that of the pointer, plus that of the capacity integer, plus that of
        the length integer, all computer architecture-dependent.
        @ A "str" DATATYPE is a sequence of heap-stored characters that is
        dynamic: its aggregate size can increase or decrease depending on what
        a program does to it.
        Its size IS NOT fixed: it is an example, rare in Rust, of a Dynamic Sized
        Type (DST), whose size is indeterminate at compile-time. In more
        Rust-y terms, a "str" does not implement the "Sized" trait, which
        specifies that a data type possesses a fixed length that the Rust
        compiler can know at compile-time. Thus, you CANNOT DIRECTLY USE IT
        IN YOUR CODE! So, basically, don't dereference a string slice!!
        @ A STRING SLICE (&str) is a borrowed subset of characters from within
        these heap-stored character (str) types. A string slice can be either
        directly created from a string literal ("let myStr: &str = 'Hello!';")
        or produced as a subset of a String ("let myStr: String = String::
        from('Hello!'); let myStrSlice: &str = myStr[..=4];").
        Its size IS fixed: It is a "fat pointer" consisting of a pointer to the
        beginning of the string, and a u8 string length.

*/

use rand::{random_bool, random_range};

enum Equation {
    Add(Box<Equation>, Box<Equation>),
    Subtract(Box<Equation>, Box<Equation>),
    Multiply(Box<Equation>, Box<Equation>),
    Divide(Box<Equation>, Box<Equation>),
    Term(f32, f32),
    //Sine(Box<Equation>),
    //Cosine(Box<Equation>),
    //Tangent(Box<Equation>),
}

impl Equation {
    /// Generates a
    fn random_op(lhs: Box<Equation>, rhs: Box<Equation>) -> Box<Equation> {
        Box::new(match random_range(1u8..=4u8) {
            1 => Equation::Add(lhs, rhs),
            2 => Equation::Subtract(lhs, rhs),
            3 => Equation::Multiply(lhs, rhs),
            4 => Equation::Divide(lhs, rhs),
            _ => *Equation::empty(), // This should never happen!!
        })
    }

    /// Generates an expression of the form `(coeff)x^(exp)`.
    /// #### Parameters
    /// * `val_rnge: f32`: Coefficients and exponents randomly generated within 
    /// the equation can only possess values between `-(val_rnge)` and `val_rnge` (inclusive).
    fn random_term(val_rnge: f32) -> Box<Equation> {
        Box::new(Equation::Term(random_range(-(val_rnge)..=val_rnge), random_range(-(val_rnge)..=val_rnge)))
    }

    /// Generates an equation with a random structure, using random mathematical
    /// operations and random coefficient and exponent values for all terms.
    /// #### Parameters
    /// * `val_rnge: f32`: Coefficients and exponents randomly generated within 
    /// the equation can only possess values between `-(val_rnge)` and `val_rnge` (inclusive).
    /// * `extend_chnc: f32`: The likelihood (out of 100.0[%]) that an Equation will
    /// "extend" from one of its sides, such that the Equation operates upon another
    /// Equation.
    fn random(val_rnge: f32, extend_chnc: f32) -> Box<Equation> {
        assert!(extend_chnc > 0.0 && extend_chnc <= 100.0);

        let (extend_lhs, extend_rhs) = (random_bool((extend_chnc / 100.0) as f64), random_bool((extend_chnc / 100.0) as f64));
        Equation::random_op(
            if extend_lhs {
                Equation::random(val_rnge, extend_chnc / 2.0)
            } else {
                Equation::random_term(val_rnge)
            },
            if extend_rhs {
                Equation::random(val_rnge, extend_chnc / 2.0)
            } else {
                Equation::random_term(val_rnge)
            },
        )
    }

    /// Produces a new equation consisting of a single term: `(0.0)x^(0.0)`.
    fn empty() -> Box<Equation> {
        Box::new(Equation::Term(0.0f32, 0.0f32))
    }

    /// Returns a String representing the Equation.
    fn prnt_eqn(&self) -> String {
        match self {
            Equation::Add(lhs, rhs) => format!("{}{}{}", lhs.prnt_eqn(), " + ", rhs.prnt_eqn()),
            Equation::Subtract(lhs, rhs) => format!("{}{}{}", lhs.prnt_eqn(), " - ", rhs.prnt_eqn()),
            Equation::Multiply(lhs, rhs) => format!("{}{}{}", lhs.prnt_eqn(), " * ", rhs.prnt_eqn()),
            Equation::Divide(lhs, rhs) => format!("{}{}{}", lhs.prnt_eqn(), " / ", rhs.prnt_eqn()),
            Equation::Term(coeff, exp) =>
            if *coeff == 0.0 {
                "0".to_string()
            } else if *exp == 0.0 {
                format!{"{}", *coeff}
            } else {
                format!("{}x{}", *coeff, if *exp == 1.0 {String::new()} else {format!("^{}", *exp)})
            },
        }
    }

    /// Evaluates this 
    fn eval_eqn(&self, x_val: f32) -> f32 {
        match self {
            Equation::Add(lhs, rhs) => lhs.eval_eqn(x_val) + lhs.eval_eqn(x_val),
            Equation::Subtract(lhs, rhs) => lhs.eval_eqn(x_val) - lhs.eval_eqn(x_val),
            Equation::Multiply(lhs, rhs) => lhs.eval_eqn(x_val) * lhs.eval_eqn(x_val),
            Equation::Divide(lhs, rhs) => {
                if rhs.eval_eqn(x_val) == 0.0f32 {
                    f32::MAX
                } else {
                    lhs.eval_eqn(x_val) / rhs.eval_eqn(x_val)
                }
            },
            Equation::Term(coeff, exp) => *coeff * x_val.powf(*exp),
        }
    }

    fn depth (&self) -> u8 {
        unimplemented!();
    }

    fn crossover(&self, other_eqn: Box<Equation>) -> Box<Equation> {

    }
}

// TODO: Once this function has been successfully implemented, make it work
// with a CSV!

pub fn symbolic_regression_genetic( points: [(f32, f32)],
                                    pop_size: u8, 
                                    mutation_chance: f32, 
                                    generations: u8) {
    assert!(generations > 0 && (mutation_chance > 0.0 && mutation_chance <= 100.0) && pop_size > 0);

    let mut population: [Box<Equation>; 10] = [Equation::random(5.0, 50.0); 10];

    for _gen_idx in 0..generations{

    }
}

fn main() {
    // A test equation equivalent to y = (3x^2 + 4x - 2).
    let sample_points: [(f32, f32); 10] = [ (30.0, 6.0), (6.0, 15.0),
                                            (16.0, 30.0), (2.0, 7.0),
                                            (22.0, 3.0), (23.0, 16.0),
                                            (27.0, 3.0), (19.0, 26.0),
                                            (26.0, 29.0), (29.0, 25.0)];
    let test_eqn: Box<Equation> = Equation::random(10.0f32, 50.0f32);
    let test_eqn_2: Box<Equation> = Equation::random(10.0f32, 50.0f32);
    println!("Here is a crossover of {} and {}: {}", test_eqn.prnt_eqn(), test_eqn_2.print_eqn(), test_eqn.crossover(test_eqn_2).print_eqn());
}

// === OLD IMPLEMENTATION BELOW -- PLEASE IGNORE ===

/*trait IsEqn {
    fn eqn_str(&self) -> String {
        String::new()
    }

    fn eval_eqn(&self, x_val: f32) -> f32;

    fn random (term_val_rnge: f32, extend_chnc: f32) -> Self;
}

struct Term {
    coeff: f32,
    exp: f32,
}

impl IsEqn for Term {
    fn eval_eqn (&self, x_val: f32) -> f32 {
        self.coeff * (x_val.powf(self.exp))
    }

    fn eqn_str(&self) -> String {
        format!("{}{}",
                self.coeff,
                match self.exp {
                0.0f32 => "".to_string(),
                1.0f32 => "x".to_string(),
                other_exp => format!("x^{}", self.exp),
                })
    }

    fn random (term_val_rnge: f32, _extend_chnc: f32) -> Term {
        Term{ coeff: rand_range(-(term_val_rnge)..=term_val_rnge), 
              exp: rand_range(-(term_val_rnge)..=term_val_rnge) }
    }
}

// TODO: Add something here for invalid operations.
enum Operation <L, R> {
    Add(L, R),
    Subtract(L, R),
    Multiply(L, R),
    Divide(L, R),
}

impl <L, R> IsEqn for Operation <L, R>
where L: IsEqn, R: IsEqn {
    fn random (term_val_rnge: f32, extend_chnc: f32) -> Operation <L, R> {
        assert!(extend_chnc >= 0.0f32 && extend_chnc <= 100.0f32);

        let extend_eq_side: (bool, bool) = (if rand_range(0.0f32..=100.0f32) >= extend_chnc { true } else { false },
                                            if rand_range(0.0f32..=100.0f32) >= extend_chnc { true } else { false });

        let eqn_lhs = Operation::random(term_val_rnge, extend_chnc / 2.0);
        let eqn_rhs = Operation::random(term_val_rnge, extend_chnc / 2.0);

        if !extend_eq_side.0 {
            let eqn_lhs = Term::random(term_val_rnge, extend_chnc / 2.0);
        } if !extend_eq_side.1 {
            let eqn_rhs = Term::random(term_val_rnge, extend_chnc / 2.0);
        }

        match rand_range(1u8..=4u8) {
            1 => Operation::Add(eqn_lhs, eqn_rhs),
            2 => Operation::Subtract(eqn_lhs, eqn_rhs),
            3 => Operation::Multiply(eqn_lhs, eqn_rhs),
            4 => Operation::Divide(eqn_lhs, eqn_rhs),
        }
    }

    fn eval_eqn(&self, x_val: f32) -> f32 {
        match self {
            Operation::Add(lhs, rhs) => lhs.eval_eqn(x_val) + rhs.eval_eqn(x_val),
            Operation::Subtract(lhs, rhs) => lhs.eval_eqn(x_val) - rhs.eval_eqn(x_val),
            Operation::Multiply(lhs, rhs) => lhs.eval_eqn(x_val) * rhs.eval_eqn(x_val),
            Operation::Divide(lhs, rhs) => { if rhs.eval_eqn(x_val) == 0.0f32 { // Cannot divide by zero!
                                                        f32::MAX
                                                    } else {
                                                        lhs.eval_eqn(x_val) / rhs.eval_eqn(x_val)
                                                    }},
        }
    }

    fn eqn_str(&self) -> String {
        let (lhs, rhs, op): (&L, &R, &str) = 
        match self {
            Operation::Add(lh, rh) => (lh, rh, " + "),
            Operation::Subtract(lh, rh) => (lh, rh, " - "),
            Operation::Multiply(lh, rh) => (lh, rh, " * "),
            Operation::Divide(lh, rh) => (lh, rh, " / "),
        };

        [lhs.eqn_str(),
        op.to_string(),
        rhs.eqn_str()].concat()
    }
}

impl <L, R> Operation <L, R> {

    fn op_str(&self) -> String {
        match self {
            Operation::Add(_, _) => " + ",
            Operation::Subtract(_, _) => " - ",
            Operation::Multiply(_, _) => " * ",
            Operation::Divide(_, _) => " / ",
        }.to_string()
    }
}*/