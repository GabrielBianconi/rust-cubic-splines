use csv;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};


const EQUATIONS_PER_SPLINE: usize = 4;
const PARAMETERS_PER_SPLINE: usize = 4;


#[derive(Debug, Deserialize)]
struct Knot {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize)]
struct Spline {
    a: f64,  // S(x) = a * x.powi(3)
    b: f64,  //      + b * x.powi(2)
    c: f64,  //      + c * x
    d: f64,  //      + d
    knot0: f64,  // with knot0 <= x
    knot1: f64,  //                 <= knot1
}

fn main() {
    let knots: Vec<Knot> = load_knots("./input.csv");
    
    // dbg!(&knots);

    let num_splines = knots.len() - 1;
    let nrows = num_splines * EQUATIONS_PER_SPLINE;
    let ncols = num_splines * PARAMETERS_PER_SPLINE;

    let mut a = DMatrix::<f64>::zeros(nrows, ncols);
    let mut b = DMatrix::<f64>::zeros(nrows, 1);

    for (i, spline) in knots.windows(2).enumerate() {
        let (knot0, knot1) = (&spline[0], &spline[1]);

        let row_offset = i * EQUATIONS_PER_SPLINE;
        let col_offset = i * PARAMETERS_PER_SPLINE;

        // knot0: a * x_0**3 + b * x_0**2 + c * x_0 + d = y_0
        a[(row_offset, col_offset + 0)] = knot0.x.powi(3);
        a[(row_offset, col_offset + 1)] = knot0.x.powi(2);
        a[(row_offset, col_offset + 2)] = knot0.x;
        a[(row_offset, col_offset + 3)] = 1.0;
        b[(row_offset, 0)] = knot0.y;

        // knot1: a * x_1**3 + b * x_1**2 + c * x_1 + d = y_1
        a[(row_offset + 1, col_offset + 0)] = knot1.x.powi(3);
        a[(row_offset + 1, col_offset + 1)] = knot1.x.powi(2);
        a[(row_offset + 1, col_offset + 2)] = knot1.x;
        a[(row_offset + 1, col_offset + 3)] = 1.0;
        b[(row_offset + 1, 0)] = knot1.y;

        // Check the first and second derivatives match the next spline
        if i < num_splines - 1 {
            let col_offset_next = col_offset + PARAMETERS_PER_SPLINE;

            // S_i'(x_{i+1}) = S_{i+1}'(x_{i+1})
            // => S_i'(x_{i+1}) - S_{i+1}'(x_{i+1}) = 0
            a[(row_offset + 2, col_offset + 0)] = 3.0 * knot1.x.powi(2);
            a[(row_offset + 2, col_offset + 1)] = 2.0 * knot1.x;
            a[(row_offset + 2, col_offset + 2)] = 1.0;

            a[(row_offset + 2, col_offset_next + 0)] = -3.0 * knot1.x.powi(2);
            a[(row_offset + 2, col_offset_next + 1)] = -2.0 * knot1.x;
            a[(row_offset + 2, col_offset_next + 2)] = -1.0;

            // S_i''(x_{i+1}) = S_{i+1}''(x_{i+1})
            // => S_i''(x_{i+1}) - S_{i+1}''(x_{i+1}) = 0
            a[(row_offset + 3, col_offset + 0)] = 6.0 * knot1.x;
            a[(row_offset + 3, col_offset + 1)] = 2.0;

            a[(row_offset + 3, col_offset_next + 0)] = -6.0 * knot1.x;
            a[(row_offset + 3, col_offset_next + 1)] = -2.0;
        }
    }

    // Add endpoint constraints
    a[(nrows - 2, 0)] = 6.0 * knots[0].x;
    a[(nrows - 2, 1)] = 2.0;

    a[(nrows - 1, ncols - 4)] = 6.0 * knots[knots.len() - 1].x;
    a[(nrows - 1, ncols - 3)] = 2.0;

    // println!("A: {}", a);

    // println!("b: {}", b);

    // Solve the linear system
    let a_inv = a.try_inverse().expect("Failed to invert the A matrix.");
    let solution =  a_inv * b;

    // println!("solution: {}", solution);

    // Encode the splines
    let mut splines: Vec<Spline> = Vec::new();

    for i in 0..num_splines {
        let spline_offset: usize = i * PARAMETERS_PER_SPLINE;

        let spline: Spline = Spline {
            a: solution[spline_offset],
            b: solution[spline_offset + 1],
            c: solution[spline_offset + 2],
            d: solution[spline_offset + 3],
            knot0: knots[i].x,
            knot1: knots[i+1].x,
        };

        splines.push(spline);
    }

    dbg!(&splines);

    // Export the splines into a CSV
    save_splines("output.csv", &splines);
}


fn load_knots(input_path: &str) -> Vec<Knot> {
    // Load knots from CSV
    let mut knots: Vec<Knot> = csv::Reader::from_path(input_path).expect("Failed to read input CSV.")
        .deserialize().collect::<Result<Vec<Knot>, _>>().expect("Failed to parse input CSV.");

    // Ensure the knots are finite floats
    knots.iter().all(|knot| knot.x.is_finite() && knot.y.is_finite())
        .then(|| ()).expect("Invalid floats in the input CSV.");

    // Sort knots by x component
    // (`unwrap` will never trigger since we've checked for finiteness above)
    knots.sort_by(|knot0, knot1| knot0.x.partial_cmp(&knot1.x).unwrap());

    // Ensure the knots x positions are unique (requires sorting, above)
    let nknots = knots.len();
    knots.dedup_by_key(|knot| knot.x);
    (knots.len() == nknots).then(|| ()).expect("Found duplicate knot x positions in input.");

    knots
}


fn save_splines(output_path: &str, splines: &Vec<Spline>) {
    let mut wtr = csv::Writer::from_path(output_path)
        .expect("Failed to open output CSV.");
    
    for spline in splines {
        wtr.serialize(spline)
            .expect("Filed to serialize a spline into the output CSV.");
    }

    wtr.flush()
        .expect("Failed to flush the output CSV.");
}
