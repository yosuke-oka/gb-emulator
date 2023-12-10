use crate::gameboy::CPU_CLOCK_HZ;

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

pub const SAMPLE_RATE: u128 = 48000;
pub const SAMPLES: usize = 512;

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
    samples: Box<[f32; SAMPLES * 2]>,
    sample_idx: usize,
    audio: audio::Audio,
}

impl Apu {
    pub fn new(audio: audio::Audio) -> Self {
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
            samples: Box::new([0.0; SAMPLES * 2]),
            sample_idx: 0,
            audio,
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

            if self.cycles % (CPU_CLOCK_HZ / SAMPLE_RATE) == 0 {
                let left_sample = ((((self.nr51 >> 7) & 0b1) as f32) * self.channel4.dac_output()
                    + (((self.nr51 >> 6) & 0b1) as f32) * self.channel3.dac_output()
                    + (((self.nr51 >> 5) & 0b1) as f32) * self.channel2.dac_output()
                    + (((self.nr51 >> 4) & 0b1) as f32) * self.channel1.dac_output())
                    / 4.0;
                let right_sample = ((((self.nr51 >> 3) & 0b1) as f32) * self.channel4.dac_output()
                    + (((self.nr51 >> 2) & 0b1) as f32) * self.channel3.dac_output()
                    + (((self.nr51 >> 1) & 0b1) as f32) * self.channel2.dac_output()
                    + ((self.nr51 & 0b1) as f32) * self.channel1.dac_output())
                    / 4.0;
                self.samples[self.sample_idx * 2] =
                    (((self.nr50 >> 4) & 0x7) as f32 / 7.0) * left_sample;
                self.samples[self.sample_idx * 2 + 1] =
                    ((self.nr50 & 0x7) as f32 / 7.0) * right_sample;
                self.sample_idx += 1;
            }

            if self.sample_idx >= SAMPLES {
                self.audio.queue(self.samples.as_ref());
                self.sample_idx = 0;
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10..=0xFF14 => self.channel1.read_nrxx(addr - 0xFF10),
            0xFF16..=0xFF19 => self.channel2.read_nrxx(addr - 0xFF15),
            0xFF1A..=0xFF1E => self.channel3.read_nrxx(addr - 0xFF1A),
            0xFF20..=0xFF23 => self.channel4.read_nrxx(addr - 0xFF1F),
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => {
                let mut ret = 0;
                ret |= (self.channel1.enabled as u8) << 0;
                ret |= (self.channel2.enabled as u8) << 1;
                ret |= (self.channel3.enabled as u8) << 2;
                ret |= (self.channel4.enabled as u8) << 3;
                ret |= 0x70;
                ret |= (self.enabled as u8) << 7;
                ret
            }
            0xFF30..=0xFF3F => self.channel3.wave_ram[(addr - 0xFF30) as usize],
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, mut val: u8) {
        if !self.enabled
            && ![0xFF11, 0xFF16, 0xFF1B, 0xFF20, 0xFF26].contains(&addr)
            && !(0xFF30..=0xFF3F).contains(&addr)
        {
            return;
        }

        if !self.enabled && [0xFF11, 0xFF16, 0xFF20].contains(&addr) {
            val &= 0b0011_1111;
        };

        match addr {
            0xFF24 => self.nr50 = val,
            0xFF25 => self.nr51 = val,
            0xFF26 => {
                let enabled = val & 0x80 > 0;
                if !enabled && self.enabled {
                    for addr in 0xFF10..=0xFF25 {
                        self.write(addr, 0x00);
                    }
                } else if enabled && !self.enabled {
                    self.fs = 0;
                    self.channel1.wave_duty_position = 0;
                    self.channel2.wave_duty_position = 0;
                    self.channel3.wave_duty_position = 0;
                }
                self.enabled = enabled;
            }
            0xFF10..=0xFF14 => self.channel1.write_nrxx(addr - 0xFF10, val),
            0xFF15..=0xFF19 => self.channel2.write_nrxx(addr - 0xFF15, val),
            0xFF1A..=0xFF1E => self.channel3.write_nrxx(addr - 0xFF1A, val),
            0xFF1F..=0xFF23 => self.channel4.write_nrxx(addr - 0xFF1F, val),
            0xFF30..=0xFF3F => self.channel3.wave_ram[(addr - 0xFF30) as usize] = val,

            _ => unreachable!(),
        }
    }
}
