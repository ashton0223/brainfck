use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::process;
// Needed to work on debug builds
use std::num::Wrapping;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} filename.bf", args[0]);
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut mem: [u8; 30000] = [0; 30000];
    let mut ptr: usize = 0;
    let mut loops: Vec<usize> = Vec::new();
    let mut pc = 0;
    let mut program: Vec<char> = contents.chars().collect();
    let program_len = program.len();
    loop {
        if pc == program_len {
            break;
        }
        run_char(&mut mem, &mut ptr,  &mut program, &mut loops, &mut pc);
        pc += 1;
    }
    println!();
}

fn run_char(mem: &mut [u8; 30000], ptr: &mut usize, program: &mut Vec<char>, loops: &mut Vec<usize>, pc: &mut usize) {
    match program[*pc] {
        '>' => {
            *ptr = wrapping_add_usize(ptr, &1) as usize;
        },
        '<' => {
            *ptr = wrapping_subtract_usize(ptr, &1) as usize;
        },
        '+' => {
            mem[*ptr] = wrapping_add_u8(&mem[*ptr], &1);
        },
        '-' => {
            mem[*ptr] = wrapping_subtract_u8(&mem[*ptr], &1);
        },
        '.' => {
            print!("{}", mem[*ptr] as char);
        },
        ',' => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            // Only takes first char
            let chars: Vec<char> = buffer.chars().collect();
            mem[*ptr] = chars[0] as u8;
        },
        '[' => {
            if mem[*ptr] == 0 {
                let mut counter = 1;
                let mut i = *pc;
                let length = program.len();
                while counter > 0 {
                    match program[i] as char {
                        '[' => {
                            counter += 1;
                        },
                        ']' => {
                            counter -= 1;
                        }
                        _ => {}
                    }
                    i += 1;
                    if i >= length {
                        println!();
                        process::exit(1);
                    }
                }
                *pc = i;
            } else {
                loops.push(*pc);
            }
        },
        ']' => {
            let old_pc = loops.pop().unwrap();
            if mem[*ptr] != 0 {
                *pc = old_pc - 1;
            }
        }
        _ => {}
    }
}

fn wrapping_add_u8(num1: &u8, num2: &u8) -> u8 {
    let num1 = Wrapping(*num1);
    let num2 = Wrapping(*num2);
    (num1 + num2).0
}

fn wrapping_add_usize(num1: &usize, num2: &usize) -> usize {
    let num1 = Wrapping(*num1);
    let num2 = Wrapping(*num2);
    (num1 + num2).0
}

fn wrapping_subtract_u8(num1: &u8, num2: &u8) -> u8 {
    let d_num1 = *num1;
    let num1 = Wrapping(d_num1);
    let num2 = Wrapping(*num2 as u8);
    (num1 - num2).0
}

fn wrapping_subtract_usize(num1: &usize, num2: &usize) -> usize {
    let num1 = Wrapping(*num1);
    let num2 = Wrapping(*num2);
    (num1 - num2).0
}