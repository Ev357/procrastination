use core::{
    fmt::{self, Display},
    ops::{Add, Mul},
};

#[derive(Debug, Default, Clone)]
pub struct SuperString {
    pub data: String,
}

impl SuperString {
    pub fn new(data: &str) -> Self {
        Self {
            data: String::from(data),
        }
    }
}

impl Display for SuperString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Add for SuperString {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let data = match (self.data.as_str(), other.data.as_str()) {
            ("" | "0", "" | "0") => String::from(""),
            ("" | "0", _) => other.data,
            (_, "" | "0") => self.data,
            (_, _) => format!("{} + {}", self.data, other.data),
        };

        Self { data }
    }
}

impl Mul for SuperString {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let data = match (self.data.as_str(), rhs.data.as_str()) {
            ("" | "0", _) | (_, "" | "0") => String::from(""),
            ("1", _) => rhs.data,
            (_, "1") => self.data,
            (_, _) => format!("{}{}", self.data, rhs.data),
        };

        Self { data }
    }
}
