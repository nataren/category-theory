use std::fmt;
use std::str::FromStr;
use category_theory::id as id;

#[cfg(test)]
mod test {
    #[test]
    fn identity_test() {
        assert!(to_pair(id("1,0.4")) == to_pair("1,0.4"));
        assert!(id(to_pair("1,0.4")) == to_pair("1,0.4"));
    }

    struct Pair(i32, f32);

    impl fmt::Display for Pair {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Pair) -> bool {
            return (self.0 == other.0) && (self.1 == other.1);
        }
    }

    impl Eq for Pair {}

    fn to_pair(csv: &str) -> Option<Pair> {
        let mut parts = csv.split(',');
        let first = parts.nth(0);
        let second = parts.nth(1);
        if let Some(f) = first {
            if let Some(s) = second {
                let parsed_first = i32::from_str(f);
                let parsed_second = f32::from_str(s);
                if parsed_first.is_ok() && parsed_second.is_ok() {
                    Some(Pair(parsed_first.unwrap(), parsed_second.unwrap()));
                }
            }
        }
        None
    }
}
