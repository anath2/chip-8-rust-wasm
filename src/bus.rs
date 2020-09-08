/// BUS communicates between CPU, IO devices and Memory

use crate::memory::Ram;
use crate::keypad::Keypad;
use crate::display::Display;


pub struct Bus {
    ram: Ram,
    sound: bool,
    keypad: Keypad,
    display: Display,
}


impl Bus {

    // New
    pub fn new() -> Bus {
        Bus {
            sound: false,
            ram: Ram::new(),
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }

    // Reset
    pub fn reset(&mut self) {
        self.ram.reset();
        self.display.clrs();
    }

    //Memory:
    pub fn memread(&self, addr: u16) -> u8 {
        self.ram.memread(addr)
    }

    pub fn memwrite(&mut self, addr: u16, val: u8) {
        self.ram.memwrite(addr, val)
    }

    // Display
    pub fn draw(&mut self, x: u8, y: u8, addr: u16, sprite_len: u16) -> bool{
        let mut sprite: Vec<u8> = vec![];

        for i in 0..sprite_len {
            let idx  = addr + i as u16;
            let byte = self.memread(idx);
            sprite.push(byte)
        }

        self.display.draw_sprite(x, y, &sprite)
    }

    pub fn clrs(&mut self) {
        self.display.clrs()
    }

    pub fn get_vram(&self) -> Vec<u8> {
        self.display.get_vram()
    }

    // Sound
    pub fn is_sound_on(&self) -> bool {
        self.sound
    }

    pub fn set_sound_on(&mut self) {
        self.sound = true;
    }

    pub fn set_sound_off(&mut self) {
        self.sound = false;
    }

    // Keypad
    pub fn press_key(&mut self, key: Option<u8>) {
        self.keypad.set_pressed_key(key);
    }

    pub fn release_key(&mut self, keycode: u8) {
        if self.keypad.is_key_pressed(keycode) {
            self.keypad.set_pressed_key(None)
        }
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keypad.get_pressed_key()
    }

    pub fn is_key_pressed(&self, keycode: u8) -> bool {
        self.keypad.is_key_pressed(keycode)
    }

}

