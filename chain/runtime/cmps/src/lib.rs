//! CMPS: Cognitive Multi-Proof System (stub)
//! Layers: PoC², PoCog, PoSyn, PoAd, PoInt.

#[derive(Clone, Copy, Debug)]
pub struct Scores {
    pub continuity: f64,
    pub cognition:  f64,
    pub synergy:    f64,
    pub adaptation: f64,
    pub integrity:  f64,
}

/// Weighted composite score.
/// `w = (w0..w4)` corresponds to (PoC², PoCog, PoSyn, PoAd, PoInt).
pub fn composite(s: &Scores, w: (f64, f64, f64, f64, f64)) -> f64 {
    s.continuity * w.0
        + s.cognition * w.1
        + s.synergy * w.2
        + s.adaptation * w.3
        + s.integrity * w.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_in_range_and_print() {
        let s = Scores {
            continuity: 0.9,
            cognition:  0.8,
            synergy:    0.7,
            adaptation: 0.6,
            integrity:  0.95,
        };
        let w = (0.25, 0.30, 0.20, 0.15, 0.10);
        let v = composite(&s, w);

        println!("🧠 ThinkOS CMPS composite score (unit test) = {:.6}", v);

        assert!(
            v.is_finite(),
            "Composite not finite: {v}"
        );
        assert!(
            v >= 0.0 && v <= 1.1,
            "Composite out of expected range: {v}"
        );
    }
}
