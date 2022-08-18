use std::iter::zip;

use crate::{argparse::EvaluateConfig, common::SplineSegment};

pub fn run_evaluate(config: EvaluateConfig) {
    let spline: Vec<SplineSegment> = load_spline(&config.input_path);
    let results: Vec<f64> = evaluate(&spline, &config.values);

    for (x, y) in zip(config.values, results) {
        println!("S({}) = {}", x, y);
    }
}

pub fn evaluate(spline: &[SplineSegment], values: &[f64]) -> Vec<f64> {
    let mut results: Vec<f64> = vec![];

    for x in values {
        let i: usize = find_spline_segment(spline, *x);
        results.push(spline[i].evaluate(*x));
    }

    results
}

fn find_spline_segment(spline: &[SplineSegment], x: f64) -> usize {
    assert!(!spline.is_empty(), "You can't evaluate an empty spline.");

    let mut i: usize = 0;
    let mut j: usize = spline.len() - 1;

    assert!(x >= spline[0].knot0, "{x} is outside the spline range.");
    assert!(x <= spline[j].knot1, "{x} is outside the spline range.");

    while i < j {
        let mid: usize = ((i + j) / 2) as usize;

        if spline[mid].knot1 < x {
            i = mid + 1;
        } else if spline[mid].knot0 > x {
            j = mid - 1;
        } else {
            return mid;
        }
    }

    i
}

fn load_spline(input_path: &str) -> Vec<SplineSegment> {
    // Load knots from CSV
    let spline: Vec<SplineSegment> = csv::Reader::from_path(input_path)
        .expect("Failed to read input CSV.")
        .deserialize()
        .collect::<Result<Vec<SplineSegment>, _>>()
        .expect("Failed to parse input CSV.");

    // Ensure the knots are finite floats
    spline
        .iter()
        .all(|segment| {
            segment.a.is_finite()
                && segment.b.is_finite()
                && segment.c.is_finite()
                && segment.d.is_finite()
                && segment.knot0.is_finite()
                && segment.knot1.is_finite()
        })
        .then(|| ())
        .expect("Invalid floats in the input CSV.");

    // TODO: validate the spline is continuous and properly sorted

    spline
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_valid_spline() -> Vec<SplineSegment> {
        vec![
            SplineSegment {
                a: -13.907284768211797,
                b: 0.0,
                c: -0.5231788079470334,
                d: 10.0,
                knot0: 0.0,
                knot1: 0.5,
            },
            SplineSegment {
                a: 80.1324503311254,
                b: -141.0596026490058,
                c: 70.00662251655588,
                d: -1.7549668874171704,
                knot0: 0.5,
                knot1: 0.8,
            },
            SplineSegment {
                a: -85.43046357615866,
                b: 256.2913907284759,
                c: -247.8741721854296,
                d: 83.01324503311228,
                knot0: 0.8,
                knot1: 1.0,
            },
        ]
    }

    #[test]
    fn test_find_spline_segment_correct() {
        let spline: Vec<SplineSegment> = get_valid_spline();

        assert_eq!(find_spline_segment(&spline, 0.0), 0);
        assert_eq!(find_spline_segment(&spline, 0.1), 0);
        assert!(find_spline_segment(&spline, 0.5) == 0 || find_spline_segment(&spline, 0.5) == 1);
        assert_eq!(find_spline_segment(&spline, 0.6), 1);
        assert!(find_spline_segment(&spline, 0.8) == 1 || find_spline_segment(&spline, 0.8) == 2);
        assert_eq!(find_spline_segment(&spline, 0.9), 2);
        assert_eq!(find_spline_segment(&spline, 1.0), 2);
    }

    #[test]
    fn test_find_spline_segment_single() {
        let spline: Vec<SplineSegment> = vec![SplineSegment {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            knot0: 0.0,
            knot1: 1.0,
        }];

        assert_eq!(find_spline_segment(&spline, 0.0), 0);
        assert_eq!(find_spline_segment(&spline, 0.5), 0);
        assert_eq!(find_spline_segment(&spline, 1.0), 0);
    }

    #[test]
    #[should_panic]
    fn test_find_spline_segment_out_of_range_lower() {
        let spline: Vec<SplineSegment> = get_valid_spline();
        find_spline_segment(&spline, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_find_spline_segment_out_of_range_higher() {
        let spline: Vec<SplineSegment> = get_valid_spline();
        find_spline_segment(&spline, 100.0);
    }

    #[test]
    #[should_panic]
    fn test_find_spline_segment_empty_spline() {
        let spline: Vec<SplineSegment> = vec![];
        let value: f64 = 123.4;
        find_spline_segment(&spline, value);
    }

    #[test]
    fn test_evaluate_correct() {
        let spline: Vec<SplineSegment> = get_valid_spline();

        assert_eq!(evaluate(&spline, &[0.6]), vec![6.776158940397357]);

        let y: Vec<f64> = evaluate(&spline, &[0.0, 0.5, 0.8, 1.0]);

        assert!((y[0] - 10.0).abs() < 1e-8);
        assert!((y[1] - 8.0).abs() < 1e-8);
        assert!((y[2] - 5.0).abs() < 1e-8);
        assert!((y[3] - 6.0).abs() < 1e-8);
    }

    #[test]
    #[should_panic]
    fn test_evaluate_out_of_range_lower() {
        let spline: Vec<SplineSegment> = get_valid_spline();
        evaluate(&spline, &[-10.0]);
    }

    #[test]
    #[should_panic]
    fn test_evaluate_out_of_range_higherer() {
        let spline: Vec<SplineSegment> = get_valid_spline();
        evaluate(&spline, &[100.0]);
    }

    #[test]
    #[should_panic]
    fn test_evaluate_empty() {
        let spline: Vec<SplineSegment> = vec![];
        evaluate(&spline, &[-10.0]);
    }
}
