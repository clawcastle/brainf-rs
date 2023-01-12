use std::io::{Read, Write};
use std::{error::Error, fmt::Display};

fn main() {
    let mut interpreter = BrainfuckInterpreter::<30_000>::new();

    interpreter.run(",.");
}

struct BrainfuckInterpreter<const N: usize> {
    memory: [u8; N],
    memory_pointer: MemoryPointer<N>,
}

impl<const N: usize> BrainfuckInterpreter<N> {
    pub fn new() -> Self {
        Self {
            memory: [0; N],
            memory_pointer: MemoryPointer::<N>::new(),
        }
    }

    pub fn run(&mut self, program: &'_ str) {
        let instructions: Vec<Instruction> = program
            .chars()
            .filter_map(|c| Instruction::try_from(c).ok())
            .collect();

        for instruction in &instructions {
            match instruction {
                Instruction::IncreasePointer => self.memory_pointer.add(1),
                Instruction::DecreasePointer => self.memory_pointer.subtract(1),
                Instruction::IncreaseValue => self.memory[self.memory_pointer] += 1,
                Instruction::DecreaseValue => self.memory[self.memory_pointer] -= 1,
                Instruction::ReadChar => {
                    if let Some(Ok(c)) = std::io::stdin().bytes().next() {
                        self.memory[self.memory_pointer] = c
                    } else {
                        panic!("Error occured while reading character from stdin.")
                    }
                }
                Instruction::WriteChar => {
                    if std::io::stdout()
                        .write(&[self.memory[self.memory_pointer]])
                        .is_err()
                    {
                        panic!("Error occurred while writing to stdout.")
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MemoryPointer<const N: usize> {
    pointer: usize,
}

impl<const N: usize> MemoryPointer<N> {
    fn new() -> Self {
        Self { pointer: 0 }
    }

    fn add(&mut self, n: usize) {
        self.pointer = (self.pointer + n) % N;
    }

    fn subtract(&mut self, n: usize) {
        if self.pointer >= n {
            self.pointer -= n;
        } else {
            self.pointer = N - ((n & N) - self.pointer)
        }
    }
}

impl<const N: usize> std::ops::Index<MemoryPointer<N>> for [u8; N] {
    type Output = u8;

    fn index(&self, memory_pointer: MemoryPointer<N>) -> &Self::Output {
        &self[memory_pointer.pointer]
    }
}

impl<const N: usize> std::ops::IndexMut<MemoryPointer<N>> for [u8; N] {
    fn index_mut(&mut self, memory_pointer: MemoryPointer<N>) -> &mut Self::Output {
        &mut self[memory_pointer.pointer]
    }
}

#[derive(Debug)]
enum Instruction {
    IncreasePointer,
    DecreasePointer,
    IncreaseValue,
    DecreaseValue,
    StartLoop,
    EndLoop,
    ReadChar,
    WriteChar,
}

impl TryFrom<char> for Instruction {
    type Error = InvalidCharacterError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Instruction::IncreasePointer),
            '<' => Ok(Instruction::DecreasePointer),
            '+' => Ok(Instruction::IncreaseValue),
            '-' => Ok(Instruction::DecreaseValue),
            '[' => Ok(Instruction::StartLoop),
            ']' => Ok(Instruction::EndLoop),
            ',' => Ok(Instruction::ReadChar),
            '.' => Ok(Instruction::WriteChar),
            _ => Err(InvalidCharacterError::new(c)),
        }
    }
}

#[derive(Debug)]
struct InvalidCharacterError {
    character: char,
}

impl InvalidCharacterError {
    pub fn new(c: char) -> Self {
        Self { character: c }
    }
}

impl Error for InvalidCharacterError {}

impl Display for InvalidCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid character encountered: {}", self.character)
    }
}
