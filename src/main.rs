mod argparse;
mod common;
mod interpolate;

use interpolate::interpolate;

fn main() {
    match argparse::parse() {
        argparse::Config::Interpolate(c) => interpolate(&c.input_path, &c.output_path),
        argparse::Config::Evaluate(_) => panic!("evaluate still not implemented."),
    }
}
