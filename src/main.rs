use std::{fs, env};
use std::collections::VecDeque;

use anyhow::{anyhow, Context, Result as AResult};

#[derive(Debug)]
struct SumikkoBrain {
    pointer: usize,
    list: VecDeque<u8>,
}

impl SumikkoBrain {
    fn inc_sumikko(&mut self) -> AResult<()> {
        if self.list[self.pointer] == 255 { 
            return Err(anyhow!("これ以上値を増やせないよ〜!!")) 
        }
        self.list[self.pointer] += 1;
        return Ok(())
    }

    fn dec_sumikko(&mut self) -> AResult<()> {
        if self.list[self.pointer] == 0 { 
            return Err(anyhow!("これ以上値を減らせないよ〜!!")) 
        }
        self.list[self.pointer] -= 1;
        return Ok(())
    }
}

fn main() -> AResult<()> {
    let filepath = env::args().nth(1)
        .with_context(|| "引数がおかしいよ〜!!")?;
    let contents = fs::read_to_string(filepath)
        .with_context(|| "ファイルが見当たらないよ〜!!")?
        .chars()
        .collect::<Vec<char>>();

    let mut pointer = 0;
    let mut sumikko_brain = SumikkoBrain { pointer: 0, list: VecDeque::from(vec![0]) };
    
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

        match contents.get(pointer - 5..pointer) { // ぺんぎん?: すみっコを１匹増やす
            Some(slice) if slice == penguin => {
                sumikko_brain.inc_sumikko()?;
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

        match contents.get(pointer - 3..pointer) { // とかげ: すみっコを１匹減らす
            Some(slice) if slice == tokage => {
                sumikko_brain.dec_sumikko()?;
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
    println!("{:?}", sumikko_brain);
    return Ok(())
}