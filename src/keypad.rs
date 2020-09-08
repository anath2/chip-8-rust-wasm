/// CHIP 8 console keypad consists of 16 buttons, each represented
/// hexadecimal digits 0 - F in memory


pub struct Keypad { key_pressed: Option<u8> }


impl Keypad {

    pub fn new() -> Keypad {
        Keypad {key_pressed: None}
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.key_pressed
    }

    pub fn set_pressed_key(&mut self, key: Option<u8>)  {
        self.key_pressed = key
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        match self.key_pressed {
            Some(k) => k == key_code,
            _ => false
        }
    }

}


