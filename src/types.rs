/// Types whose valid values are enumerable.
pub trait EnumValues {
    /// Build a custom message representing valid values.
    fn valid_values(&self) -> String;
}

/// Represents a number (`usize`) selection, whose valid values are within a specific range.
pub struct RangeUsize {
    pub lo: usize,
    pub hi: usize,
}

impl RangeUsize {
    pub fn new(lo: usize, hi: usize) -> RangeUsize {
        RangeUsize { lo, hi }
    }
}

impl EnumValues for RangeUsize {
    fn valid_values(&self) -> String {
        format!(
            "[{}]",
            (self.lo..=self.hi)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Represents a confirmation message, with only 2 valid values: `confirm`, `cancel`.
pub struct Confirmation {
    confirm: &'static str,
    cancel: &'static str,
}

impl Confirmation {
    pub const fn yes_or_no() -> Confirmation {
        Confirmation {
            confirm: "yes",
            cancel: "no",
        }
    }
}

impl EnumValues for Confirmation {
    fn valid_values(&self) -> String {
        format!("[{}, {}]", self.confirm, self.cancel)
    }
}
