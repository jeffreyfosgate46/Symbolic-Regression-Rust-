// SYMBOLIC REGRESSION GENETIC ALGORITHM IN RUST
// By Jeffrey Fosgate
// Last updated September 23, 2026
// CURRENT STATUS: NON-FUNCTIONAL / CONCEPTUAL

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

use rand::{random_bool, random_range};
use std::fmt::{Display, Error, Formatter}; // Okay -- technically, this makes displaying Equations
                                           // directly work... but surely there's a smarter way of
                                           // approaching this!
use std::cmp::max;

/// An **equation** is an expression of the form `y = f(x)`, where `f(x)` is a function
/// that operates upon one or more **terms** (of the form `(coeff)x^(exp)`, where
/// `coeff`, `exp` => **R**) using one or more **mathematical operations** (i.e.,
/// `+`, `-`, `*`, `/`). `f(x)` in this case can consist either of a single term (i.e., `y = 2x^2`),
/// or of multiple terms operated upon each other (i.e., `y = 3x^3 - 2x^2 + x`); thus,
/// *all terms are equations*, but ***not** all equations are terms.*
/// #### Variants
/// * `Add(Box<Equation>, Box<Equation>)`: Represents the *sum* of two terms or equations (i.e., `4x^2 [+] 9x^3`).
/// * `Subtract(Box<Equation>, Box<Equation>)`: Represents the *difference* of two terms or equations (i.e., 
/// `4x^2 [-] 9x^3`).
/// * `Multiply(Box<Equation>, Box<Equation>)`: Represents the *product* of two terms or equations (i.e., `4x^2
/// [*] 9x^3`).
/// * `Divide(Box<Equation>, Box<Equation>)`: Represents the *quotient* of two terms or equations (i.e., `4x^2 [/]
/// 9x^3`).
/// * `Term(f32, f32)`: Represents a single term within a (possibly) larger equation.
/// A term's `f32` values represent the term's *coefficient* and *exponent* respectively;
/// that is, a term is of the form `(coeff)x^(exp)`.
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

// TODO: Maybe implement this for Box<Equation> instead, since that indirection is
// necessarily needed for this recursive type?
impl Equation {
    /// Generates an Equation that randomly unifies two other Equations / Terms
    /// in one of four ways: `lhs + rhs`, `lhs - rhs`, `lhs * rhs` or `lhs / rhs`.
    /// #### Parameters
    /// * `lhs: Box<Equation>`: The equation on the *left-hand side* of the randomly-
    /// generated operation.
    /// * `rhs: Box<Equation>`: The equation on the *right-hand side* of the randomly-
    /// generated equation.
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
    /// Equation. For example, if an equation "extends" once from its left-side,
    /// it may look something like `(2x / 3x^2) + 4x^3` -- an Equation inside of another
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
            Equation::Add(lhs, rhs) => return format!("{}{}{}", lhs.prnt_eqn(), " + ", rhs.prnt_eqn()),
            Equation::Subtract(lhs, rhs) => return format!("{}{}{}", lhs.prnt_eqn(), " - ", rhs.prnt_eqn()),
            Equation::Multiply(lhs, rhs) => return format!("{}{}{}", lhs.prnt_eqn(), " * ", rhs.prnt_eqn()),
            Equation::Divide(lhs, rhs) => return format!("{}{}{}", lhs.prnt_eqn(), " / ", rhs.prnt_eqn()),
            Equation::Term(coeff, exp) =>
            if *coeff == 0.0 {
                return "0".to_string();
            } else if *exp == 0.0 {
                return format!{"{}", *coeff};
            } else {
                return format!("{}x{}", *coeff, if *exp == 1.0 {String::new()} else {format!("^{}", *exp)});
            },
        }
    }

    /// Evaluates `y = self` when `x = x_val`.
    /// #### Parameters
    /// * `x_val: f32`: The x-value from which this Equation should be evaluated.
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

    /// Completely replaces this Equation in-place.
    /// #### Parameters
    /// * `replacement: Box<Equation>`: The equation to replace this one.
    fn replace_with(self, replacement: Box<Equation>) {
        unimplemented!();
        //self = replacement.clone();
    }

    fn lhs_rhs(&self) -> Option<(&Box<Equation>, &Box<Equation>)> {
        match self {
            Equation::Add(lhs, rhs) | Equation::Subtract(lhs, rhs)
            | Equation::Multiply(lhs, rhs) | Equation::Divide(lhs, rhs)
            => Some((lhs, rhs)),
            _ => None,
        }
    }

    /// Determines the maximum depth of this Equation.
    /// **NOT YET IMPLEMENTED; DO NOT USE!**
    fn depth (&self, current_depth: u8) -> u8 {
        match self {
            Equation::Add(lhs, rhs) |
            Equation::Subtract(lhs, rhs) |
            Equation::Multiply(lhs, rhs) |
            Equation::Divide(lhs, rhs) => lhs.depth(current_depth + 1).max(rhs.depth(current_depth + 1)),
            Equation::Term(_, _) => current_depth + 1,
        }
    }

    /// Simplifies this Equation in-place by combining all Terms with like
    /// coefficients. **NOT YET IMPLEMENTED; DO NOT USE!**
    fn simplify(self) -> Box<Equation> {
        Box::new(self);
        unimplemented!();
    }

    /// Performs a crossover between `self` and `other_eqn: Box<Equation>`.
    /// NOT YET IMPLEMENTED; DO NOT USE!
    fn crossover(eqn: &Box<Equation>, other_eqn: &Box<Equation>) -> Box<Equation> {
        todo!("Come back again later!");    
    }
}

/// Prints an Equation in a format that more directly showcases the enumerators
/// comprising the overall Equation, and the values used for its Terms.
impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { // TODO: What the hell is a Formatter?
                                                                // What the hell does <'_> mean?? Investigate!

        match self {
            Equation::Add(lhs, rhs) => write!(f, "[Add({}, {})]", *lhs, *rhs),
            Equation::Subtract(lhs, rhs) => write!(f, "[Subtract({}, {})]", *lhs, *rhs),
            Equation::Multiply(lhs, rhs) => write!(f, "[Multiply({}, {})]", *lhs, *rhs),
            Equation::Divide(lhs, rhs) => write!(f, "[Divide({}, {})]", *lhs, *rhs),
            Equation::Term(coeff, exp) => write!(f, "[Term({}, {})]", coeff, exp),
        }
    } 
}

// TODO: Once this function has been successfully implemented, make it work
// with a CSV!

/// Performs symbolic regression in Rust using genetic programming.
pub fn symbolic_regression_genetic( points: &[(f32, f32)], // Change this at some point!!
                                    pop_size: u8, 
                                    mutation_chance: f32, 
                                    generations: u8) -> Box<Equation> {
    assert!(generations > 0 && (mutation_chance > 0.0 && mutation_chance <= 100.0) && pop_size > 0);

    todo!("Nope! Not done yet! Come back again later!");
    //let mut population: [Box<Equation>; 10] = [Equation::random(5.0, 50.0); 10];
}

fn main() {
    let sample_points: [(f32, f32); 10] = [ (30.0, 6.0), (6.0, 15.0),
                                            (16.0, 30.0), (2.0, 7.0),
                                            (22.0, 3.0), (23.0, 16.0),
                                            (27.0, 3.0), (19.0, 26.0),
                                            (26.0, 29.0), (29.0, 25.0)];
    let test_eqn_1: Box<Equation> = Equation::random(10.0f32, 50.0f32);
    /*let test_eqn_2: Box<Equation> = Equation::random(10.0f32, 50.0f32);
    println!("Equation 1: {}\nEquation 2: {}\nCrossover: {}\n", test_eqn_1.prnt_eqn(), test_eqn_2.prnt_eqn(), Equation::crossover(&test_eqn_1, &test_eqn_2).prnt_eqn());
    println!("...And just to make sure we still own everything, let's look at those original equations one more time!");
    println!("Equation 1 (again): {}\nEquation 2 (again): {}", test_eqn_1.prnt_eqn(), test_eqn_2.prnt_eqn());*/
    println!("{}", test_eqn_1); // Tests the Display trait for Equation.
    //println!("{}", test_eqn_1.prnt_eqn()); // Tests the .prnt_eqn() function for Equation.
    let (lhs, rhs) = if let Some(lh_rh) = test_eqn_1.lhs_rhs() {
        lh_rh
    } else {
        (&Equation::empty(), &Equation::empty())
    };
    let lhs_depth = lhs.depth(0);
    let rhs_depth = rhs.depth(0);
    println!("The LHS and RHS depth of this equation are {} and {} respectively.", lhs_depth, rhs_depth);
}