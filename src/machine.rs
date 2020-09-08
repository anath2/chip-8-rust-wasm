/// CHIP8 Entry point

use std::fmt;
use crate::cpu;
use crate::utils;
use crate::bus::Bus;
extern crate web_sys;
use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[wasm_bindgen]
pub struct Console {
    bus: Bus,
    cpu: cpu::Cpu,
}


#[wasm_bindgen]
impl Console {

    pub fn new() -> Console {
        utils::set_panic_hook();
        Console { bus: Bus::new(), cpu: cpu::Cpu::new() }
    }

    // Render human readable display
    pub fn render(&self) -> String {
        self.to_string()
    }

    // Loads ROM into memory
    pub fn load_rom(&mut self, rom: &[u8]) {
        let start_addr = cpu::PROG_START;

        for (idx, byte) in rom.iter().cloned().enumerate() {
            self.bus.memwrite(start_addr + idx as u16, byte);
        }
    }

    // Resets machine
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }

    // Execute Cycle
    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
   }

    // Gets display memory
    pub fn get_vram(&self) -> Vec<u8> {
        self.bus.get_vram()
    }

    // Check whether to beep or not
    pub fn beep(&self) -> bool {
        self.bus.is_sound_on()
    }

    // Press key on keypad
    pub fn press_key(&mut self, keycode: u8) {
        let key = Some(keycode);
        self.bus.press_key(key)
    }

    // Resets keypad if key is pressed
    pub fn release_key(&mut self, keycode: u8) {
        self.bus.release_key(keycode);
    }

    // Get pressed key
    pub fn get_pressed_key(&self) -> Option<u8> {
        self.bus.get_pressed_key()
    }

}

impl fmt::Display for Console {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_width = 64;
        let sound = if self.beep() {"ON"} else {"OFF"};

        let pressed_key = match self.get_pressed_key() {
            None => String::from("None"),
            Some(k) => format!("{:x}", k)
        };

        for row in self.get_vram().as_slice().chunks(display_width as usize) {
            for &px in row {
                let fill = if px == 0 { '\u{25a0}' } else { '\u{25a1}' };
                write!(f, "{}", fill)?;
            }

            write!(f, "\n")?
        }

        write!(f, "KEY - {}\n", pressed_key)?;
        write!(f, "SND - {}\n", sound)?;
        Ok(())
    }
}
