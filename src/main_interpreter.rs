use crate::opcode::Opcode;
use std::fmt::{Debug, Error};
use std::io::prelude::*;

mod opcode;
// use opcode::{Code, Opcode};

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32), // >>> === SHR(4)
    SHL(u32),
    ADD(u8), // ++++ === ADD(4)
    SUB(u8),
    PUTCHAR(u8),
    GETCHAR(u8),
    JIZ(u32), // Jump if zero
    JNZ(u32), // Jump if not zero
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    // pub fn from(&self,data: Vec<u8>) ->Result<Self,Box<dyn std::error:Error> >{
    //         Ok(（）)
    // }
    fn from(data: Vec<opcode::Opcode>) -> Result<(), Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        let mut jstack: Vec<u32> = Vec::new();

        for e in data {
            match e {
                Opcode::SHR => match instrs.last_mut() {
                    Some(IR::SHR(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHR(1));
                    }
                },
                Opcode::SHL => match instrs.last_mut() {
                    Some(IR::SHL(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHL(1));
                    }
                },
                Opcode::ADD => {}
                Opcode::SUB => {}
                Opcode::PUTCHAR => {}
                Opcode::GETCHAR => {}
                Opcode::LB => {}
                Opcode::RB => {
                    // 理论代码正常是触发不了这个错误的
                    let j = instrs.pop().ok_or("pop from empty list")?;
                    // println!()
                }
            }
        }
        Ok(())
        // Ok(Code {
        //
        // })
    }
}

fn main() {
    // opcode::Code
    println!("Hello, world!");
}
