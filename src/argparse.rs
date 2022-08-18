use std::env;

#[derive(Debug)]
pub struct InterpolateConfig {
    pub input_path: String,
    pub output_path: String,
}

#[derive(Debug)]
pub struct EvaluateConfig {
    pub input_path: String,
    pub values: Vec<f64>,
}

#[derive(Debug)]
pub enum Config {
    Interpolate(InterpolateConfig),
    Evaluate(EvaluateConfig),
}

pub fn parse() -> Config {
    let mut args = env::args().skip(1); // ignore the path

    match &args.next() {
        Some(command) => match command.as_str() {
            "interpolate" => {
                let input_path = args.next().expect("Please enter the input path.");
                let output_path = args.next().expect("Please enter the output path.");

                assert!(
                    args.next().is_none(),
                    "You entered too many arguments for this command."
                );

                Config::Interpolate(InterpolateConfig {
                    input_path,
                    output_path,
                })
            }
            "evaluate" => {
                let input_path: String = args.next().expect("Please enter the input path.");
                let values: Vec<f64> = args
                    .map(|value| value.parse().unwrap())
                    .collect::<Vec<f64>>();

                Config::Evaluate(EvaluateConfig { input_path, values })
            }
            _ => panic!("Please enter a valid command."),
        },
        _ => {
            panic!("Please enter a command.")
        }
    }
}
