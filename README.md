# Cubic Spline Interpolation in Rust

This repository implements [cubic spline interpolation](https://en.wikiversity.org/wiki/Cubic_Spline_Interpolation) in Rust. It relies heavily on `nalgebra` for linear algebra.

> I just started learning Rust (I haven't even finished The Book yet). This is just an exercise for myself. The program seems to work as expected, but it's probably very non-idiomatic and might have edge cases I haven't anticipated. **You probably shouldn't use it in production (or at all).**

## Usage

### Interpolation

```
cargo run interpolate knots.csv spline.csv
```

- `knots.csv` is an input CSV with knots (see `crate::common::Knot`)
- `spline.csv` is an output CSV with the fitted spline segments (see `crate::common::SplineSegments`)

### Evaluation

```
cargo run evaluate spline.csv VALUE1 VALUE2 ...
```

- `spline.csv` is an input CSV with the fitted spline segments (see `crate::common::SplineSegments`)
- `VALUE1 VALUE2 ...` is a sequence of floats (e.g. `0.0 0.1 1.0`)

## Pending Work

- [ ] Refactor error handling
- [ ] More testing
- [ ] GUI
