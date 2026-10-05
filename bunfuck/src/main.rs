use std::env;
use std::fs::File;
use std::io::Read;

const MAX_MEMORY: usize = 640 * 1024;
const MAX_STACK: usize = 1024;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    IncDp,
    DecDp,
    IncVal,
    DecVal,
    Output,
    Input,
    OpenBracketUnlinked,
    CloseBracketUnlinked,
    OpenBracket(usize),
    CloseBracket(usize)
}

impl Op {
    fn get(c: char) -> Option<Self> {
        match c {
            '>' => Some(Op::IncDp),
            '<' => Some(Op::DecDp),
            '+' => Some(Op::IncVal),
            '-' => Some(Op::DecVal),
            '.' => Some(Op::Output),
            ',' => Some(Op::Input),
            '[' => Some(Op::OpenBracketUnlinked),
            ']' => Some(Op::CloseBracketUnlinked),
            _ => None,
        }
    }
}

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .expect("The program name should always be the first arg");
    let Some(path) = args.next() else {
        print_usage(&program);
        return;
    };
    
    let debug = false; // todo: replace with flag
    
    let mut file = File::open(&path).expect("Path not found");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Could not read file");
    
    let mut debug_line = 0;
    let mut debug_column = 0;
    let mut ops = Vec::<Op>::with_capacity(code.len()); // should be about what we need
    for c in code.chars() {
        if c.is_whitespace() {
            if c == '\n' {
                debug_line += 1;
                debug_column = 0;
            } else {
                debug_column += 1;
            }
            continue;
        }
        
        let op = if debug {
            Op::get(c).expect(&format!("Unknown char '{}' at {}:{}:{}", c, &path, debug_line + 1, debug_column + 1))
        } else {
            let Some(op) = Op::get(c) else {
                continue;
            };
            op
        };
        
        ops.push(op);
    }
    
    let mut paren_stack = vec!();
    for i in 0..ops.len() {
        let Some(op) = ops.get(i) else {
            panic!("Could not find op");
        };
        
        if *op == Op::OpenBracketUnlinked {
            paren_stack.push(i);
        }
        if *op == Op::CloseBracketUnlinked {
            let Some(open) = paren_stack.pop() else {
                panic!("No more brackets on the stack");
            };
            ops[i] = Op::CloseBracket(open);
            ops[open] = Op::OpenBracket(i);
        }
    }
    
    if !paren_stack.is_empty() {
        panic!("Brackets are unbalanced");
    }
    
    run(ops);
}

fn run(ops: Vec<Op>) {
    let mut tape: [u8; MAX_MEMORY] = [0; MAX_MEMORY]; // 640KB should be enough for anybody
    let mut dp: usize = 0;
    let mut ip: usize = 0;
    
    while ip < ops.len() {
        let Some(op) = ops.get(ip) else {
            panic!("DP is out of ops size");
        };
        
        match op {
            Op::IncDp => dp = dp.wrapping_add(1),
            Op::DecDp => dp = dp.wrapping_sub(1),
            Op::IncVal => tape[dp] = tape[dp].wrapping_add(1),
            Op::DecVal => tape[dp] = tape[dp].wrapping_sub(1),
            Op::Output => {
                let Some(char) = char::from_u32(tape[dp] as u32) else {
                    continue;
                };
                print!("{}", char);
            },
            Op::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Except stdin to work lol");
                let char = input.chars().nth(0).unwrap_or_default();
                tape[dp] = char as u8;
            }
            Op::OpenBracket(jump) => {
                if tape[dp] == 0 {
                    ip = *jump;
                }
            }
            Op::CloseBracket(jump) => {
                if tape[dp] != 0 {
                    ip = *jump;
                }
            }
            
            Op::OpenBracketUnlinked | Op::CloseBracketUnlinked => unreachable!("Unlinked brackets"),
        }
        
        ip += 1;
    }
}

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <input>");
}