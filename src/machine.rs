/// CHIP8 Entry point

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
    pub fn get_vram(&self) -> *const u8 {
        self.bus.get_vram().as_ptr()
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
