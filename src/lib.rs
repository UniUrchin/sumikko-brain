use core::slice::Iter;
use regex::Regex;
use thiserror::Error as ThisError;
use wasm_bindgen::prelude::*;

const MEMORY_SIZE: usize = 30000;
const MAX_POINTER: usize = MEMORY_SIZE - 1;
const MAX_VALUE: u8 = 255;

#[derive(Debug, PartialEq)]
enum SmbrNeoCommand {
  IncrementPointer,
  DecrementPointer,
  IncrementValue,
  DecrementValue,
  PrintChar,
  ReadInput,
  JumpForward,
  JumpBackward,
  CopyValue,
  PasteValue,
  ClearValue,
}

impl SmbrNeoCommand {
  fn from_str(command: &str) -> Result<Self, SmbrNeoError> {
    match command {
      "しろくま" => Ok(SmbrNeoCommand::IncrementPointer),
      "とんかつ" => Ok(SmbrNeoCommand::DecrementPointer),
      "ぺんぎん?" => Ok(SmbrNeoCommand::IncrementValue),
      "とかげ" => Ok(SmbrNeoCommand::DecrementValue),
      "ねこ" => Ok(SmbrNeoCommand::PrintChar),
      "たぴおか" => Ok(SmbrNeoCommand::ReadInput),
      "えびふらいのしっぽ" => Ok(SmbrNeoCommand::JumpForward),
      "あじふらいのしっぽ" => Ok(SmbrNeoCommand::JumpBackward),
      "すずめ" => Ok(SmbrNeoCommand::CopyValue),
      "おばけ" => Ok(SmbrNeoCommand::PasteValue),
      "ざっそう" => Ok(SmbrNeoCommand::ClearValue),
      _ => Err(SmbrNeoError::ParseError),
    }
  }
}

#[derive(ThisError, Debug)]
enum SmbrNeoError {
  #[error("InvalidLoopError: Loop command does not correspond")]
  InvalidLoopError,
  #[error("ParseError: Failed to parse code")]
  ParseError,
  #[error(transparent)]
  RegexError(#[from] regex::Error),
}

#[wasm_bindgen]
pub fn run(code: &str, input: &str) -> Result<String, String> {
  let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
  let mut pointer: usize = 0;
  let mut index: usize = 0;
  let mut stack: Vec<usize> = Vec::new();
  let mut copy: u8 = 0;
  let mut input: Iter<u8> = input.as_bytes().iter();
  let mut output: String = String::new();

  let commands = match parse_code(code) {
    Ok(commands) => commands,
    Err(e) => return Err(e.to_string()),
  };

  while index < commands.len() {
    match commands[index] {
      SmbrNeoCommand::IncrementPointer => match pointer {
        MAX_POINTER => pointer = 0,
        _ => pointer += 1,
      },
      SmbrNeoCommand::DecrementPointer => match pointer {
        0 => pointer = MAX_POINTER,
        _ => pointer -= 1,
      },
      SmbrNeoCommand::IncrementValue => match memory[pointer] {
        MAX_VALUE => memory[pointer] = 0,
        _ => memory[pointer] += 1,
      },
      SmbrNeoCommand::DecrementValue => match memory[pointer] {
        0 => memory[pointer] = MAX_VALUE,
        _ => memory[pointer] -= 1,
      },
      SmbrNeoCommand::PrintChar => output.push(memory[pointer] as char),
      SmbrNeoCommand::ReadInput => match input.next() {
        Some(byte) => memory[pointer] = *byte,
        None => memory[pointer] = 0,
      },
      SmbrNeoCommand::JumpForward => {
        let mut depth = 1;
        let mut tmp = index;

        while depth > 0 {
          tmp += 1;
          if tmp >= commands.len() {
            return Err(SmbrNeoError::InvalidLoopError.to_string());
          }
          match commands[tmp] {
            SmbrNeoCommand::JumpForward => depth += 1,
            SmbrNeoCommand::JumpBackward => depth -= 1,
            _ => (),
          }
        }

        match memory[pointer] {
          0 => index = tmp,
          _ => stack.push(index),
        }
      }
      SmbrNeoCommand::JumpBackward => match memory[pointer] {
        0 => {
          match stack.pop() {
            Some(_) => (),
            None => return Err(SmbrNeoError::InvalidLoopError.to_string()),
          };
        }
        _ => match stack.last() {
          Some(value) => index = *value,
          None => return Err(SmbrNeoError::InvalidLoopError.to_string()),
        },
      },
      SmbrNeoCommand::CopyValue => copy = memory[pointer],
      SmbrNeoCommand::PasteValue => memory[pointer] = copy,
      SmbrNeoCommand::ClearValue => memory[pointer] = 0,
    }
    index += 1;
  }

  return Ok(output);
}

fn parse_code(code: &str) -> Result<Vec<SmbrNeoCommand>, SmbrNeoError> {
  let re = Regex::new(
    r"しろくま|とんかつ|ぺんぎん\?|とかげ|ねこ|たぴおか|えびふらいのしっぽ|あじふらいのしっぽ|すずめ|おばけ|ざっそう",
  )?;
  return Ok(
    re.find_iter(code)
      .map(|matches| matches.as_str())
      .map(|command| SmbrNeoCommand::from_str(command))
      .collect::<Result<Vec<SmbrNeoCommand>, SmbrNeoError>>()?,
  );
}
