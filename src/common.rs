use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Knot {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize)]
pub struct SplineSegment {
    pub a: f64,     // S(x) = a * x.powi(3)
    pub b: f64,     //      + b * x.powi(2)
    pub c: f64,     //      + c * x
    pub d: f64,     //      + d
    pub knot0: f64, // with knot0 <= x
    pub knot1: f64, //                 <= knot1
}
