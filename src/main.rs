mod opcode;

use opcode::{Code, Opcode};
use std::io::{Read, Write};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0; 1] } //
    }
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        // let aa: Result<u8, u8>;
        let code = Code::from(data)?;

        let code_len = code.instrs.len();

        let mut pc: usize = 0;
        let mut ps: usize = 0;

        loop {
            if pc >= code_len {
                println!("pc={} code_len={}", pc, code_len);
                // 退出循环
                break;
            }
            match code.instrs[pc] {
                Opcode::SHR => {
                    ps += 1;
                    if ps == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::SHL => {
                    if ps != 0 {
                        ps -= 1;
                    }
                }
                Opcode::ADD => self.stack[ps] = self.stack[ps].overflowing_add(1).0,
                Opcode::SUB => self.stack[ps] = self.stack[ps].overflowing_sub(1).0,
                Opcode::PUTCHAR => {
                    std::io::stdout().write(&[self.stack[ps]])?;
                    // print!("{}", self.stack[ps] as char);
                }
                Opcode::GETCHAR => {
                    // std::io::stdout().write(&self.stack[ps].to_be_bytes())?;
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                _ => unreachable!(),
            }
            pc += 1;
        }

        Ok(())
    }
}

fn main1() -> Result<(), Box<dyn std::error::Error>> {
    // let a = 1;
    // println!("{}", a);
    let args: Vec<String> = std::env::args().collect(); // 获取命令行参数

    let data = std::fs::read(&args[1])?;

    let code = Code::from(data)?;

    // println!("{:#?}", code);
    println!("{:?}", code.instrs);

    // mod1::Opcode::ADD;
    // println!("Hello, world!");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;

    let mut interpreter = Interpreter::new();
    interpreter.run(data).expect("TODO: panic message");

    Ok(())
}
// rust-toolchain
// .\target\debug\hello .\res\hello_world.bf
