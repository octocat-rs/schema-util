use clap::Parser;
use schema_util::{model::Modeler, traits::PrintAsStruct};
use std::{error::Error, fs};

// TODO: Figure out how to handle piping
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read
    #[clap(short, long)]
    file: String,

    /// Name of the file to write the struct to. If this is not set, the struct
    /// will be printed to stdout.
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let text = fs::read_to_string(args.file)?;
    let json = serde_json::from_str::<Modeler>(text.as_str())?;
    let output = json.print();

    if let Some(f) = args.output {
        fs::write(f, output)?;
    } else {
        println!("{output}");
    }

    Ok(())
}
