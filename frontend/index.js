import { Console } from "wasm-chip-8";
import { memory } from "wasm-chip-8/chip_8_wasm_bg"


// List of available ROMS
const ROMS = [
    'PONG',
    '15PUZZLE',
    'BLITZ',
    'CONNECT4',
    'HIDDEN',
    'INVADERS',
    'MAZE',
    'MISSILE',
    'SYZYGY',
    'TETRIS',
    'UFO',
    'VERS',
    'BLINKY',
    'BRIX',
    'GUESS',
    'IBM',
    'KALEID',
    'MERLIN',
    'PONG',
    'PUZZLE',
    'TANK',
    'TICTAC',
    'VBRIX',
    'WIPEOFF'
];

// Maps 4 keys for each row from 1 to v
const KEYMAP = {
    49: 0x0, // 1
    50: 0x1, // 2
    51: 0x2, // 3
    52: 0x3, // 4
    81: 0x4, // Q
    87: 0x5, // W
    69: 0x6, // E
    82: 0x7, // R
    65: 0x8, // A
    83: 0x9, // S
    68: 0xa, // D
    70: 0xb, // F
    90: 0xc, // Z
    88: 0xd, // X
    67: 0xe, // C
    86: 0xf  // V
}

const PIXEL_SIZE = 8;
const DISPLAY_WIDTH = 64;
const DISPLAY_HEIGHT = 32;
const PIXEL_ON_COLOR = "#FFFFFF";
const PIXEL_OFF_COLOR = "#000000";


async function run() {

    const machine = Console.new();
    let machineRunning = false;

    function updateDisplay() {
        const displayMemory = new Uint8Array(
            memory.buffer,
            machine.get_vram(),
            2048
        );

        draw(displayMemory);
    }

    function draw(displayMemory) {
        const canvas = document.getElementById("chip-8-screen");
        canvas.width = PIXEL_SIZE * DISPLAY_WIDTH;
        canvas.height = PIXEL_SIZE * DISPLAY_HEIGHT;
        const ctx = canvas.getContext('2d');
        ctx.beginPath();

        for (let y = 0; y < DISPLAY_HEIGHT; y++) {
            for (let x = 0; x < DISPLAY_WIDTH; x++ ) {
                const idx = y * DISPLAY_WIDTH + x;

                ctx.fillStyle = displayMemory[idx] === 1 ? PIXEL_ON_COLOR : PIXEL_OFF_COLOR;
                ctx.fillRect(
                    x * PIXEL_SIZE,
                    y * PIXEL_SIZE,
                    PIXEL_SIZE,
                    PIXEL_SIZE
                );
            }
        }

        ctx.stroke();
    }

    function initKeypad() {
        window.addEventListener('keydown', e => {
            let machineKey = KEYMAP[e.keyCode]

            if (machineKey !== null) {
                machine.press_key(machineKey)
                let keyHtml = document.getElementById("keycode-" + String(machineKey))
                keyHtml.style.backgroundColor = "black";
                keyHtml.style.color = "white";
           }
        })

        window.addEventListener('keyup', e => {
            let machineKey = KEYMAP[e.keyCode]

            if (machineKey !== null) {
                machine.release_key(KEYMAP[e.keyCode])
                let keyHtml = document.getElementById("keycode-" + String(machineKey))
                keyHtml.style.backgroundColor = null;
                keyHtml.style.color = null;
            }
        })
    }

    function init() {
        loadRom('PONG');
        let romSelector = document.getElementById('rom-select');

        ROMS.forEach(r => {
            let opt = document.createElement('option');
            opt.value = r;
            opt.innerHTML = r;
            romSelector.appendChild(opt)
        });

        romSelector.addEventListener('change', () => {
            machineRunning = false;
            machine.reset();
            updateDisplay();
            loadRom(romSelector.value)
        });

        let playPauseButton = document.getElementById('run');
        let stepButton = document.getElementById('step');
        let resetButton = document.getElementById('reset');

        stepButton.addEventListener("click", e => {if (!machineRunning) {tick()}})
        resetButton.addEventListener("click", e => {machine.reset()})

        playPauseButton.addEventListener("click", e => {
            if (machineRunning) {
                machineRunning = false
            } else {
                machineRunning = true;
                window.requestAnimationFrame(runLoop);
            }
        });

        initKeypad();
        updateDisplay();
    }

    function loadRom(rom_name) {
        if (!rom_name) return

        fetch(`roms/${rom_name}`)
        .then(i => i.arrayBuffer())
        .then(buffer => {
            let rom = new Uint8Array(buffer)
            machine.load_rom(rom)
        });
    }

    function runLoop() {
        if (machineRunning) {
            tick()
            window.requestAnimationFrame(runLoop)
        }
    }

    function tick() {
        machine.tick()
        updateDisplay();
    }

    init();
}


run();