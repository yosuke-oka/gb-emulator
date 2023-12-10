mod channel1;
mod channel2;
mod channel3;
mod channel4;

const WAVE_DUTY: [[f32; 8]; 4] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], // 12.5%
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0], // 25%
    [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], // 50%
    [0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], // 75%
];

trait Channel {
    fn emulate_t_cycle(&mut self);
    fn dac_output(&self) -> f32;
    fn read_nrxx(&self, x: u16) -> u8;
    fn write_nrxx(&mut self, x: u16, val: u8);
}

pub struct Apu {
    enabled: bool,
    nr50: u8,
    nr51: u8,
    cycles: u128,
    fs: u8,
    channel1: channel1::Channel1,
    channel2: channel2::Channel2,
    channel3: channel3::Channel3,
    channel4: channel4::Channel4,
    //samples: Box<[f32; SAMPLES * 2]>,
    //sample_idx: usize,
    //callback: Option<Rc<dyn Fn(&[f32])>>,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            enabled: false,
            nr50: 0,
            nr51: 0,
            cycles: 0,
            fs: 0,
            channel1: channel1::Channel1::default(),
            channel2: channel2::Channel2::default(),
            channel3: channel3::Channel3::default(),
            channel4: channel4::Channel4::default(),
            //samples: Box::new([0.0; SAMPLES * 2]),
            //sample_idx: 0,
            //callback: None,
        }
    }

    pub fn emulate_cycle(&mut self) {
        for _ in 0..4 {
            self.channel1.emulate_t_cycle();
            self.channel2.emulate_t_cycle();
            self.channel3.emulate_t_cycle();
            self.channel4.emulate_t_cycle();

            // frame sequencer ticks 8192 cycle
            if self.cycles & 0x1FFF == 0 {
                self.channel1.emulate_fs_cycle(self.fs);
                self.channel2.emulate_fs_cycle(self.fs);
                self.channel3.emulate_fs_cycle(self.fs);
                self.channel4.emulate_fs_cycle(self.fs);
                self.cycles = 0;
                self.fs = (self.fs + 1) & 7;
            }
        }
    }
}
