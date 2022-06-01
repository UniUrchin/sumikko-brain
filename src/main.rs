use std::env;
use std::fs;

use anyhow::{Context, Result as AResult};

fn main() -> AResult<()> {
    let filepath = env::args().nth(1)
        .with_context(|| "引数がおかしいんだよ〜!!")?;
    let contents = fs::read_to_string(filepath)
        .with_context(|| "ファイルが見当たらないよ〜!!")?
        .chars()
        .collect::<Vec<char>>();

    let mut pointer = 0;
    
    let sirokuma = String::from("しろくま").chars().collect::<Vec<char>>();
    let tonkatu = String::from("とんかつ").chars().collect::<Vec<char>>();
    let penguin = String::from("ぺんぎん?").chars().collect::<Vec<char>>();
    let tokage = String::from("とかげ").chars().collect::<Vec<char>>();
    let neko = String::from("ねこ").chars().collect::<Vec<char>>();
    let tapioka = String::from("たぴおか").chars().collect::<Vec<char>>();
    let ebitail = String::from("えびふらいのしっぽ").chars().collect::<Vec<char>>();
    let ajitail = String::from("あじふらいのしっぽ").chars().collect::<Vec<char>>();
    let zasso = String::from("ざっそう").chars().collect::<Vec<char>>();
    let hurosiki = String::from("ふろしき").chars().collect::<Vec<char>>();

    while pointer < contents.len() {
        pointer += 9;

        match contents.get(pointer - 9..pointer) {
            Some(slice) if slice == ebitail => {
                println!("えびふらいのしっぽ");
            },
            Some(slice) if slice == ajitail => {
                println!("あじふらいのしっぽ");
            },
            _ => pointer -= 4,
        }

        match contents.get(pointer - 5..pointer) {
            Some(slice) if slice == penguin => {
                println!("ぺんぎん?");
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 4..pointer) {
            Some(slice) if slice == sirokuma => {
                println!("しろくま");
            },
            Some(slice) if slice == tonkatu => {
                println!("とんかつ");
            },
            Some(slice) if slice == tapioka => {
                println!("たぴおか");
            },
            Some(slice) if slice == zasso => {
                println!("ざっそう");
            },
            Some(slice) if slice == hurosiki => {
                println!("ふろしき");
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 3..pointer) {
            Some(slice) if slice == tokage => {
                println!("とかげ");
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 2..pointer) {
            Some(slice) if slice == neko => {
                println!("ねこ");
            },
            _ => pointer -= 1,
        }
    }

    return Ok(())
}