use std::env;
use std::fs;

use anyhow::{Context, Result as AResult};

fn main() -> AResult<()> {
    let filepath = env::args()
        .nth(1)
        .with_context(|| "引数がおかしいんだよ〜!!")?;
    let contents = fs::read_to_string(filepath)?;

    println!("{}", contents);

    return Ok(())
}