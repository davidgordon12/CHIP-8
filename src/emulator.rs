#![allow(dead_code)]        // allowed for now
#![allow(unused_mut)]       // allowed for now
#![allow(unused_variables)] // allowed for now 
const SCREEN_WIDTH: usize     =  64;
const SCREEN_HEIGHT: usize    =  32;
const NUM_V_REG: usize        =  16;
const NUM_KEYS: usize         =  16;
const RAM_SIZE: usize         =  4096;
const STACK_SIZE: usize       =  16;
const FONTSET_SIZE: usize     =  80;
const MAX_PROGRAM_MEMORY: u16 =  3584;
const PROGRAM_START_ADDR: u16 =  0x200;
const PROGRAM_END_ADDR: u16   =  0xFFF;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    pc: u16,
    sp: u16,
    ram: [u8; RAM_SIZE],
    v: [u8; NUM_V_REG],
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    index: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emulator {
    pub fn new() -> Self {
       let mut emu = Self {
            pc: PROGRAM_START_ADDR,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; NUM_V_REG],
            keys: [false; NUM_KEYS],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            index: 0,
            sp: 0,
            dt: 0,
            st: 0,
        };

        emu.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    pub fn reset(&mut self) {
        self.pc = PROGRAM_START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.stack = [0; STACK_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v = [0; NUM_V_REG];
        self.index = 0;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.keys = [false; NUM_KEYS];
        self.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn cycle(&mut self) {
        // Fetch -> Decode -> Execute
        let op = self.fetch();
        
        // Drop the reference by copying result to opcode
        let opcode = self.decode(op).to_string();

        // Now we can reference *self again, pass opcode as a slice
        self.execute(&opcode[..], op); 
    }

    pub fn fetch(&mut self) -> u16 {
        let hi_byte = self.ram[self.pc as usize] as u16;
        let lo_byte = self.ram[(self.pc + 1) as usize] as u16;

        let opcode = (hi_byte << 8) | lo_byte;
        self.pc += 2;

        opcode
    }

    pub fn decode(&mut self, op: u16) -> &str {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8; 
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;
        match(digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return "0000",
            (0, 0, 0xE, 0) => return "00E0",
            (0, 0, 0xE, 0xE) => return "00E0",
            (1, _, _, _) => return "1nnn",
            (2, _, _, _) => return "2nnn",
            (3, _, _, _) => return "3nnn",
            (4, _, _, _) => return "4xnn",
            (5, _, _, 0) => return "5xy0",
            (6, _, _, _) => return "6xnn",
            (7, _, _, _) => return "7xnn",
            (8, _, _, 0) => return "8xy0",
            (8, _, _, 1) => return "8xy1",
            (8, _, _, 2) => return "8xy2",
            (8, _, _, 3) => return "8xy3",
            (8, _, _, 4) => return "8xy4",
            (8, _, _, 5) => return "8xy5",
            (8, _, _, 6) => return "8xy6",
            (8, _, _, 7) => return "8xy7",
            (8, 0, 0, 0xE) => return "8xyE",
            (9, _, _, 0) => return "9xy0",
            (0xA, _, _, _) => return "Annn",
            (0xB, _, _, _) => return "Bnnn",
            (0xC, _, _, _) => return "Cxnn",
            (0xD, _, _, _) => return "Dxyn",
            (0xE, _, 9, 0xE) => return "Ex93",
            (0xE, _, 0xA, 1) => return "ExA1",
            (0xF, _, 0, 7) => return "Fx07",
            (0xF, _, 0, 0xA) => return "Fx0A",
            (0xF, _, 1, 5) => return "Fx15",
            (0xF, _, 1, 8) => return "Fx18",
            (0xF, _, 1, 0xE) => return "Fx1E",
            (0xF, _, 2, 9) => return "Fx29",
            (0xF, _, 3, 3) => return "Fx33",
            (0xF, _, 5, 5) => return "Fx55",
            (0xF, _, 6, 5) => return "Fx65",
            
            (_, _, _, _) => self.panic("Unimplemented opcode received."),
        }
    }

    pub fn execute(&mut self, opcode: &str, op: u16) {
        match opcode {
            // NOP
            "0000" => return,

            // CLS
            "00E0" =>  { 
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // RET
            "00EE" => {
                self.pc = self.pop();
            }

            // JMP
            "1nnn" => {
                self.pc = op & 0xFFF;
            }

            // CALL
            "2nnn" => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            }

            // SKIP next if VX == NN
            "3xnn" => {
                let x = ((op & 0x0F00) >> 8) as usize; 

                if self.v[x] == (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }

            "4xnn" => {
                let x = ((op & 0x0F00) >> 8) as usize; 

                if self.v[x] != (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }

            "5xy0" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }

            "6xnn" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let val = (op & 0xFF) as u8;

                self.v[x] = self.v[x].wrapping_add(val);
            }

            "7xnn" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let val = (op & 0xFF) as u8;

                self.v[x] += val;
            }

            "8xy0" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                self.v[x] = self.v[y];
            }

            "8xy1" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                self.v[x] |= self.v[y];
            }

            "8xy2" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                self.v[x] &= self.v[y];
            }

            "8xy3" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                self.v[x] ^= self.v[y];
            }

            "8xy4" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                
                let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);

                self.v[x] = sum;
                self.v[0xF] = overflow as u8;
            }

            "8xy5" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                
                let (sum, underflow) = self.v[x].overflowing_sub(self.v[y]);

                let underflow = if underflow { 0 } else { 1 };

                self.v[x] = sum;
                self.v[0xF] = underflow;
            }

            "8xy6" => {
                let x = ((op & 0x0F00) >> 8) as usize;

                let dropoff = self.v[x] & 1;

                self.v[x] >>= 1;
                self.v[0xF] = dropoff;
            }

            "8xy7" => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                
                let (sum, underflow) = self.v[y].overflowing_sub(self.v[x]);

                let underflow = if underflow { 0 } else { 1 };

                self.v[x] = sum;
                self.v[0xF] = underflow;
            }

            "8xy8" => {
                let x = ((op & 0x0F00) >> 8) as usize;

                let dropoff = (self.v[x] >> 7) & 1;

                self.v[x] <<= 1;
                self.v[0xF] = dropoff;
            }



            _ => self.panic("Unimplemented")
        }
    }

    pub fn increment_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }

        if self.st > 0 {
            if self.st == 1 {
                // beep
            }
            self.st -= 1; 
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.sp >= 16 { self.panic("Cannot push onto stack, pointer at 16.") }

        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp <= 0 { self.panic("Cannot pop stack, pointer at 0.") }

        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn panic(&self, info: &str) -> ! {
        println!("Panic: {}", info);
        loop {}
    }
}
