mod argparse;
mod common;
mod interpolate;

use interpolate::interpolate;

fn main() {
    match argparse::parse() {
        argparse::Config::Interpolate(config) => interpolate(config),
        argparse::Config::Evaluate(_) => panic!("evaluate still not implemented."),
    }
}
