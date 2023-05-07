use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
  #[arg(help = "SMBR file path")]
  filepath: String,
  #[arg(help = "input value [option]")]
  input: Option<String>,
}

fn main() {
  let args = Args::parse();
  let path = args.filepath;
  let input = args.input.map_or(String::new(), |input| input);
  let code = std::fs::read_to_string(&path).unwrap();

  let result = match sumikko_brain::run(&code, &input) {
    Ok(output) => output,
    Err(message) => message,
  };

  println!("{}", result);
}
