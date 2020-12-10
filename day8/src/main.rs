use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Assembly { opcode: String, argument: i32 }

#[derive(Clone, Debug)]
enum Operation { Acc(i32), Jmp(i32), Nop(i32) }

fn parse_operation(assembly: Assembly) -> Option<Operation> {
    if assembly.opcode == "nop" {
        Some(Operation::Nop(assembly.argument))
    } else if assembly.opcode == "jmp" {
        Some(Operation::Jmp(assembly.argument))
    } else if assembly.opcode == "acc" {
        Some(Operation::Acc(assembly.argument))
    } else {
        None
    }
}

struct Machine { pc : i32, acc : i32 }

fn step(code : &Vec<Operation>, machine : &mut Machine) {
    match code.get(machine.pc as usize).unwrap() {
        Operation::Nop(_) => { machine.pc += 1; }
        Operation::Acc(acc_offset) => { machine.acc += acc_offset; machine.pc += 1; }
        Operation::Jmp(jmp_offset) => { machine.pc += jmp_offset; }
    }
}

fn run_code_until_loop(code : &Vec<Operation>) {
    let mut machine : Machine = Machine { pc: 0, acc: 0 };
    let mut visited_pcs : Vec<i32> = Vec::new();
    while !visited_pcs.contains(&machine.pc) {
        visited_pcs.push(machine.pc);
        step(&code, &mut machine);
    }
    println!("{}", machine.acc);
}

fn run_code_until_end(code : &Vec<Operation>) {
    let mut iters = 0;
    let mut machine : Machine = Machine { pc: 0, acc: 0 };
    while machine.pc != code.len() as i32 {
        step(&code, &mut machine);
        iters += 1;
        if iters > 20000 {
            return;
        }
    }
    println!("{}", machine.acc);
}

fn main() {
    let mut code : Vec<Operation> = Vec::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let assembly = Assembly
          { opcode: line.get(0..3).unwrap().to_string() 
          , argument: line.get(4..).unwrap().parse::<i32>().unwrap()
          };
        code.push(parse_operation(assembly).unwrap());
    }
    run_code_until_loop(&code);

    for (op_index, op) in code.iter().enumerate() {
        let mut new_code = code.clone();
        match op {
            Operation::Nop(offset) => { new_code[op_index] = Operation::Jmp(*offset); },
            Operation::Jmp(offset) => { new_code[op_index] = Operation::Nop(*offset); },
            _ => continue
        }
        run_code_until_end(&new_code);
    }
}
