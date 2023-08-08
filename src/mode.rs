use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    RNA,
    DNA,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rna" => Ok(Mode::RNA),
            "dna" => Ok(Mode::DNA),
            _ => Err("Invalid mode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Mode::from_str("rna"), Ok(Mode::RNA));
        assert_eq!(Mode::from_str("dna"), Ok(Mode::DNA));
        assert_eq!(Mode::from_str("invalid"), Err("Invalid mode"));
    }
}