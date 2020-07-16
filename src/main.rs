use args::Args;
use std::env;
use std::error::Error;
use getopts::Occur;

mod labyrinth;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args = parse_args(env::args())?;
    if args.value_of("help")? {
        println!("{}", args.full_usage());
        return Ok(());
    }
    let input_path: String = args.value_of("path")?;
    if args.value_of("binary")? {
        todo!("binary mode");
    } else {
        labyrinth::solve(&input_path);
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
                Some("input/sample.in".to_owned()),
    );
    args.parse(raw_args)?;
    Ok(args)
}