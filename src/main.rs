struct Chip8 {
    // 4 KB memory
    memory: [u8; 4096],

    // 16 general-purpose registers
    v: [u8; 16],

    // Index registers
    i: u16,

    // Program Counter
    pc: u16,

    // 16-level stack
    stack: [u16; 16],

    // Stack pointer
    sp: u8,

    // Timers
    delay: u8,
    sound: u8,

    // 64x32 monochrome display
    display: [[bool; 64]; 32],

    // input keys
    keys: [bool; 16],
}

impl Chip8 {
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay: 0,
            sound: 0,
            display: {
                let mut screen = [[false; 64]; 32];
                screen[5][10] = true;
                screen
            },
            keys: [false; 16],
        }
    }
    fn reset(&mut self) {
        self.pc = 0x200;
        self.v = [0; 16];
        self.i = 0;
        self.sp = 0;
    }
}

fn main() {
    let c8 = Chip8::new();

    println!("V0 = {}", c8.v[0]);
    println!("VF = {}", c8.v[15]);

    println!("Pixel (10,5) = {}", c8.display[5][10]);

    println!("PC       = {:#06x}", c8.pc);
    println!("Memory   = {} bytes", c8.memory.len());
    println!("Register = {} buah", c8.v.len());
    println!("Stack    = {} level", c8.stack.len());
    println!("Display  = {} x {} pixels", 64, 32);
}
