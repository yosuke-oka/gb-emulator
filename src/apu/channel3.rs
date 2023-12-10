use std::cmp::min;

use super::Channel;

#[derive(Default)]
pub struct Channel3 {
    dac_enabled: bool,
    frequency: u16,
    frequency_timer: u16,
    pub wave_duty_position: usize,
    length_timer: u16,
    length_enabled: bool,
    pub enabled: bool,
    pub wave_ram: [u8; 16],
    volume_shift: u8,
    output_level: u8,
}
impl Channel3 {
    pub fn emulate_fs_cycle(&mut self, fs: u8) {
        if fs & 1 == 0 {
            self.length();
        }
    }

    fn length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            self.enabled &= self.length_timer > 0;
        }
    }
}

impl Channel for Channel3 {
    fn emulate_t_cycle(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 2;
            self.wave_duty_position = (self.wave_duty_position + 1) & 31;
        }
        self.frequency_timer -= 1;
    }

    fn dac_output(&self) -> f32 {
        if self.dac_enabled && self.enabled {
            let ret = (0xF
                & (self.wave_ram[self.wave_duty_position >> 1]
                    >> ((self.wave_duty_position & 1) << 2))
                    >> self.volume_shift) as f32;
            (ret / 7.5) - 1.0
        } else {
            0.0
        }
    }

    fn read_nrxx(&self, x: u16) -> u8 {
        match x {
            0 => ((self.dac_enabled as u8) << 7) | 0x7F,
            2 => (self.output_level << 5) | 0x9F,
            4 => ((self.length_enabled as u8) << 6) | 0xBF,
            _ => unreachable!(),
        }
    }
    fn write_nrxx(&mut self, x: u16, val: u8) {
        match x {
            0 => {
                self.dac_enabled = val & 0x80 != 0;
                self.enabled &= self.dac_enabled;
            }
            1 => self.length_timer = 256 - val as u16,
            2 => {
                self.output_level = (val >> 5) & 0x03;
                self.volume_shift = min(4, self.output_level.wrapping_sub(1));
            }
            3 => self.frequency = (self.frequency & 0x700) | val as u16,
            4 => {
                self.frequency = (self.frequency & 0xFF) | (((val & 0x7) as u16) << 8);
                self.length_enabled = val & 0x40 != 0;
                if self.length_enabled {
                    self.length_timer = 256;
                }
                let trigger = val & 0x80 != 0;
                if trigger && self.dac_enabled {
                    self.enabled = true;
                }
            }
            _ => unreachable!(),
        }
    }
}
