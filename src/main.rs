use std::{env, io};

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
    Stop,
}

#[derive(Debug)]
enum Error {
    IO,
    Parse,
    Eof,
    StackUnderflow,
}

use std::io::Read;

use Inst::*;

fn eval_binop(stack: &mut Vec<i32>, op: fn(i32, i32) -> i32) -> Result<(), Error> {
    let o2 = stack.pop().ok_or(Error::StackUnderflow)?;
    let o1 = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(op(o1, o2));
    Ok(())
}

fn eval(program: &[Inst], stack: &mut Vec<i32>) -> Result<i32, Error> {
    let Some(i) = program.first() else {
        return Err(Error::Eof);
    };

    match i {
        Con(n) => stack.push(*n),
        Add => eval_binop(stack, |a, b| a + b)?,
        Sub => eval_binop(stack, |a, b| a - b)?,
        Mul => eval_binop(stack, |a, b| a * b)?,
        Div => eval_binop(stack, |a, b| a / b)?,
        Dup => {
            let o = stack.pop().unwrap();
            stack.push(o);
            stack.push(o);
        }
        Drop => {
            stack.pop();
        }
        Swap => {
            let o2 = stack.pop().unwrap();
            let o1 = stack.pop().unwrap();
            stack.push(o2);
            stack.push(o1);
        }
        Stop => return Ok(stack[0]),
    }

    eval(&program[1..], stack)
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
        "." => Stop,
        s => {
            let n = s.parse::<i32>().or(Err(Error::Parse))?;
            Con(n)
        }
    })
}

fn parse_program(input: &str) -> Result<Vec<Inst>, Error> {
    input.split_whitespace().map(parse_token).collect()
}

fn read_input() -> Result<String, Error> {
    if std::env::args().len() > 1 {
        Ok(env::args().skip(1).collect::<Vec<_>>().join(" "))
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).or(Err(Error::IO))?;
        Ok(input)
    }
}

fn main() -> Result<(), Error> {
    let input = read_input()?;
    println!("{input}");
    let program = parse_program(&input)?;
    let mut stack = vec![];
    let result = eval(&program, &mut stack)?;
    println!("{result:?}");
    Ok(())
}
