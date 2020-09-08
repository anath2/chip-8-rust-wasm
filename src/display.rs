/// Display controls the display for chip_8 console emulator

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const VRAM_SIZE: usize = HEIGHT * WIDTH;


// Display represented by a 1D array of bytes
pub struct Display {
    vram: [u8; VRAM_SIZE],
}


// Get memory address from 2D coords
pub fn get_addr_from_xy(x: usize, y: usize) -> usize {
    (y * WIDTH) + x
}


impl Display {

    // All pixels are set to 0 when display is initialized
    pub fn new() ->  Display {
        Display { vram: [0u8; VRAM_SIZE] }
    }

    // Sets all pixels to false
    pub fn clrs(&mut self) {
        self.vram = [0u8; VRAM_SIZE];
    }

    // Draw sprite from memslice, starting at (x, y), return true if collision
    pub fn draw_sprite(&mut self, x: u8, y: u8, memslice: &[u8]) -> bool {
        let origin_x = x as usize;
        let origin_y = y as usize;
        let sprite_len = memslice.len();

        let mut collision = false;

        for y in 0..sprite_len {
            let byte = memslice[y];

            for x in 0..8 {
                // Since sprites are binary coded
                let px = (byte >> (7 - x)) & 0b000_0001;
                let x_coord = (x + origin_x) % WIDTH;
                let y_coord = (y + origin_y) % HEIGHT;
                let vram_addr = get_addr_from_xy(x_coord, y_coord);
                let prev_px = self.vram[vram_addr];

                self.vram[vram_addr] ^= px;  // Save xor'd value

                if prev_px == 1 && self.vram[vram_addr] == 0 {
                    collision = true;
                }
            }
        }

        collision
    }

    // Get copy of the display buffer
    pub fn get_vram(&self) -> Vec<u8> {
        self.vram.clone().to_vec()
    }
}
