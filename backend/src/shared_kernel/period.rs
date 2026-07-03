#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    start_timestamp: i64,
    end_timestamp: i64,
}

impl Period {
    pub fn new(start_timestamp: i64, end_timestamp: i64) -> Result<Self, &'static str> {
        if start_timestamp >= end_timestamp {
            return Err("Start timestamp must be strictly before end timestamp");
        }
        Ok(Self { start_timestamp, end_timestamp })
    }

    pub fn contains(&self, timestamp: i64) -> bool {
        timestamp >= self.start_timestamp && timestamp <= self.end_timestamp
    }

    pub fn overlaps_with(&self, other: &Period) -> bool {
        self.start_timestamp <= other.end_timestamp && self.end_timestamp >= other.start_timestamp
    }

    pub fn start(&self) -> i64 {
        self.start_timestamp
    }

    pub fn end(&self) -> i64 {
        self.end_timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_start_equal_to_end() {
        assert!(Period::new(100, 100).is_err());
    }

    #[test]
    fn new_rejects_start_after_end() {
        assert!(Period::new(200, 100).is_err());
    }

    #[test]
    fn contains_exact_boundaries() {
        let p = Period::new(100, 200).unwrap();
        assert!(p.contains(100));
        assert!(p.contains(200));
    }

    #[test]
    fn contains_outside_range() {
        let p = Period::new(100, 200).unwrap();
        assert!(!p.contains(99));
        assert!(!p.contains(201));
    }

    #[test]
    fn overlaps_with_partial() {
        let a = Period::new(100, 200).unwrap();
        let b = Period::new(150, 250).unwrap();
        assert!(a.overlaps_with(&b));
        assert!(b.overlaps_with(&a));
    }

    #[test]
    fn overlaps_with_containment() {
        let outer = Period::new(100, 300).unwrap();
        let inner = Period::new(150, 200).unwrap();
        assert!(outer.overlaps_with(&inner));
        assert!(inner.overlaps_with(&outer));
    }

    #[test]
    fn overlaps_with_adjacent_touching() {
        let a = Period::new(100, 200).unwrap();
        let b = Period::new(200, 300).unwrap();
        assert!(a.overlaps_with(&b)); // end == start → overlap
    }

    #[test]
    fn no_overlap_when_separate() {
        let a = Period::new(100, 200).unwrap();
        let b = Period::new(300, 400).unwrap();
        assert!(!a.overlaps_with(&b));
        assert!(!b.overlaps_with(&a));
    }
}
