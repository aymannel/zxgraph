use fraction::Fraction;

#[derive(Debug, Clone)]
pub struct Phase {
    angle: Fraction,
}

impl Phase {
    pub fn zero() -> Self {
        Phase::from(0.0)
    }

    pub fn one() -> Self {
        Phase::from(1.0)
    }

    pub fn plus() -> Self {
        Phase::from(0.5)
    }

    pub fn minus() -> Self {
        Phase::from(-0.5)
    }

    pub fn from(angle: f64) -> Self {
        let mut frac = Fraction::from(angle % 2.0);
        if frac < Fraction::from(0) {
            frac += Fraction::from(2)
        }
        Phase { angle: frac }
    }

    pub fn to_latex(&self) -> String {
        match (self.angle.numer().unwrap(), self.angle.denom().unwrap()) {
            (0, 1) => String::from(""),
            (1, 1) => String::from("$\\pi$"),
            (1, d) => format!("$\\frac{{\\pi}}{{{}}}$", d),
            (n, d) => format!("$\\frac{{{}\\pi}}{{{}}}$", n, d),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.angle == Fraction::from(0)
    }

    pub fn angle(&self) -> Fraction {
        self.angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! frac {
        ($num:expr, $den:expr) => {
            Fraction::from($num) / Fraction::from($den)
        };
    }

    #[test]
    fn test_phase_from_zero() {
        let phase = Phase::from(0.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    #[test]
    fn test_phase_from_positive_angle_under_two_pi() {
        let phase = Phase::from(1.25);
        assert_eq!(phase.angle(), Fraction::from(1.25 % 2.0));
    }

    #[test]
    fn test_phase_wraps_around_two_pi() {
        let phase = Phase::from(4.5);
        assert_eq!(phase.angle(), Fraction::from(0.5));
    }

    #[test]
    fn test_phase_from_negative_angle() {
        let phase = Phase::from(-0.5);
        assert_eq!(phase.angle(), Fraction::from(1.5));
    }

    #[test]
    fn test_phase_from_exactly_two_pi_wraps_to_zero() {
        let phase = Phase::from(2.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    #[test]
    fn test_phase_from_multiple_of_two_pi_wraps_to_zero() {
        let phase = Phase::from(6.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    // Phase::is_zero()

    #[test]
    fn test_is_zero_true_for_zero() {
        let phase = Phase::from(0.0);
        assert!(phase.is_zero());
    }

    #[test]
    fn test_is_zero_false_for_nonzero() {
        let phase = Phase::from(1.0);
        assert!(!phase.is_zero());
    }

    // Phase::to_latex()

    #[test]
    fn test_to_latex_zero() {
        let phase = Phase { angle: Fraction::from(0) };
        assert_eq!(phase.to_latex(), "");
    }

    #[test]
    fn test_to_latex_pi() {
        let phase = Phase { angle: Fraction::from(1) };
        assert_eq!(phase.to_latex(), "$\\pi$");
    }

    #[test]
    fn test_to_latex_half_pi() {
        let phase = Phase { angle: frac!(1, 2) };
        assert_eq!(phase.to_latex(), "$\\frac{\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_three_halves_pi() {
        let phase = Phase { angle: frac!(3, 2) };
        assert_eq!(phase.to_latex(), "$\\frac{3\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_negative_half_pi() {
        let phase = Phase { angle: frac!(-1, 2) };
        assert_eq!(phase.to_latex(), "$\\frac{\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_fraction_with_large_denominator() {
        let phase = Phase { angle: frac!(1, 8) };
        assert_eq!(phase.to_latex(), "$\\frac{\\pi}{8}$");
    }
}
