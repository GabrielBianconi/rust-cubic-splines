mod argparse;
mod common;
mod evaluate;
mod interpolate;

use evaluate::run_evaluate;
use interpolate::run_interpolate;

fn main() {
    match argparse::parse() {
        argparse::Config::Interpolate(c) => run_interpolate(c),
        argparse::Config::Evaluate(c) => run_evaluate(c),
    };
}
