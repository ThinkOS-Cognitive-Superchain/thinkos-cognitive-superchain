/// CMPS: Cognitive Multi-Proof System (stub)
/// Layers: PoC², PoCog, PoSyn, PoAd, PoInt.

#[derive(Clone, Copy, Debug)]
pub struct Scores {
    pub continuity: f64,
    pub cognition:  f64,
    pub synergy:    f64,
    pub adaptation: f64,
    pub integrity:  f64,
}

/// Weighted composite score.
/// w = (w0..w4) corresponding to (PoC², PoCog, PoSyn, PoAd, PoInt)
pub fn composite(s: &Scores, w: (f64,f64,f64,f64,f64)) -> f64 {
    s.continuity*w.0 + s.cognition*w.1 + s.synergy*w.2 + s.adaptation*w.3 + s.integrity*w.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_works() {
        let s = Scores { continuity: 1.0, cognition: 1.0, synergy: 1.0, adaptation: 1.0, integrity: 1.0 };
        let w = (0.25, 0.30, 0.20, 0.15, 0.10);
        let v = composite(&s, w);
        assert!((v - 1.0).abs() < 1e-12);
    }
}
