pub struct CPU {
    pub accumulator: u8,
    pub x: u8,
    pub status: u8,
    pub counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            status: 0,
            counter: 0,
            x = 0,
        }
    }

    fn readmem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn writemem(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.counter = 0x8000;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }




    fn lda(&mut self, value: u8) {
        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);

    }

    fn tax(&mut self) {
        self.x = self.accumulator;
        self.update_zero_and_negative_flags(self.x);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.counter = 0;
        loop {
            let op = program[self.counter as usize];
            self.counter += 1;
            match op {
                0xA9 => {
                    let param = program[self.counter as usize];
                    self.counter += 1;
                    self.lda(param);
                }

                0xAA => {
                    self.tax();
                }


                0x00 => {
                    return;
                }

                0xe8 => self.inx();

                _ => todo!()


            }
        }
       
}
