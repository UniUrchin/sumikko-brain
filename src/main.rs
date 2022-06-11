use std::{env, fs};
use std::io::Read;
use std::collections::VecDeque;

use anyhow::{anyhow, Context, Result as AResult};
use rand::Rng;

#[derive(Debug)]
struct SumikkoBrain {
    pointer: usize,
    list: VecDeque<u8>,
    tail: VecDeque<usize>,
}

impl SumikkoBrain {
    fn move_right(&mut self) {
        if self.pointer == self.list.len() - 1 {
            self.list.push_back(0);
        }
        self.pointer += 1;
    }

    fn move_left(&mut self) {
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

    fn print_sumikko_integer(&mut self) {
        print!("{}", self.list[self.pointer]);
    }

    fn print_sumikko_ascii(&mut self) {
        print!("{}", self.list[self.pointer] as char);
    }

    fn read_sumikko(&mut self) -> AResult<()> {
        let mut input: [u8; 1] = [0];
        match std::io::stdin().read_exact(&mut input) {
            Ok(_) => self.list[self.pointer] = input[0],
            _ => return Err(anyhow!("すみっコの気持ちが読み取れないよ〜!!")),
        }
        return Ok(())
    }

    fn create_memory(&mut self, pointer: &mut usize) {
        self.tail.push_back(*pointer);
    }

    fn remember_memory(&mut self, pointer: &mut usize) -> AResult<()> {
        if self.tail.is_empty() {
            return Err(anyhow!("すみっコ達のことが思い出せないよ〜!!"))
        }
        if self.list[self.pointer] != 0 {
            match self.tail.back() {
                Some(smbr_pointer) => {
                    *pointer = *smbr_pointer;
                },
                None => (),
            }
        } else {
            self.tail.pop_back();
        }
        return Ok(())
    }

    fn random_sumikko(&mut self) {
        let mut rng = rand::thread_rng();
        self.list[self.pointer] = rng.gen_range(0..=255);
    }
}

fn main() -> AResult<()> {
    let filepath = env::args()
        .nth(1)
        .with_context(|| "コマンドの引数がおかしいよ〜!!")?;
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
    
    let sirokuma = String::from("しろくま")
        .chars()
        .collect::<Vec<char>>();
    let tonkatu = String::from("とんかつ")
        .chars()
        .collect::<Vec<char>>();
    let penguin = String::from("ぺんぎん?")
        .chars()
        .collect::<Vec<char>>();
    let tokage = String::from("とかげ")
        .chars()
        .collect::<Vec<char>>();
    let neko = String::from("ねこ")
        .chars()
        .collect::<Vec<char>>();
    let tapioka = String::from("たぴおか")
        .chars()
        .collect::<Vec<char>>();
    let ebitail = String::from("えびふらいのしっぽ")
        .chars()
        .collect::<Vec<char>>();
    let ajitail = String::from("あじふらいのしっぽ")
        .chars()
        .collect::<Vec<char>>();
    let zasso = String::from("ざっそう")
        .chars()
        .collect::<Vec<char>>();
    let hurosiki = String::from("ふろしき")
        .chars()
        .collect::<Vec<char>>();

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
            Some(slice) if slice == sirokuma => { // しろくま: 右の方に移動する
                sumikko_brain.move_right();
                continue
            },
            Some(slice) if slice == tonkatu => { // とんかつ: 左の方に移動する
                sumikko_brain.move_left();
                continue
            },
            Some(slice) if slice == tapioka => { // たぴおか: すみっコの気持ちを読み取る
                sumikko_brain.read_sumikko()?;
                continue
            },
            Some(slice) if slice == zasso => { // ざっそう: すみっコの数を数字で表示する
                sumikko_brain.print_sumikko_integer();
                continue
            },
            Some(slice) if slice == hurosiki => { // ふろしき: ランダムにすみっコを呼んでくる
                sumikko_brain.random_sumikko();
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

        match contents.get(pointer - 2..pointer) { // ねこ: すみっコの数を文字で表示する
            Some(slice) if slice == neko => {
                sumikko_brain.print_sumikko_ascii();
                continue
            },
            _ => pointer -= 1,
        }
    }
    return Ok(())
}