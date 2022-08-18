use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Knot {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SplineSegment {
    pub a: f64,     // S(x) = a * x.powi(3)
    pub b: f64,     //      + b * x.powi(2)
    pub c: f64,     //      + c * x
    pub d: f64,     //      + d
    pub knot0: f64, // with knot0 <= x
    pub knot1: f64, //                 <= knot1
}

impl SplineSegment {
    pub fn evaluate(&self, x: f64) -> f64 {
        assert!(x >= self.knot0, "{x} is outside the spline range.");
        assert!(x <= self.knot1, "{x} is outside the spline range.");

        self.a * x.powi(3) + self.b * x.powi(2) + self.c * x + self.d
    }
}
