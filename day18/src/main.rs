use std::io::{self, BufRead};
use std::slice::Iter;

#[derive(Debug)]
enum Token { Plus, Times, OpenParen, CloseParen, Num(u64) }

#[derive(Debug)]
enum Expr { Add(Box<Expr>, Box<Expr>), Multiply(Box<Expr>, Box<Expr>), Bracketed(Box<Expr>), Num(u64) }

fn lex(line : String) -> Vec<Token> {
    let mut next_char : Option<char> = None;
    let mut chars = line.chars();
    let mut tokens : Vec<Token> = Vec::new();
    while let Some(char) = next_char.or_else(|| chars.next()) {
        next_char = None;
        if char == '+' {
            tokens.push(Token::Plus)
        } else if char == '*' {
            tokens.push(Token::Times)
        } else if char == '(' {
            tokens.push(Token::OpenParen)
        } else if char == ')' {
            tokens.push(Token::CloseParen)
        } else if char == ' ' {
            // nothing
        } else if char.is_digit(10) {
            let mut n : u64 = 0;
            let mut current_char = Some(char);
            while let Some(next_d) = current_char.and_then(|c| c.to_digit(10)) {
                n = n * 10 + (next_d as u64);
                current_char = chars.next();
            }
            tokens.push(Token::Num(n));
            next_char = current_char;
        }
    }
    tokens
}


fn parse1(lexed : &mut Iter<Token>) -> Expr {
    if let Some(op) = lexed.next() {
        match op {
            Token::Num(n) => {
                return Expr::Num(*n);
            },
            Token::OpenParen => {
                return parse(lexed);
            },
            _ => {
                panic!("what.");
            }
        }
    } else {
        panic!("nhat.");
    }
}



fn parse(lexed : &mut Iter<Token>) -> Expr {
    let mut so_far : Option<Expr> = None;
    while let Some(op) = lexed.next() {
        match op {
            Token::Num(n) => {
                so_far = Some(Expr::Num(*n));
            }
            Token::OpenParen => {
                so_far = Some(parse(lexed));
            }
            Token::CloseParen => {
                return so_far.unwrap();
            },
            Token::Plus => {
                let second = parse1(lexed);
                so_far = Some(Expr::Add(Box::new(so_far.unwrap()), Box::new(second)));
            },
            Token::Times => {
                let second = parse1(lexed);
                so_far = Some(Expr::Multiply(Box::new(so_far.unwrap()), Box::new(second)));
            }
        }
    }
    return so_far.unwrap();
}

fn eval(expr : Expr) -> u64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Bracketed(expr) => eval(*expr),
        Expr::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expr::Multiply(lhs, rhs) => eval(*lhs) * eval(*rhs),
    }
}

fn main() {
    let mut sum = 0;
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let lexed = lex(line);
        let parsed = parse(&mut lexed.iter());

        dbg!(&parsed);
        let evaled = eval(parsed);
        dbg!(evaled);
        sum += evaled;
    }
    println!("{}", sum);
}
