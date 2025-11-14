use fraction::Fraction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase {
    angle: Fraction,
}

impl Phase {
    pub fn new(angle: f64) -> Self {
        let mut frac = Fraction::from(angle % 2.0);
        if frac < Fraction::from(0) {
            frac += Fraction::from(2)
        }
        Phase { angle: frac }
    }

    pub fn zero() -> Self {
        Phase::new(0.0)
    }

    pub fn one() -> Self {
        Phase::new(1.0)
    }

    pub fn plus() -> Self {
        Phase::new(0.5)
    }

    pub fn minus() -> Self {
        Phase::new(-0.5)
    }

    pub fn angle(&self) -> Fraction {
        self.angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::Exportable;

    macro_rules! frac {
        ($num:expr, $den:expr) => {
            Fraction::from($num) / Fraction::from($den)
        };
    }

    #[test]
    fn test_phase_from_zero() {
        let phase = Phase::new(0.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    #[test]
    fn test_phase_from_positive_angle_under_two_pi() {
        let phase = Phase::new(1.25);
        assert_eq!(phase.angle(), Fraction::from(1.25 % 2.0));
    }

    #[test]
    fn test_phase_wraps_around_two_pi() {
        let phase = Phase::new(4.5);
        assert_eq!(phase.angle(), Fraction::from(0.5));
    }

    #[test]
    fn test_phase_from_negative_angle() {
        let phase = Phase::new(-0.5);
        assert_eq!(phase.angle(), Fraction::from(1.5));
    }

    #[test]
    fn test_phase_from_exactly_two_pi_wraps_to_zero() {
        let phase = Phase::new(2.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    #[test]
    fn test_phase_from_multiple_of_two_pi_wraps_to_zero() {
        let phase = Phase::new(6.0);
        assert_eq!(phase.angle(), Fraction::from(0));
    }

    // Phase::to_latex()

    #[test]
    fn test_to_latex_zero() {
        let phase = Phase { angle: Fraction::from(0) };
        assert_eq!(phase.to_tex().unwrap(), "");
    }

    #[test]
    fn test_to_latex_pi() {
        let phase = Phase { angle: Fraction::from(1) };
        assert_eq!(phase.to_tex().unwrap(), "$\\pi$");
    }

    #[test]
    fn test_to_latex_half_pi() {
        let phase = Phase { angle: frac!(1, 2) };
        assert_eq!(phase.to_tex().unwrap(), "$\\frac{\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_three_halves_pi() {
        let phase = Phase { angle: frac!(3, 2) };
        assert_eq!(phase.to_tex().unwrap(), "$\\frac{3\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_negative_half_pi() {
        let phase = Phase { angle: frac!(-1, 2) };
        assert_eq!(phase.to_tex().unwrap(), "$\\frac{\\pi}{2}$");
    }

    #[test]
    fn test_to_latex_fraction_with_large_denominator() {
        let phase = Phase { angle: frac!(1, 8) };
        assert_eq!(phase.to_tex().unwrap(), "$\\frac{\\pi}{8}$");
    }
}
