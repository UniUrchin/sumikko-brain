use std::{fs, env};
use std::collections::VecDeque;

use anyhow::{anyhow, Context, Result as AResult};

#[derive(Debug)]
struct SumikkoBrain {
    pointer: usize,
    list: VecDeque<u8>,
    tail: VecDeque<usize>,
}

impl SumikkoBrain {
    fn move_east(&mut self) {
        if self.pointer == self.list.len() - 1 {
            self.list.push_back(0);
        }
        self.pointer += 1;
    }

    fn move_west(&mut self) {
        if self.pointer == 0 {
            self.list.push_front(0);
        } else {
            self.pointer -= 1;
        }
    }

    fn inc_sumikko(&mut self) -> AResult<()> {
        if self.list[self.pointer] == 255 { 
            return Err(anyhow!("これ以上すみっコを増やせないよ〜!!")) 
        }
        self.list[self.pointer] += 1;
        return Ok(())
    }

    fn dec_sumikko(&mut self) -> AResult<()> {
        if self.list[self.pointer] == 0 { 
            return Err(anyhow!("これ以上すみっコを減らせないよ〜!!")) 
        }
        self.list[self.pointer] -= 1;
        return Ok(())
    }

    fn print_ascii(&mut self) {
        print!("{}", self.list[self.pointer] as char);
    }

    fn create_memory(&mut self, pointer: &mut usize) {
        self.tail.push_back(*pointer);
    }

    fn remember_memory(&mut self, pointer: &mut usize) -> AResult<()> {
        if self.list[self.pointer] != 0 {
            match self.tail.back() {
                Some(smbr_pointer) => {
                    *pointer = *smbr_pointer;
                },
                None => return Err(anyhow!("すみっコ達のことが思い出せないよ〜!!")),
            }
        } else {
            self.tail.pop_back();
        }
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
    let mut sumikko_brain = SumikkoBrain { 
        pointer: 0, 
        list: VecDeque::from(vec![0]),
        tail: VecDeque::new(),
    };
    
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
            Some(slice) if slice == ebitail => { // すみっコ達との思い出を作る
                sumikko_brain.create_memory(&mut pointer);
                continue
            },
            Some(slice) if slice == ajitail => { // すみっコ達との思い出に浸る
                sumikko_brain.remember_memory(&mut pointer)?;
                continue
            },
            _ => pointer -= 4,
        }

        match contents.get(pointer - 5..pointer) { // ぺんぎん?: すみっコを１匹増やす
            Some(slice) if slice == penguin => {
                sumikko_brain.inc_sumikko()?;
                continue
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 4..pointer) {
            Some(slice) if slice == sirokuma => { // しろくま: 東の街に移動する
                sumikko_brain.move_east();
                continue
            },
            Some(slice) if slice == tonkatu => { // とんかつ: 西の街に移動する
                sumikko_brain.move_west();
                continue
            },
            Some(slice) if slice == tapioka => {
                println!("たぴおか");
                continue
            },
            Some(slice) if slice == zasso => {
                println!("ざっそう");
                continue
            },
            Some(slice) if slice == hurosiki => {
                println!("ふろしき");
                continue
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 3..pointer) { // とかげ: すみっコを１匹減らす
            Some(slice) if slice == tokage => {
                sumikko_brain.dec_sumikko()?;
                continue
            },
            _ => pointer -= 1,
        }

        match contents.get(pointer - 2..pointer) { // ねこ: 説明は後で考える...
            Some(slice) if slice == neko => {
                sumikko_brain.print_ascii();
                continue
            },
            _ => pointer -= 1,
        }
    }
    // println!("{}: {:?}", pointer, sumikko_brain);
    return Ok(())
}