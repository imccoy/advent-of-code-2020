use std::collections::HashMap;
use std::io::{self, BufRead};

enum MaskBit { MaskZero, MaskOne, MaskBlank }
struct Mask { mask_bits : Vec<MaskBit> }
struct Write { address : u64, value : u64 }

enum ProgramLine { ProgramMask(Mask), ProgramWrite(Write) }

fn parse_mask(mask : &str) -> Mask {
    Mask { mask_bits: mask
      .chars()
      .map(|c| {
        match c {
            '0' => MaskBit::MaskZero,
            '1' => MaskBit::MaskOne,
            _   => MaskBit::MaskBlank
        }
      })
      .collect()}
}

fn parse_write(mem : &str, value : &str) -> Write {
    Write { address: mem[4..mem.len() - 1].parse::<u64>().unwrap(), value: value.parse::<u64>().unwrap() }
}

fn parse_line(line : String) -> ProgramLine {
    let bits = line.split(" = ").collect::<Vec<&str>>();
    if bits[0] == "mask" {
        ProgramLine::ProgramMask(parse_mask(bits[1]))
    } else {
        ProgramLine::ProgramWrite(parse_write(bits[0], bits[1]))
    }
}

fn part1(program : &Vec<ProgramLine>) {
    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut mask_out : u64 = 0xffffffffffff;
    let mut mask_in : u64 = 0;
    for line in program {
        match line {
            ProgramLine::ProgramMask(mask) => {
                mask_out = 0;
                mask_in = 0;
                for mask_bit in &mask.mask_bits {
                    mask_out = mask_out << 1;
                    mask_in = mask_in << 1;
                    match mask_bit {
                        MaskBit::MaskBlank => {
                            mask_out |= 1;
                            mask_in |= 0;
                        },
                        MaskBit::MaskOne => {
                            mask_out |= 1;
                            mask_in |= 1;
                        },
                        MaskBit::MaskZero => {
                            mask_out |= 0;
                            mask_in |= 0;
                        }
                    }
                }
            },
            ProgramLine::ProgramWrite(write) => {
                mem.insert(write.address, (write.value & mask_out) | mask_in);
            }
        }
    }
    println!("{}", mem.values().sum::<u64>());
}

fn write_all_float_values(mem : &mut HashMap<u64, u64>, floats : &[u64], address : u64, value : u64) {
    if floats.len() == 0 {
        mem.insert(address, value);
    } else {
        write_all_float_values(mem, &floats[1..], address | (1 << floats[0]), value);
        write_all_float_values(mem, &floats[1..], address & !(1 << floats[0]), value);
    }
}

fn part2(program : &Vec<ProgramLine>) {
    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut mask_in : u64 = 0;
    let mut floats : Vec<u64> = Vec::new();
    for line in program {
        match line {
            ProgramLine::ProgramMask(mask) => {
                floats = Vec::new();
                let mut current_bit = 36;
                mask_in = 0;
                for mask_bit in &mask.mask_bits {
                    current_bit -= 1;
                    match mask_bit {
                        MaskBit::MaskBlank => {
                            floats.push(current_bit);
                        },
                        MaskBit::MaskOne => {
                            mask_in |= 1 << current_bit;
                        },
                        _ => {
                            // do nothing
                        }
                    }
                }
            },
            ProgramLine::ProgramWrite(write) => {
                write_all_float_values(&mut mem, &floats[0..], write.address | mask_in, write.value);
            }
        }
    }
    println!("{}", mem.values().sum::<u64>());
}


fn main() {
    let program : Vec<ProgramLine> = io::stdin().lock().lines().map(|line| parse_line(line.unwrap())).collect();
    part1(&program);
    part2(&program);
}
