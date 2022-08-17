use std::env;

#[derive(Debug)]
pub struct InterpolateConfig {
    pub input_path: String,
    pub output_path: String,
}

#[derive(Debug)]
pub struct EvaluateConfig {
    pub input_path: String,
}

#[derive(Debug)]
pub enum Config {
    Interpolate(InterpolateConfig),
    Evaluate(EvaluateConfig),
}

pub fn parse() -> Config {
    let mut args = env::args().skip(1); // ignore the path

    match &args.next() {
        Some(command) => {
            let config = match command.as_str() {
                "interpolate" => {
                    let input_path = args.next().expect("Please enter the input path.");
                    let output_path = args.next().expect("Please enter the output path.");

                    Config::Interpolate(InterpolateConfig {
                        input_path,
                        output_path,
                    })
                }
                "evaluate" => {
                    let input_path = args.next().expect("Please enter the input path.");
                    
                    Config::Evaluate(EvaluateConfig { input_path })
                }
                _ => panic!("Please enter a valid command."),
            };

            assert!(
                args.next().is_none(),
                "You entered too many arguments for this command."
            );

            config
        }
        _ => {
            panic!("Please enter a command.")
        }
    }
}
