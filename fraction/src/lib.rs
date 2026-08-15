pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

#[derive(Debug)]
pub enum FractionError {
    ZeroDenominator,
}

impl Fraction {
    pub fn init(numerator: i32, denominator: i32) -> Result<Self, FractionError> {
        if denominator == 0 {
            return Err(FractionError::ZeroDenominator);
        }
        let mut res = Self {
            numerator,
            denominator,
        };
        res.check_negatives();
        Ok(res)
    }

    pub fn add(&mut self, frac: &mut Fraction) {
        self.check_negatives();
        frac.check_negatives();
        let mut numerator;
        let denominator;
        let frac1_denominator = self.denominator;
        let frac2_denominator = frac.denominator;

        if frac1_denominator != frac2_denominator {
            denominator = frac1_denominator * frac2_denominator;
            numerator = self.numerator * frac2_denominator;
            numerator += frac.numerator * frac1_denominator;
        } else {
            denominator = frac1_denominator;
            numerator = self.numerator + frac.numerator;
        }
        self.numerator = numerator;
        self.denominator = denominator;
    }

    pub fn subtract(&mut self, frac: &mut Fraction) {
        frac.numerator *= -1;
        self.add(frac);
    }

    pub fn multiply(&mut self, frac: &mut Fraction) {
        self.check_negatives();
        frac.check_negatives();
        let numerator = self.numerator * frac.numerator;
        let denominator = self.denominator * frac.denominator;
        self.numerator = numerator;
        self.denominator = denominator;
    }

    pub fn divide(&mut self, frac: &mut Fraction) {
        frac.invert();
        self.multiply(frac);
    }

    pub fn invert(&mut self) {
        self.check_negatives();
        let temp = self.numerator;
        self.numerator = self.denominator;
        self.denominator = temp;
    }

    pub fn negate(&mut self) {
        self.check_negatives();
        self.numerator *= -1;
    }

    pub fn reduce(&mut self) {
        self.check_negatives();
        let mut numerator = self.numerator;
        let mut denominator = self.denominator;
        if numerator < 0 {
            numerator *= -1;
        }
        while denominator > 0 {
            let temp = numerator;
            numerator = denominator;
            denominator = temp % denominator;
        }
        self.numerator /= numerator;
        self.denominator /= numerator;
    }

    fn check_negatives(&mut self) {
        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut frac = Fraction::init(10, 25).unwrap();
        assert_eq!(10, frac.numerator);
        assert_eq!(25, frac.denominator);
        frac = Fraction::init(-10, 17).unwrap();
        assert_eq!(-10, frac.numerator);
        assert_eq!(17, frac.denominator);
        frac = Fraction::init(4, -16).unwrap();
        assert_eq!(-4, frac.numerator);
        assert_eq!(16, frac.denominator);
        frac = Fraction::init(-12, -19).unwrap();
        assert_eq!(12, frac.numerator);
        assert_eq!(19, frac.denominator);
        let result = Fraction::init(10, 0);
        assert!(matches!(result, Err(FractionError::ZeroDenominator)));
    }

    #[test]
    fn test_add() {
        let mut frac1 = Fraction::init(20, 38).unwrap();
        let mut frac2 = Fraction::init(3, 8).unwrap();
        frac1.add(&mut frac2);
        assert_eq!(274, frac1.numerator);
        assert_eq!(304, frac1.denominator);
        frac1 = Fraction::init(19, 27).unwrap();
        frac2 = Fraction::init(4, 27).unwrap();
        frac1.add(&mut frac2);
        assert_eq!(23, frac1.numerator);
        assert_eq!(27, frac1.denominator);
    }

    #[test]
    fn test_multiply() {
        let mut frac1 = Fraction::init(20, 38).unwrap();
        let mut frac2 = Fraction::init(3, 8).unwrap();
        frac1.multiply(&mut frac2);
        assert_eq!(60, frac1.numerator);
        assert_eq!(304, frac1.denominator);
    }

    #[test]
    fn test_reduce() {
        let mut frac = Fraction::init(1020, 390).unwrap();
        frac.reduce();
        assert_eq!(34, frac.numerator);
        assert_eq!(13, frac.denominator);
    }
}
