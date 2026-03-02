use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{FitBoth, FitError, FitOverflow, FitUnderflow, UpperBound};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod fit_error {
    use super::*;

    #[test]
    fn from_overflow() {
        let over = FitOverflow::unbounded(MAX_CAP);
        let err = FitError::from(over);
        assert_eq!(err, FitError::Overflow(FitOverflow::unbounded(MAX_CAP)));
    }

    #[test]
    fn from_underflow() {
        let under = FitUnderflow::new(UNDER_CAP, MIN_CAP);
        let err = FitError::from(under);
        assert_eq!(err, FitError::Underflow(FitUnderflow::new(UNDER_CAP, MIN_CAP)));
    }

    #[test]
    fn from_both() {
        let over = FitOverflow::unbounded(MAX_CAP);
        let under = FitUnderflow::new(UNDER_CAP, MIN_CAP);
        let both = FitBoth::new(over, under);
        let err = FitError::from(both);

        let expected_over = FitOverflow::unbounded(MAX_CAP);
        let expected_under = FitUnderflow::new(UNDER_CAP, MIN_CAP);
        let expected_both = FitBoth::new(expected_over, expected_under);
        assert_eq!(err, FitError::Both(expected_both));
    }
}

mod overflows {
    use super::*;

    check_eq!(unbounded: FitOverflow::unbounded(MAX_CAP).max_size() => UpperBound::Unbounded);
    check_eq!(fixed: FitOverflow::fixed(OVER_CAP, MAX_CAP).max_size() => UpperBound::Fixed(OVER_CAP));
    panics!(panic_fixed: FitOverflow::fixed(CAP, MAX_CAP) => "max_size must be > max_cap");
}

mod underflows {
    use super::*;

    check_eq!(new: FitUnderflow::new(UNDER_CAP, MIN_CAP).min_size() => UNDER_CAP);
    panics!(panic_new: FitUnderflow::new(CAP, MIN_CAP) => "min_size must be < min_cap");
}

mod both {
    use super::*;

    #[test]
    fn new_valid() {
        let over = FitOverflow::unbounded(MaxCapVal(10));
        let under = FitUnderflow::new(4, MinCapVal(5));
        let both = FitBoth::new(over, under);
        assert_eq!(both.overflow(), &FitOverflow::unbounded(MaxCapVal(10)));
        assert_eq!(both.underflow(), &FitUnderflow::new(4, MinCapVal(5)));
    }

    panics!(panic_new_invalid: {
        let over = FitOverflow::unbounded(MaxCapVal(5));
        let under = FitUnderflow::new(4, MinCapVal(10));
        let _ = FitBoth::new(over, under);
    } => "Invalid capacity constraint: min_cap must be <= max_cap");
}
