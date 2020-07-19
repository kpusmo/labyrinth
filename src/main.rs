use args::Args;
use std::env;
use std::error::Error;
use getopts::Occur;
use labyrinth::{solve_labyrinth, solve_binary};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args = parse_args(env::args())?;
    if args.value_of("help")? {
        println!("{}", args.full_usage());
        return Ok(());
    }
    let input_path: String = args.value_of("path")?;
    if args.value_of("binary")? {
        match solve_binary(&input_path) {
            Ok(results) => {
                for r in results {
                    println!("{}", r);
                }
            },
            Err(error) => println!("Error: {}", error.to_string())
        }
    } else {
        match solve_labyrinth(&input_path) {
            Ok(turns) => println!("{}", turns),
            Err(error) => println!("Error: {}", error.to_string())
        }
    }
    Ok(())
}

fn parse_args(raw_args: env::Args) -> Result<Args> {
    let mut args = Args::new("labirynth", "Finds a way out from a labyrinth");
    args.flag("h", "help", "Prints this message");
    args.flag("b", "binary", "Binary calc mode");
    args.option("p",
                "path",
                "Input file path",
                "PATH",
                Occur::Optional,
                Some("input/sample-1.in".to_owned()),
    );
    args.parse(raw_args)?;
    Ok(args)
}