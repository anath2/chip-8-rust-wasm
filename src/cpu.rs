/// CHIP 8 CPU

use crate::utils;
use crate::bus::Bus;


// Mem from 0 to 0x199 is reserved
pub const PROG_START: u16 = 0x200u16;


pub struct Cpu {
    st: u8,           // Sound timer
    dt: u8,           // delay timer
    pc: u16,          // Program counter
    idx: u16,         // Memory index
    v: [u8; 16],      // data registers
    stack: Vec<u16>   // Stack for macros and procedures
}


enum ProgramCounterKind{
    Next,
    Skip,
    Jump(u16)
}


impl Cpu {

    pub fn new() -> Cpu {

        Cpu {
            st: 0,
            dt: 0,
            idx: 0,
            v: [0; 16],
            pc: PROG_START,
            stack: Vec::<u16>::new()
        }

    }

    // Reset all stored registers
    pub fn reset(&mut self) {
        self.st = 0u8;
        self.dt = 0u8;
        self.idx = 0u16;
        self.v  = [0u8; 16];
        self.pc = PROG_START;
        self.stack = Vec::<u16>::new();
    }

    // Execute one cpu cycle
    pub fn tick(&mut self, bus: &mut Bus) {
        utils::log_str("REGISTERS V0 - v16");
        utils::log_u8_array(&self.v);
        utils::log_str("MEMORY INDEX");
        utils::log_u16(self.idx);
        utils::log_str("MEMORY BLOCK");
        utils::log_u8(bus.memread(self.idx));
        utils::log_str("STACK");
        utils::log_u16_array(&self.stack);

        let hi = bus.memread(self.pc) as u16;
        let lo = bus.memread(self.pc + 1) as u16;

        // Addresses are stored big endian
        let opcode = (hi << 8) | lo;
        utils::log_str("OPCODE");
        utils::log_u16(opcode);

        // Decrement delay timer
        if self.dt > 0 {
            self.dt -= 1;
        }

        // Make sound and decrement sound timer
        if self.st > 0 {

            if !bus.is_sound_on() {
                bus.set_sound_on();
            }

            self.st -= 1
        } else {
            bus.set_sound_off();
        }

        match self.execute_instruction(opcode, bus) {
            ProgramCounterKind::Next => self.pc += 2,
            ProgramCounterKind::Skip => self.pc += 4,
            ProgramCounterKind::Jump(n) => self.pc = n
        };
    }

    // Legend:
    // NNN - 2-byte (used for mem address);
    // NN - 1-byte constant (used for data)
    // N - 4-bit constant
    // X, Y - 4-bit register id
    // PC: Program counter
    // I: Instruction pointer
    // VN: One of V0 to VF data register
    fn execute_instruction(
        &mut self,
        opcode: u16,
        bus: &mut Bus
    ) -> ProgramCounterKind {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;

        let p1 = opcode & 0x000F;
        let p2 = (opcode & 0x00F0) >> 4;
        let p3 = (opcode & 0x0F00) >> 8;
        let p4 = (opcode & 0xF000) >> 12;

        match (p4, p3, p2, p1) {
            (0, 0, 0xE, 0) => self.call_00e0(bus),
            (0, 0, 0xE, 0xE) => self.call_00ee(),
            (0x1, _, _, _) => self.call_1nnn(nnn),
            (0x2, _, _, _) => self.call_2nnn(nnn),
            (0x3, _, _, _) => self.call_3xnn(x, nn),
            (0x4, _, _, _) => self.call_4xnn(x, nn),
            (0x5, _, _, 0) => self.call_5xy0(x, y),
            (0x6, _, _, _) => self.call_6xnn(x, nn),
            (0x7, _, _, _) => self.call_7xnn(x, nn),
            (0x8, _, _, 0) => self.call_8xy0(x, y),
            (0x8, _, _, 1) => self.call_8xy1(x, y),
            (0x8, _, _, 2) => self.call_8xy2(x, y),
            (0x8, _, _, 3) => self.call_8xy3(x, y),
            (0x8, _, _, 4) => self.call_8xy4(x, y),
            (0x8, _, _, 5) => self.call_8xy5(x, y),
            (0x8, _, _, 6) => self.call_8xy6(x, y),
            (0x8, _, _, 7) => self.call_8xy7(x, y),
            (0x8, _, _, 0xE) => self.call_8xye(x, y),
            (0x9, _, _, 0) => self.call_9xy0(x, y),
            (0xA, _, _, _) => self.call_annn(nnn),
            (0xB, _, _, _) => self.call_bnnn(nnn),
            (0xC, _, _, _) => self.call_cxnn(x, nn),
            (0xD, _, _, _) => self.call_dxyn(bus, x, y, n),
            (0xE, _, 9, 0xE) => self.call_ex9e(bus, x),
            (0xE, _, 0xA, 1) => self.call_exa1(bus, x),
            (0xF, _, 0, 7) => self.call_fx07(x),
            (0xF, _, 0, 0xA) => self.call_fx0a(bus, x),
            (0xF, _, 1, 5) => self.call_fx15(x),
            (0xF, _, 1, 8) => self.call_fx18(x),
            (0xF, _, 1, 0xE) => self.call_fx1e(x),
            (0xF, _, 2, 9) => self.call_fx29(x),
            (0xF, _, 3, 3) => self.call_fx33(bus, x),
            (0xF, _, 5, 5) => self.call_fx55(bus, x),
            (0xF, _, 6, 5) => self.call_fx65(bus, x),
            (_, _, _, _) => ProgramCounterKind::Next
        }
    }

    // CLRS
    fn call_00e0(&mut self, bus: &mut Bus) -> ProgramCounterKind {
        utils::log_str("CLRS");
        bus.clrs();
        ProgramCounterKind::Next
    }

    // RET
    fn call_00ee(&mut self) -> ProgramCounterKind {
        let addr = self.stack.pop().unwrap();
        utils::log_str(&format!("RET {:x}", addr));
        ProgramCounterKind::Jump(addr)
    }

    // JMP
    fn call_1nnn(&mut self, nnn: u16) -> ProgramCounterKind {
        utils::log_str(&format!("JMP {:x}", nnn));
        ProgramCounterKind::Jump(nnn)
    }

    // CALL
    fn call_2nnn(&mut self, nnn: u16) -> ProgramCounterKind {
        let curr = self.pc + 2;
        self.stack.push(curr);
        utils::log_str(&format!("CALL {:x}", nnn));
        ProgramCounterKind::Jump(nnn)
    }

    // SE Vx KK
    fn call_3xnn(&mut self, x: usize, nn: u8) -> ProgramCounterKind {
        utils::log_str(&format!("SE v{:x} {:x}", x, nn));
        if self.v[x] == nn {ProgramCounterKind::Skip}
        else {ProgramCounterKind::Next}
    }

    // SNE Vx KK
    fn call_4xnn( &mut self, x: usize, nn: u8) -> ProgramCounterKind {
        utils::log_str(&format!("SNE v{:x} {:x}", x, nn));
        if self.v[x] == nn {ProgramCounterKind::Next}
         else {ProgramCounterKind::Skip}
    }

    // SE Vx Vy
    fn call_5xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("SE v{:x} v{:x}", x, y));
        if self.v[x] == self.v[y] {ProgramCounterKind::Skip}
        else {ProgramCounterKind::Next}
    }

    // LD Vx NN
    fn call_6xnn(&mut self, x: usize, nn: u8) -> ProgramCounterKind {
        utils::log_str(&format!("LD v{:x} {:x}", x, nn));
        self.v[x] = nn;
        ProgramCounterKind::Next
    }

    // ADD Vx NN
    fn call_7xnn(&mut self, x: usize, nn: u8) -> ProgramCounterKind {
        utils::log_str(&format!("ADD v{:x} {:x}", x, nn));
        let (sum, _) = self.v[x].overflowing_add(nn);
        self.v[x] = sum;
        ProgramCounterKind::Next
    }

    // LD Vx = Vy
    fn call_8xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD v{:x} v{:x}", x, y));
        self.v[x] = self.v[y] ;
        ProgramCounterKind::Next
    }

    // Vx = Vx | Vy
     fn call_8xy1(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("OR v{:x} v{:x}", x, y));
        self.v[x] = self.v[x] | self.v[y];
        ProgramCounterKind::Next
    }

    // Vx = Vx & Vy
    fn call_8xy2(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("AND v{:x} v{:x}", x, y));
        self.v[x] = self.v[x] & self.v[y];
        ProgramCounterKind::Next
    }

    // Vx = Vx ^ Vy
    fn call_8xy3(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("XOR v{:x} v{:x}", x, y));
        self.v[x] = self.v[x] ^ self.v[y];
        ProgramCounterKind::Next
    }

    // Vx = Vx + Vy; Vf = carry
    fn call_8xy4(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
        utils::log_str(&format!("ADD v{:x} v{:x}", x, y));
        utils::log_str(&format!("LD vf {:x}", carry as u8));
        self.v[x] = sum;
        self.v[0xf] = if carry {1} else {0};
        ProgramCounterKind::Next
    }

    // Vx = Vx - Vy; Vf = borrow
    fn call_8xy5(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        let (sub, borrow) = self.v[x].overflowing_sub(self.v[y]);
        utils::log_str(&format!("SUB v{:x} v{:x}", x, y));
        utils::log_str(&format!("LD vf {:x}", borrow as u8));
        self.v[x] = sub;
        self.v[0xf] = if borrow {0} else {1};
        ProgramCounterKind::Next
    }

    // SHR Vx
    fn call_8xy6(&mut self, x: usize, _y: usize) -> ProgramCounterKind {
        let lsb = self.v[x] & 0x1;
        utils::log_str(&format!("SHR v{:x}", x));
        utils::log_str(&format!("LD vf {:x}", lsb));
        self.v[0xF] = lsb;
        self.v[x] >>= 1;
        ProgramCounterKind::Next
    }

    // Vx = Vy - Vx; Vf = Borrow
    fn call_8xy7(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        let (sub, borrow) = self.v[y].overflowing_sub(self.v[x]);
        utils::log_str(&format!("SUB v{:x} v{:x}", y, x));
        utils::log_str(&format!("LD vf {:x}", borrow as u8));
        self.v[x] = sub;
        self.v[0xf] = if borrow {0} else {1};
        ProgramCounterKind::Next
   }

    // SHL Vx
    fn call_8xye(&mut self, x: usize, _y: usize) -> ProgramCounterKind {
        let msb = self.v[x] & 0x80;
        utils::log_str(&format!("SHL v{:x}", x));
        utils::log_str(&format!("LD vf {:x}", msb));
        self.v[0xF] = msb;
        self.v[x] <<= 1;
        ProgramCounterKind::Next
    }

    // SNE Vx = Vy
    fn call_9xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
        utils::log_str(&format!("SNE v{:x} v{:x}", x, y));
        if self.v[x] == self.v[y] {ProgramCounterKind::Next}
        else {ProgramCounterKind::Skip}
    }

    // i = nnn
    fn call_annn(&mut self, nnn: u16) -> ProgramCounterKind {
        utils::log_str(&format!("LD idx {:x}", nnn));
        self.idx = nnn;
        ProgramCounterKind::Next
    }

    // JMP V0 + nnn
    fn call_bnnn(&mut self, nnn: u16) -> ProgramCounterKind {
        utils::log_str(&format!("JMP v0 + {:x}", nnn));
        ProgramCounterKind::Jump(self.v[0] as u16 + nnn)
    }

    // Vx = RND & nnn
    fn call_cxnn(&mut self, x: usize, nn: u8) -> ProgramCounterKind {
        let v = utils::get_random_u8();
        utils::log_str(&format!("LD v{:x} RND + {:x}", x, nn));
        self.v[x] = v & nn;
        ProgramCounterKind::Next
    }

    // DRAW x y n
    fn call_dxyn(&mut self, bus: &mut Bus, x: usize, y: usize, n: u8) -> ProgramCounterKind {
        utils::log_str(&format!("DRAW v{:x} v{:x} {:x}", x, y, n));
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];
        let collision = bus.draw(vx, vy, self.idx, n as u16);
        self.v[0xf] = if collision {1} else {0};
        ProgramCounterKind::Next
    }

    // SKIP if Keypressed
    fn call_ex9e(&self, bus: &Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("SE KEYPRESS v{:x}", x));

        match bus.get_pressed_key() {
            None => ProgramCounterKind::Next,
            Some(k) => if k == self.v[x] {ProgramCounterKind::Skip}
                                    else {ProgramCounterKind::Next}
        }
    }

    // SKIP if !Keypressed
    fn call_exa1(&mut self, bus: &Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("SNE KEYPRESS v{:x}", x));
        match bus.get_pressed_key() {
            None => ProgramCounterKind::Next,
            Some(k) => if k == self.v[x] {ProgramCounterKind::Next}
                                    else {ProgramCounterKind::Skip}
        }
    }

    // Vx = Delay
    fn call_fx07(&mut self, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD v{:x} dt", x));
        self.v[x] = self.dt;
        ProgramCounterKind::Next
    }

    // Await Vx keypress
    fn call_fx0a(&self, bus: &Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("AWAIT KEYPRESS v{:x}", x));
        match bus.get_pressed_key() {
            None => ProgramCounterKind::Jump(self.pc), // Loop
            Some(k) => if k == self.v[x] {ProgramCounterKind::Next}
                                    else {ProgramCounterKind::Jump(self.pc)}
        }
    }

    // Delay = Vx
    fn call_fx15(&mut self, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD dt v{:x}", x));
        self.dt = self.v[x];
        ProgramCounterKind::Next
    }

    // Sound = Vx
    fn call_fx18(&mut self, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD st v{:x}", x));
        self.st = self.v[x];
        ProgramCounterKind::Next
    }

    // i = ADD Vx i
    fn call_fx1e(&mut self, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("ADD v{:x} i", x));
        let (sum, _) = (self.v[x] as u16).overflowing_add(self.idx);
        self.idx = sum;
        ProgramCounterKind::Next
    }

    // I = Sprite_addr
    fn call_fx29(&mut self, x: usize) -> ProgramCounterKind {
        let sprite_addr = (self.v[x] * 5) as u16;
        utils::log_str(&format!("LD i SPRITE_ADDR {:x}", sprite_addr));
        self.idx = sprite_addr;
        ProgramCounterKind::Next
    }

    // I..i + 2 = BCD(Vx)
    fn call_fx33(&self, bus: &mut Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD I v{:x} / 100", x));
        utils::log_str(&format!("LD I + 1 (v{:x} / 10) % 10", x));
        utils::log_str(&format!("LD I + 2 (v{:x} % 100) % 10", x));
        bus.memwrite(self.idx, self.v[x] / 100);
        bus.memwrite(self.idx + 1, (self.v[x] / 10) % 10);
        bus.memwrite(self.idx + 2, (self.v[x] % 100) % 10);
        ProgramCounterKind::Next
    }

    // MEM = V0..Vx
    fn call_fx55(&self, bus: &mut Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("DUMP v0 ..v{:x}", x));

        for v in 0..=x {
            bus.memwrite(self.idx + v as u16, self.v[v]);
        }

        ProgramCounterKind::Next
    }

    // V0..Vx = MEM
    fn call_fx65(&mut self, bus: &Bus, x: usize) -> ProgramCounterKind {
        utils::log_str(&format!("LD v0 ..v{:x}", x));

        for v in 0..=x {
            self.v[v] = bus.memread(self.idx + v as u16);
        }

        ProgramCounterKind::Next
    }

}


