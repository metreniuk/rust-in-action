// TODO: Implement other opcodes https://github.com/rust-in-action/code/blob/1st-edition/ch5/ch5-cpu4/src/main.rs#L25-L47

struct CPU {
    position_in_memory: usize,
    registers: [u8; 16],
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        println!("adding: {} + {}", arg1, arg2);
        let (val, overflowing) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;

        if overflowing {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, address: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        println!("call: {}", address);

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn run(&mut self) {
        loop {
            let op_code = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((op_code & 0xF000) >> 12) as u8;
            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let d = (op_code & 0x000F) as u8;

            let nnn = op_code & 0x0FFF;
            let _kk = (op_code & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {} is not supported", c),
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        memory: [0; 4096],
        registers: [0; 16],
        position_in_memory: 0,
        stack_pointer: 0,
        stack: [0; 16],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 + 10) + (10 + 10) = {}", cpu.registers[0]);
}
