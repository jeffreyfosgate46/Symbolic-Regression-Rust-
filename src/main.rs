// SYMBOLIC REGRESSION GENETIC ALGORITHM IN RUST
// By Jeffrey Fosgate
// Last updated September 29, 2026
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
use std::mem::MaybeUninit; // TODO: What EXACTLY is "MaybeUninit"? Sources say it's analogous to the Option enum, but how?
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

#[derive(Clone)]
enum Equation {
    Add(Box<Equation>, Box<Equation>),
    Subtract(Box<Equation>, Box<Equation>),
    Multiply(Box<Equation>, Box<Equation>),
    Divide(Box<Equation>, Box<Equation>),
    Term(f64, i128),
    //Sine(Box<Equation>),
    //Cosine(Box<Equation>),
    //Tangent(Box<Equation>),
}

/// Returns a `bool` that has a 50% chance of being `true`, and a 50% chance of being `false`.
fn fifty_fifty_chance() -> bool {
    random_bool(0.5)
}

impl Equation {
    // === # FUNCTIONS FOR INITIALIZING NEW EQUATIONS # ===

    /// Produces a single-term Equation of the form (0.0)x^(0.0). Mostly used for debugging, as this is very unlikely to be produced organically.
    fn zero_term() -> Equation {
        Equation::Term(0.0, 0)
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
    fn random(val_rnge: f64, extend_chnc: f32) -> Box<Equation> {
        assert!(extend_chnc > 0.0 && extend_chnc <= 100.0);

        if random_bool((extend_chnc / 100.0) as f64) {

            let (random_lhs, random_rhs) = 
                                            (
                                                Equation::random(val_rnge, extend_chnc / 2.0),
                                                Equation::random(val_rnge, extend_chnc / 2.0)
                                            );

            Equation::unify_with_op(
                &random_lhs,
                &random_rhs,
                match random_range(0u8..=3u8) {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    3 => '/',
                    _ => '?',
                }
            )
        } else {
            Equation::random_term(val_rnge)
        }
    }

    /// Unifies two equations with a specific mathematical operation (`lhs + rhs`, `lhs - rhs`,
    /// `lhs * rhs` or `lhs / rhs`).
    /// #### Parameters
    /// * `lhs: Box<Equation>`: The equation on the *left-hand side* of the resulting equation.
    /// * `rhs: Box<Equation>`: The equation on the *right-hand side* of the resulting equation.
    /// * `op: char`: The operation with which to unify `lhs` and `rhs`. Invalid operations will return `(0.0)x^(0.0)` (equivalent to `Equation::zero_term()`).
    /// Valid operations are:
    ///     * `op = '+'`: Returns `lhs + rhs`.
    ///     * `op = '-'`: Returns `lhs - rhs`.
    ///     * `op = '*'`: Returns `lhs * rhs`.
    ///     * `op = '/'`: Returns `lhs / rhs`.
    fn unify_with_op(lhs: &Box<Equation>, rhs: &Box<Equation>, op: char) -> Box<Equation> {
        let (lhs, rhs) = (lhs.clone(), rhs.clone());
        Box::new(match op {
            '+' => Equation::Add(lhs, rhs),
            '-' => Equation::Subtract(lhs, rhs),
            '*' => Equation::Multiply(lhs, rhs),
            '/' => Equation::Divide(lhs, rhs),
            _ => Equation::zero_term(),
        })
    }

    /// Generates an expression of the form `(coeff)x^(exp)`.
    /// #### Parameters
    /// * `val_rnge: f32`: Coefficients and exponents randomly generated within 
    /// the equation can only possess values between `-(val_rnge)` and `val_rnge` (inclusive).
    fn random_term(val_rnge: f64) -> Box<Equation> {
        Box::new(Equation::Term(random_range(-(val_rnge)..=val_rnge), random_range(0.0..=val_rnge).round() as i128))
    }

    /// Returns a String representing the Equation.
    fn prnt_eqn_enums(&self) -> String {
        match self {
            Equation::Add(lhs, rhs) => format!("[Add({}, {})]", lhs.prnt_eqn_enums(), rhs.prnt_eqn_enums()),
            Equation::Subtract(lhs, rhs) => format!("[Subtract({}, {})]", lhs.prnt_eqn_enums(), rhs.prnt_eqn_enums()),
            Equation::Multiply(lhs, rhs) => format!("[Multiply({}, {})]", lhs.prnt_eqn_enums(), rhs.prnt_eqn_enums()),
            Equation::Divide(lhs, rhs) => format!("[Divide({}, {})]", lhs.prnt_eqn_enums(), rhs.prnt_eqn_enums()),
            Equation::Term(coeff, exp) => format!("[Term({}, {})]", coeff, exp),
        }
    }

    /// Evaluates `y = self` when `x = x_val`. Returns `Option::None` if the Equation cannot be evaluated at the given `x_val`; returns
    /// `Option::Some()` enclosing the successful equation evaluation at `x = x_val` otherwise.
    /// #### Parameters
    /// * `x_val: f32`: The x-value from which this Equation should be evaluated.
    fn eval_eqn(&self, x_val: f64) -> Option<f64> {
        let evaluation: f64 = match self {
            Equation::Add(lhs, rhs) => lhs.eval_eqn(x_val)? + lhs.eval_eqn(x_val)?,
            Equation::Subtract(lhs, rhs) => lhs.eval_eqn(x_val)? - lhs.eval_eqn(x_val)?,
            Equation::Multiply(lhs, rhs) => lhs.eval_eqn(x_val)? * lhs.eval_eqn(x_val)?,
            Equation::Divide(lhs, rhs) => {
                if rhs.eval_eqn(x_val) == Some(0.0f64) { // Cannot divide by zero!
                    println!("We got a NaN! (Attempted to divide by zero)");
                    f64::NAN
                } else {
                    lhs.eval_eqn(x_val)? / rhs.eval_eqn(x_val)?
                }
            },
            Equation::Term(coeff, exp) => *coeff * x_val.powf(*exp as f64),
        };

        if evaluation.is_nan() {
            println!("We got a NaN! (misc. / likely evaluating an equation with an undefined y-value @ x = {})", x_val);
            None
        } else {
            Some(evaluation)
        }
    }

    /// Separates a non-term equation into its left-hand side and right-hand side. Returns `None` for any `Equation::Term()`.
    fn lhs_rhs(&mut self) -> Option<(&mut Box<Equation>, &mut Box<Equation>)> {
        match self {
            Equation::Add(lhs, rhs) | Equation::Subtract(lhs, rhs)
            | Equation::Multiply(lhs, rhs) | Equation::Divide(lhs, rhs)
            => Some((lhs, rhs)),
            _ => None,
        }
    }

    /// Determines the maximum depth of this Equation - that is, the greatest number of equations nested within this one.
    /// #### Parameters
    /// `current_depth: u8`: The depth count that this function should start at. For most situations, it is advisable to let `current_depth = 0u8`.
    fn depth (&self, current_depth: u8) -> u8 {
        match self {
            Equation::Add(lhs, rhs) |
            Equation::Subtract(lhs, rhs) |
            Equation::Multiply(lhs, rhs) |
            Equation::Divide(lhs, rhs) => lhs.depth(current_depth + 1).max(rhs.depth(current_depth + 1)),
            Equation::Term(_, _) => current_depth + 1,
        }
    }

    // TODO: Wouldn't it be cool if this returned the equation you got rid of during the replacement?
    /// Replaces the equation passed as `replaced` with the equation
    fn replace(replaced: &mut Box<Equation>, replacement: &Box<Equation>) {
        *replaced = replacement.clone();
    }

    /// A function for swapping two Equations in-place, similarly to how replace() does it? Not sure if I even need this.
    /*fn swap() {

    }*/

    /// Simplifies this Equation in-place by combining all Terms with like
    /// coefficients. **NOT YET IMPLEMENTED; DO NOT USE!**
    fn simplify(self) -> Box<Equation> {
        unimplemented!();
    }

    // === PRIMARY GENETIC ALGORITHM METHODS ===

    /// Produces a *crossover* equation between `eqn` and `other_eqn` thusly:
    /// * If *both* `eqn` *and* `other_eqn` are simple terms (of the form `(coeff)x^(exp)`, with no additional operations), the crossover will be a term 
    /// that uses the coefficient from one equation and the exponent from another. For instance, a crossover of `3x^2` and `4x^5` could be `3x^5`, or `4x^2`.
    /// * If *one, but not both,* of the equations is a simple term, then a random side of the non-term equation will be completely replaced with the term equation to produce
    /// the crossover. For instance, a crossover of `3x^2` and `4x^5 + (6x^7 - 2x)` could be `3x^2 + (6x^7 - 2x)` or `4x^5 + 3x^2`.
    /// * If *neither* `eqn` *nor* `other_eqn` are simple terms, then a random side of one of the equations will be completely replaced with a random side of the
    /// other equation to produce the crossover. For instance, valid crossovers of `3x + (4x^2 + 6)` and `(7x^8 - 9) - 6x^5` include `(7x^8 - 9) + (4x^2 + 6)`, 
    /// `3x - 6x^5`, and `3x + (7x^8 - 9)`, among others.
    /// *** THIS METHOD IS STILL BUGGY; DO NOT USE! ***
    fn crossover(eqn: &Box<Equation>, other_eqn: &Box<Equation>) -> Box<Equation> {
        //todo!("crossover() is still fraught with errors and must be fixed before being used.");
        let (mut eqn, mut other_eqn) = (eqn.clone(), other_eqn.clone());
        
        let is_eqn_a_term = if let Some(_) = eqn.lhs_rhs() {
            false
        } else {
            true
        };

        let is_other_eqn_a_term = if let Some(_) = other_eqn.lhs_rhs() {
            false
        } else {
            true
        };

        if is_eqn_a_term && is_other_eqn_a_term { // The case where BOTH equations are simple terms.
                let Equation::Term(eqn_coeff, eqn_exp) = *eqn else {
                    panic!("Simple term expected at crossover; equation received.");
                };
                let Equation::Term(other_eqn_coeff, other_eqn_exp) = *other_eqn else {
                    panic!("Simple term expected at crossover; equation received.");
                };
                let use_other_coeff = fifty_fifty_chance();
                Box::new(Equation::Term(if use_other_coeff { other_eqn_coeff } else { eqn_coeff },
                                        if use_other_coeff { eqn_exp } else { other_eqn_exp }))
        } else { 
            let (mut crossover, mut other) = 
                                            if !(is_eqn_a_term || is_other_eqn_a_term) { // The case where BOTH equations are complex expressions.
                                                if fifty_fifty_chance() { (eqn, other_eqn) } else { (other_eqn, eqn) }
                                            }
                                            else if is_other_eqn_a_term { // The case where "other_eqn" ONLY is a simple term.
                                                (eqn, other_eqn)
                                            } else { // The case where "eqn" ONLY is a simple term.
                                                (other_eqn, eqn)
                                            };

            let Some((crossover_lhs, crossover_rhs)) = crossover.lhs_rhs() else {
                panic!("Expected complex expression during crossover; received simple term.");
            };

            let (other_lhs, other_rhs) = match other.lhs_rhs() {
                Some((found_lhs, found_rhs)) => (&*found_lhs, &*found_rhs),
                None => (&other, &other),
            };

            Equation::replace(
                        if fifty_fifty_chance() {
                            crossover_lhs
                        } else {
                            crossover_rhs
                        },
                        if fifty_fifty_chance() {
                            other_lhs
                        } else {
                            other_rhs
                        }
                    );
            crossover
        }
    }

    /// Mutates this Equation.   
    /// 
    /// A *mutation* can occur among a given equation in one of the following ways:
    /// * If the equation is a simple term (i.e., `3x^2`), a mutation occurs by altering the term's coefficient and / or exponent slightly (i.e., `3x^2` ->
    /// `(3.05)x^(1.95)`).
    /// * If the equation is a complex expression (i.e., `3x^2 + 5x^4`), a mutation occurs either by altering the coefficient and / or exponent of one of its
    /// constituent terms slightly (see above), or by replacing one of its sub-expressions with a completely new, random expression. For instance, valid
    /// mutations of `3x^2 + 5x^4` include `3x^2 + (4.95)x^4` (mutation of a constituent term) or `3x^2 + (7x^8 / 2x)` (replacement of a sub-expression with a
    /// completely new expression).
    /// #### Parameters
    /// * `toplevel_mut_chance: f32`: If `self` is an expression of the form `<lhs> <op> <rhs>` (i.e., `3x^2 + 5x^4`), the likelihood that `lhs` or `rhs` themselves
    /// will be replaced with a completely random new expression or term. This is an especially dramatic mutation to make; it is best to keep this small for
    /// more complex expressions.
    /// * `term_mut_intensity: f32`: How dramatically a simple term's coefficient or exponent should be raised or lowered *at most*, if it is mutated. For instance,
    /// if `term_mut_intensity = 2.0f32`, and a term has a coefficient of `2` (as in, `2x^3`), then that coefficient can only possibly possess values between 
    /// `0.0` and `4.0` after it has been mutated.
    fn mutate(&mut self, toplevel_mut_chance: f32, term_mut_intensity: f64) {    
        assert!(toplevel_mut_chance >= 0.0 && toplevel_mut_chance <= 100.0);
        if let Some((lhs, rhs)) = self.lhs_rhs() {
            if random_range(0.0..=100.0) > toplevel_mut_chance {
                if fifty_fifty_chance() {
                    *lhs = Equation::random(COEFF_EXP_RNGE, EQN_EXTENSION_CHNC);
                } else {
                    *rhs = Equation::random(COEFF_EXP_RNGE, EQN_EXTENSION_CHNC);
                }
            } else {
                if fifty_fifty_chance() {
                    lhs.mutate  (
                                toplevel_mut_chance + ((100.0 - toplevel_mut_chance) / (lhs.depth(0) as f32)),
                                term_mut_intensity
                                );
                } else {
                    rhs.mutate  
                    (
                    toplevel_mut_chance + ((100.0 - toplevel_mut_chance) / (rhs.depth(0) as f32)),
                    term_mut_intensity
                    );
                };
            }
        } else {
            let Equation::Term(coeff, exp) = self else {
                panic!("Simple term expected during mutation; complex equation provided.")
            };

            let (mut mutate_coeff, mut mutate_exp): (bool, bool);
            loop {
                (mutate_coeff, mutate_exp) = (fifty_fifty_chance(), fifty_fifty_chance());
                if mutate_coeff || mutate_exp {
                    break;
                }
            }

            if mutate_coeff {
                *coeff += random_range(-(term_mut_intensity)..=(term_mut_intensity));
            } if mutate_exp {
                *exp += random_range(-(term_mut_intensity)..=(term_mut_intensity)) as i128;
            }
        }
    }

    /// Retrieves the *fitness* of this equation with respect to a set of 2D coordinates, where *fitness*
    /// denotes the closeness of an equation's overall output to the coordinates within the set.
    fn fitness(&self, points: PointSet) -> f64 {
        assert!(points.len() > 0);
        let delta_limit = 10.0f64 * points.len() as f64;
        let mut total_delta: f64 = 0.0f64;

        for point in points {
            total_delta += if let Some(evaluation) = self.eval_eqn(point.0) {
                (evaluation - point.1).abs()
            } else {
                f64::MAX
            };
        }

        if total_delta == 0.0 { 
            delta_limit
        } else if total_delta > delta_limit {
            0.0
        } else {
            delta_limit / total_delta
        }
    }

}

/// Prints an Equation in a format that more directly showcases the enumerators
/// comprising the overall Equation, and the values used for its Terms.
impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { // TODO: What the hell is a Formatter?
                                                                // What the hell does <'_> mean?? Investigate!
        let (l_paren, r_paren): (char, char) = match self.depth(0u8) {
            shallow if shallow <= 1u8 => ('[', ']'),
            2u8 => ('{', '}'),
            3u8 => ('(', ')'),
            deep if deep > 3u8 => ('<', '>'),
            _ => ('?', '?'),
        };

        match self {
            Equation::Add(lhs, rhs) => write!(f, "{}{} + {}{}", l_paren, lhs, rhs, r_paren),
            Equation::Subtract(lhs, rhs) => write!(f, "{}{} - {}{}", l_paren, lhs, rhs, r_paren),
            Equation::Multiply(lhs, rhs) => write!(f, "{}{} * {}{}", l_paren, lhs, rhs, r_paren),
            Equation::Divide(lhs, rhs) => write!(f, "{}{} / {}{}", l_paren, lhs, rhs, r_paren),
            Equation::Term(coeff, exp) =>
            if *coeff == 0.0 {
                write!(f, "0")
            } else if *exp == 0 {
                write!{f, "{}", *coeff}
            } else {
                write!(f, "{}x{}", *coeff, if *exp == 1 {String::new()} else {format!("^{}", *exp)})
            },
        }
    } 
}

// TODO: Once this function has been successfully implemented, make it work with a CSV!

/// Performs symbolic regression in Rust using genetic programming.
/// 
/// 
pub fn symbolic_regression_genetic( points: PointSet,
                                    pop_size: u8, 
                                    mutation_chance: f32, 
                                    generations: u8) -> Box<Equation> {
    assert!(generations > 0 && (mutation_chance > 0.0 && mutation_chance <= 100.0) && pop_size > 0);

    todo!("Nope! Not done yet! Come back again later!");
    //let mut population: [Box<Equation>; 10] = [Equation::random(5.0, 50.0); 10];
}

// All of these constants will be parameterized once a basic symbolic regression loop has been designed and perfected.
const NUM_OF_TEST_EQS: u8 = 3;  // How many equations should be used for testing random eq. generation,
                                // equation displaying, etc.?

const COEFF_EXP_RNGE: f64 = 10.0;       // The test term range to use for mutate(), Equation::random(), etc.
const EQN_EXTENSION_CHNC: f32 = 30.0;   // The test extension chance to use for crossover().

const NUM_OF_TEST_POINTS: u8 = 5;  // The number of 2D points to generate for tessting fitness(), etc.
const POINT_COORD_RANGE: f64 = 10.0;// The range (along both the x- and y-axis, positive and negative)
                                    // within which 2D test points can be plotted.

type Point = (f64, f64);
type PointSet = Vec<Point>;
fn main() {

    // ###### EQUATION FUNCTION TESTS BELOW -- UNCOMMENT WHEN NECESSARY ######
    // === TEST VALUES ===
    let mut test_eqns: Vec<Box<Equation>> = Vec::<Box<Equation>>::new();
    for _test_eqn in 0..NUM_OF_TEST_EQS {
        test_eqns.push(Equation::random(COEFF_EXP_RNGE, EQN_EXTENSION_CHNC));
    }

    let mut point_set: PointSet = PointSet::new(); // Okay... that's pretty cool.
    let mut points_generated = 0;
    loop {
        let new_point: Point = (random_range(-POINT_COORD_RANGE..=POINT_COORD_RANGE).round(), random_range(-POINT_COORD_RANGE..=POINT_COORD_RANGE).round());
        
        if point_set.is_empty() {
            point_set.push(new_point);
            points_generated += 1;
        } else {
            for point_idx in 0..=point_set.len()-1 {
                if point_set[point_idx].1 == new_point.1 {
                    continue;
                }

                if point_set[point_idx] == point_set[point_set.len() - 1] {
                    point_set.push(new_point);
                    points_generated += 1;
                }
            }
        }
        /*if let None = point_set.iter().find(|pt| pt.1 == new_point.1) { // This uses a closure - something you only
            point_set.push(new_point);                                                // know about because of Rust docs. Investigate!
            points_generated += 1;
        }*/
        if points_generated == NUM_OF_TEST_POINTS {
            break;
        }
    }

    println!("Here are our test points:");
    for point_idx in 0..=point_set.len()-1 {
        println!("{:?}", point_set[point_idx]);
    }

    // === PRINTING EQUATIONS ===
    for eqn_idx in 0..test_eqns.len() {
        println!("This is Equation {}: {}", eqn_idx + 1, test_eqns[eqn_idx]);
        println!("And here is Equation {} as a bunch of enums: {}", eqn_idx + 1, test_eqns[eqn_idx].prnt_eqn_enums());
    }

    // === SWAPPING AND REPLACING EQUATIONS ===
    /*let new_random_eqn = Equation::random(COEFF_EXP_RNGE, EQN_EXTENSION_CHNC);
    println!("Here's a new equation I just came up with! {}", new_random_eqn);
    println!("Now, I'll turn two equations into one before your very eyes! Remember this equation?\n{}", test_eqns[0]);
    println!("Here what our \'new equation\' looks like now! Ta-daaaa!\n{}", new_random_eqn);*/

    // === CROSSOVER TEST === (DO NOT UN-COMMENT UNTIL BUGS ARE FIXED)
    println!("Here's an example of a crossover between Equation 1 and Equation 2: {}", Equation::crossover(&test_eqns[0], &test_eqns[1]));

    // === MUTATION TEST ===
    let last_idx: usize = test_eqns.len() - 1;
    println!("Now, here's Equation {} again:\n{}", last_idx, test_eqns[last_idx]);
    test_eqns[last_idx].mutate(20.0, 30.0);
    println!("And here's Equation {} after I tweaked with it a little!\n{}", last_idx, test_eqns[last_idx]);

    // === FITNESS TEST ===
    println!("This is Equation 1\n{}", test_eqns[0]);
    println!("And this is its fitness: {}", test_eqns[0].fitness(point_set));
    
}