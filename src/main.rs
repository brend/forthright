use std::{env, fmt, io};

#[derive(Debug)]
enum Inst {
    Con(i32),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Print,
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseToken(String),
    StackUnderflow,
    DivZero,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::ParseToken(t) => write!(f, "could not parse token: {t:?}"),
            Error::StackUnderflow => write!(f, "stack underflow"),
            Error::DivZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for Error {}

use std::io::Read;

use Inst::*;

fn eval_binop(stack: &mut Vec<i32>, op: fn(i32, i32) -> i32) -> Result<(), Error> {
    let o2 = stack.pop().ok_or(Error::StackUnderflow)?;
    let o1 = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(op(o1, o2));
    Ok(())
}

fn eval(program: &[Inst]) -> Result<(), Error> {
    let mut stack = vec![];

    for inst in program {
        match inst {
            Con(n) => stack.push(*n),
            Add => eval_binop(&mut stack, |a, b| a + b)?,
            Sub => eval_binop(&mut stack, |a, b| a - b)?,
            Mul => eval_binop(&mut stack, |a, b| a * b)?,
            Div => {
                let b = stack.pop().ok_or(Error::StackUnderflow)?;
                let a = stack.pop().ok_or(Error::StackUnderflow)?;

                if b == 0 {
                    return Err(Error::DivZero);
                }

                stack.push(a / b);
            }
            Dup => {
                let o = *stack.last().ok_or(Error::StackUnderflow)?;
                stack.push(o);
            }
            Drop => {
                stack.pop().ok_or(Error::StackUnderflow)?;
            }
            Swap => {
                let len = stack.len();
                if len < 2 {
                    return Err(Error::StackUnderflow);
                }
                stack.swap(len - 1, len - 2);
            }
            Print => {
                let n = stack.pop().ok_or(Error::StackUnderflow)?;
                println!("{n}");
            }
        }
    }

    Ok(())
}

fn parse_token(token: &str) -> Result<Inst, Error> {
    Ok(match token {
        "+" => Add,
        "-" => Sub,
        "*" => Mul,
        "/" => Div,
        "dup" => Dup,
        "drop" => Drop,
        "swap" => Swap,
        "." => Print,
        s => s
            .parse::<i32>()
            .map(Con)
            .map_err(|_| Error::ParseToken(s.to_string()))?,
    })
}

fn parse_program(input: &str) -> Result<Vec<Inst>, Error> {
    input.split_whitespace().map(parse_token).collect()
}

fn read_input() -> Result<String, Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).map_err(Error::Io)?;
        Ok(input)
    } else {
        Ok(args.join(" "))
    }
}

fn run() -> Result<(), Error> {
    let input = read_input()?;
    let program = parse_program(&input)?;
    eval(&program)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
