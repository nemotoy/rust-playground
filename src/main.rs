use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN app",
    version = "0.1.0",
    author = "name",
    about = "My RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name= "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
