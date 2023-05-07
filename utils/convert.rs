use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
  #[arg(help = "SMBR file path")]
  filepath: String,
}

fn main() {
  let args = Args::parse();
  let path = args.filepath;
  let code = std::fs::read_to_string(&path).unwrap();

  let result = code
    .replace(">", "しろくま")
    .replace("<", "とんかつ")
    .replace("+", "ぺんぎん?")
    .replace("-", "とかげ")
    .replace(".", "ねこ")
    .replace(",", "たぴおか")
    .replace("*", "ふろしき")
    .replace("[", "えびふらいのしっぽ")
    .replace("]", "あじふらいのしっぽ")
    .replace("#", "ざっそう");

  std::fs::write(path.replace("bf", "smbr"), result).unwrap();
}
