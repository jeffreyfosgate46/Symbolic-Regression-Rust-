// OLD SOLUTIONS - SYMBOLIC REGRESSION IN RUST
// By Jeffrey Fosgate
// Last updated: September 23, 2026

// Below are a bunch of commented-out failed implementations of things that I felt
// were too elaborate to simply discard. I'll try to include additional commentary
// here wherever it's really needed, but for the most part, I explain myself in this
// project's text file, in the same directory as this one. Viewer beware; you're in for a scare!

// === OLD IMPLEMENTATIONS BELOW ===

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