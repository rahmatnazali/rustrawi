use regex::Regex;

/// Confix Stripping Rule Precedence Adjustment Specification.
///
/// # Reference
/// - Asian J. (2007) “Effective Techniques for Indonesian Text Retrieval” page 78-79.
/// - http://researchbank.rmit.edu.au/eserv/rmit:6312/Asian.pdf
pub struct PrecedenceAdjustment {
    regex_list: Vec<Regex>
}

impl PrecedenceAdjustment {
    pub fn new() -> Self {
        Self {
            regex_list: vec![
                Regex::new(r"^be(.*)lah$").unwrap(),
                Regex::new(r"^be(.*)an$").unwrap(),
                Regex::new(r"^me(.*)i$").unwrap(),
                Regex::new(r"^di(.*)i$").unwrap(),
                Regex::new(r"^pe(.*)i$").unwrap(),
                Regex::new(r"^ter(.*)i$").unwrap(),
            ]
        }
    }

    pub fn is_satisfied_by(&self, word: &str) -> bool {
        for r in &self.regex_list {
            if r.is_match(word) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod precedence_adjustment_test {
    use super::*;

    #[test]
    fn should_not_satisfy() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("panjangkanlah"), false);
    }

    #[test]
    fn should_satisfy_1() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("benarkanlah"), true);
    }

    #[test]
    fn should_satisfy_2() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("benarkan"), true);
    }

    #[test]
    fn should_satisfy_3() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("memaklumi"), true);
    }

    #[test]
    fn should_satisfy_4() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("digurui"), true);
    }

    #[test]
    fn should_satisfy_5() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("pelajari"), true);
    }

    #[test]
    fn should_satisfy_6() {
        let pa = PrecedenceAdjustment::new();
        assert_eq!(pa.is_satisfied_by("terndoai"), true);
    }
}
