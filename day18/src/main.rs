use std::io::{self, BufRead};
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
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


fn parse_un(lexed : &mut Expectant) -> Expr {
    if let Some(op) = next(lexed) {
        match op {
            Token::Num(n) => {
                return Expr::Num(n);
            },
            Token::OpenParen => {
                let expr = Expr::Bracketed(Box::new(parse(lexed)));
                expect(lexed, |tok| match tok { Token::CloseParen => true, _ => false });
                return expr;
            },
            _ => {
                panic!("what.");
            }
        }
    } else {
        panic!("nhat.");
    }
}

fn parse(lexed : &mut Expectant) -> Expr {
    let lhs = parse_un(lexed);
    return parse_bin(lhs, lexed);
}

fn parse_bin(lhs : Expr, lexed : &mut Expectant) -> Expr {
    if expect(lexed, |tok| match tok { Token::Plus => true, _ => false }) {
        let rhs = parse_un(lexed);
        return parse_bin(Expr::Add(Box::new(lhs), Box::new(rhs)), lexed);
    } else if expect(lexed, |tok| match tok { Token::Times => true, _ => false }) {
        let rhs = parse_un(lexed);
        return parse_bin(Expr::Multiply(Box::new(lhs), Box::new(rhs)), lexed);
    }
    return lhs;
}

// this is a utility to make the traditional "consume-next-token-if-it-looks-like-X" device
struct Expectant<'a> {
    iter: &'a mut Iter<'a, Token>,
    current_tok: Option<Token>
}

fn next(lexed : &mut Expectant) -> Option<Token> {
    match lexed.current_tok {
        Some(token) => {
            lexed.current_tok = None;
            return Some(token);
        },
        None => {
            return lexed.iter.next().map(|t| *t);
        }
    }
}

fn expect<F>(lexed : &mut Expectant, f : F) -> bool
    where F : FnOnce(Token) -> bool
{
    if lexed.current_tok.is_none() {
        lexed.current_tok = lexed.iter.next().map(|t| *t);
    }
    if let Some(tok) = lexed.current_tok {
        if f(tok) {
            lexed.current_tok = None;
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn parse2_n(lexed : &mut Expectant) -> Expr {
    if let Some(tok) = next(lexed) {
        match tok {
            Token::Num(n) => {
                return Expr::Num(n);
            },
            Token::OpenParen => {
                let expr = Expr::Bracketed(Box::new(parse2_products(lexed)));
                expect(lexed, |tok| match tok { Token::CloseParen => true, _ => false });
                return expr;
            },
            _ => panic!("not a number")
        }
    }
    panic!("no number");
}

fn parse2_next_n(lexed : &mut Expectant) -> Option<Expr> {
    if expect(lexed, |tok| match tok { Token::Plus => true, _ => false }) {
        return Some(parse2_sums(lexed));
    }
    return None;
}

fn parse2_sums(lexed : &mut Expectant) -> Expr {
    let mut n = parse2_n(lexed);
    while let Some(n2) = parse2_next_n(lexed) {
        n = Expr::Add(Box::new(n), Box::new(n2));
    }
    return n;

}

fn parse2_next_sums(lexed : &mut Expectant) -> Option<Expr> {
    if expect(lexed, |tok| match tok { Token::Times => true, _ => false }) {
        return Some(parse2_products(lexed));
    }
    return None;
}

fn parse2_products(lexed : &mut Expectant) -> Expr {
    let mut sum = parse2_sums(lexed);
    while let Some(sum2) = parse2_next_sums(lexed) {
        sum = Expr::Multiply(Box::new(sum), Box::new(sum2));
    }
    return sum;
}

fn eval(expr : Expr) -> u64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Bracketed(expr) => eval(*expr),
        Expr::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expr::Multiply(lhs, rhs) => eval(*lhs) * eval(*rhs),
    }
}




/* I thought doing something in an utterly unprincipled ad-hoc way
 * might actually be faster. And then I thought it might be fun to
 * try.
 *
 * Welcome to lol.
 */

#[derive(Debug, Clone)]
enum ExprLol { Bracketed(Vec<ExprLol>), Token(Token) }

fn unparen_lol(tokens : &mut Iter<Token>) -> Vec<ExprLol> {
    let mut result = Vec::new();
    while let Some(t) = tokens.next() {
        match t {
            Token::OpenParen => {
                result.push(ExprLol::Bracketed(unparen_lol(tokens)));
            },
            Token::CloseParen => {
                break;
            },
            _ => {
                result.push(ExprLol::Token(*t));
            }
        }
    }
    return result;
}

fn eval_lol_at(tree : &Vec<ExprLol>, index : usize) -> u64 {
    return match &tree[index] {
        ExprLol::Token(Token::Num(num)) => *num,
        ExprLol::Bracketed(subtree) => eval_lol(&subtree),
        _ => panic!("lol")
    };
}

fn eval_lol(tree : &Vec<ExprLol>) -> u64 {
    let mut n = eval_lol_at(tree, 0);
    let mut idx = 1;
    while idx < tree.len() {
        match tree[idx] {
            ExprLol::Token(Token::Plus) => {
                n += eval_lol_at(tree, idx + 1);
                idx += 2;
            },
            ExprLol::Token(Token::Times) => {
                n *= eval_lol_at(tree, idx + 1);
                idx += 2;
            },
            _ => panic!("lol")
        };
    }
    return n;
}

fn precedence_lol(tree : &Vec<ExprLol>) -> Vec<ExprLol> {
    let mut results = Vec::new();
    let mut index = 1;
    let mut tree : Vec<ExprLol> = tree.iter().map(|t| match t {
        ExprLol::Bracketed(nexts) => ExprLol::Bracketed(precedence_lol(&nexts)),
        token => token.clone()
    }).collect();
    results.push(tree[0].clone());
    while index < tree.len() {
        let next = &tree[index + 1];
        match &tree[index] {
            ExprLol::Token(Token::Plus) => {
                let last = results.pop().unwrap();
                results.push(ExprLol::Bracketed(vec!(last, ExprLol::Token(Token::Plus), next.clone())));
            },
            ExprLol::Token(Token::Times) => {
                results.push(tree[index].clone());
                results.push(next.clone());
            },
            _ => panic!("lol")
        }
        index += 2;
    }
    return results;
}
/*
 * </lol>
 */

fn main() {
    let mut sum = 0;
    let mut sum2 = 0;
    let mut sumlol = 0;
    let mut sum2lol = 0;
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let lexed = lex(line.clone());

        let mut ex2 = Expectant { iter: &mut lexed.iter(), current_tok: None };
        let parsed = parse(&mut ex2);
        let evaled = eval(parsed);
        sum += evaled;

        sumlol += eval_lol(&unparen_lol(&mut lexed.iter()));

        let mut ex2 = Expectant { iter: &mut lexed.iter(), current_tok: None };
        let parsed2 = parse2_products(&mut ex2);
        let evaled2 = eval(parsed2);
        sum2 += evaled2;

        sum2lol += eval_lol(&precedence_lol(&unparen_lol(&mut lexed.iter())));
    }
    println!("{}", sum);
    println!("{}", sum2);
    println!("{}", sumlol);
    println!("{}", sum2lol);
}
