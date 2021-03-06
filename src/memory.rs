/// CHIP-8 Memory

pub const MEM_SIZE: usize = 4096;

const FONT_SPRITES: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub struct Ram { mem: [u8; MEM_SIZE] }


impl Ram {

    pub fn new() -> Ram {
        let mut ram = Ram { mem: [0u8; MEM_SIZE] };
        ram.mem[0..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);
        ram
    }

    pub fn reset(&mut self){
        self.mem = [0u8; MEM_SIZE];
        self.mem[0..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);
    }

    pub fn memwrite(&mut self, addr: u16, byte: u8) {
        self.mem[addr as usize] = byte;
    }

    pub fn memread(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

}



