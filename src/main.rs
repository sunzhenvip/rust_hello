mod opcode;
use opcode::{Code, Opcode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0; 1] } //
    }
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let aa: Result<u8, u8>;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

// .\target\debug\hello .\res\hello_world.bf
