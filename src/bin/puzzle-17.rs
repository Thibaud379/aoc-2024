#![allow(dead_code)]
use std::{env, fs::File as FileFs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-17.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let mut raw_data = String::new();
    let Ok(mut file) = FileFs::open(args[2].clone()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    file.read_to_string(&mut raw_data).unwrap();

    let data = parse_input(&raw_data);
    let res: PuzzleResult = match args[1].as_str() {
        "1" => part1(data),
        "2" => part2(data),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-17.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{}`!", res);
}

#[allow(non_camel_case_types)]
type u3 = u8;
type PuzzleResult = String;
type RegType = u64;

fn print_result(res: &Vec<u3>) -> String {
    let mut r = res.into_iter().fold(String::new(), |mut acc: String, d| {
        acc.push_str(&d.to_string());
        acc.push(',');
        acc
    });
    if r.len() > 0 {
        r.remove(r.len() - 1);
    }
    r
}

#[derive(Clone, Debug)]
struct PuzzleData {
    reg_a: RegType,
    reg_b: RegType,
    reg_c: RegType,
    prog: Vec<u3>,
}

#[derive(Clone, Debug, Copy)]
struct Op {
    code: OpCode,
    operand: u3,
}

impl From<&[u8]> for Op {
    fn from(value: &[u8]) -> Self {
        Op {
            code: value[0].into(),
            operand: value[1],
        }
    }
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

fn remove_text(input: &str) -> &str {
    let colon = input.find(':').unwrap();
    &input[(colon + 2)..]
}
fn parse_input(data: &str) -> PuzzleData {
    let mut lines = data.lines();

    let reg_a = remove_text(lines.next().unwrap()).parse().unwrap();
    let reg_b = remove_text(lines.next().unwrap()).parse().unwrap();
    let reg_c = remove_text(lines.next().unwrap()).parse().unwrap();
    lines.next().unwrap();
    let prog = remove_text(lines.next().unwrap())
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<u3>>();

    PuzzleData {
        reg_a,
        reg_b,
        reg_c,
        prog,
    }
}

#[derive(Clone, Debug)]
struct Vm {
    regs: [RegType; 4],
    prog: Vec<u3>,
    out: Vec<u3>,
}

impl Vm {
    fn new(data: &PuzzleData) -> Self {
        Vm {
            regs: [data.reg_a, data.reg_b, data.reg_c, 0],
            prog: data.prog.clone(),
            out: vec![],
        }
    }

    fn reg_a(&self) -> RegType {
        self.regs[0]
    }
    fn reg_a_mut(&mut self) -> &mut RegType {
        &mut self.regs[0]
    }
    fn reg_b(&self) -> RegType {
        self.regs[1]
    }
    fn reg_b_mut(&mut self) -> &mut RegType {
        &mut self.regs[1]
    }
    fn reg_c(&self) -> RegType {
        self.regs[2]
    }
    fn reg_c_mut(&mut self) -> &mut RegType {
        &mut self.regs[2]
    }
    fn pc(&self) -> RegType {
        self.regs[3]
    }
    fn pc_mut(&mut self) -> &mut RegType {
        &mut self.regs[3]
    }

    fn get_op(&self) -> Option<Op> {
        self.prog
            .get((self.pc() as usize)..(self.pc() as usize + 2))
            .map(From::from)
    }
    fn combo(&self, operand: u3) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a(),
            5 => self.reg_b(),
            6 => self.reg_c(),

            _ => 0,
        }
    }

    // Returns false if the vm halted
    fn step(&mut self) -> Option<Op> {
        let op = self.get_op()?;
        let operand = op.operand;
        let mut move_pc = true;
        match op.code {
            OpCode::Bxl => {
                *self.reg_b_mut() = self.reg_b() ^ operand as u64;
            }
            OpCode::Bxc => {
                *self.reg_b_mut() = self.reg_b() ^ self.reg_c();
            }
            OpCode::Bst => {
                *self.reg_b_mut() = self.combo(operand) & 0b111;
            }
            OpCode::Jnz => {
                if self.reg_a() != 0 {
                    *self.pc_mut() = self.combo(operand);
                    move_pc = false;
                }
            }
            OpCode::Out => {
                self.out.push((self.combo(operand) & 0b111) as u3);
            }
            OpCode::Adv => {
                let res = self.reg_a() >> self.combo(operand);
                *self.reg_a_mut() = res;
            }
            OpCode::Bdv => {
                let res = self.reg_a() >> self.combo(operand);
                *self.reg_b_mut() = res;
            }
            OpCode::Cdv => {
                let res = self.reg_a() >> self.combo(operand);
                *self.reg_c_mut() = res;
            }
        }
        if move_pc {
            *self.pc_mut() += 2;
        }

        Some(op)
    }

    fn run(&mut self) {
        while let Some(_op) = self.step() {}
    }
}

fn part1(data: PuzzleData) -> PuzzleResult {
    let mut vm = Vm::new(&data);
    while let Some(_op) = vm.step() {
        println!("{_op:?}");
    }
    print_result(&vm.out)
}
fn part2(data: PuzzleData) -> PuzzleResult {
    let goal = data.prog.clone();
    let mut past_as = vec![0];
    for i in 0..goal.len() {
        let pas = past_as.clone();
        past_as.clear();
        for past_a in pas {
            for a in 0..8 {
                let mut vm = Vm::new(&data.clone());
                *vm.reg_a_mut() = a | (past_a << 3);
                vm.run();

                if vm.out[..(i + 1)] == goal[goal.len() - 1 - i..] {
                    past_as.push((past_a << 3) + a);
                }
                println!("a: {} | out: {}", a | (past_a << 3), print_result(&vm.out));
            }
        }
    }

    format!("{:?}", past_as.iter().min())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_1: &'static str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EX_2: &'static str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    const EX_3: &'static str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_1() {
        let data_1 = parse_input(EX_1);
        assert_eq!(part1(data_1), "4,6,3,5,6,3,5,2,1,0");
        println!();
        let data_2 = parse_input(EX_2);
        assert_eq!(part1(data_2), "4,2,5,6,7,7,7,7,3,1,0");
    }
    #[test]
    fn test_2() {
        let data_3 = parse_input(EX_3);
        assert_eq!(part2(data_3), "117440");
    }
}
