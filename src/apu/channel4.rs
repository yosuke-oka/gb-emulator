use std::cmp::max;

use super::Channel;

#[derive(Default)]
struct Channel4 {
    dac_enabled: bool,
    frequency_timer: u16,
    length_timer: u8,
    length_enabled: bool,
    enabled: bool,
    period: u8,
    period_timer: u8,
    current_volume: u8,
    is_upwards: bool,
    initial_volume: u8,
    lfsr: u16, // Linear Feedback Shift Register
    shift_amount: usize,
    width_mode: bool,
    divisor_code: u16,
}

impl Channel4 {
    fn length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            self.enabled &= self.length_timer > 0;
        }
    }

    fn envelope(&mut self) {
        if self.period != 0 {
            if self.period_timer > 0 {
                self.period_timer -= 1;
            }

            if self.period_timer == 0 {
                self.period_timer = self.period;
                if self.current_volume < 0xF && self.is_upwards {
                    self.current_volume += 1;
                } else if self.current_volume > 0x0 && !self.is_upwards {
                    self.current_volume -= 1;
                }
            }
        }
    }
}

impl Channel for Channel4 {
    fn emulate_t_cycle(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = max(8, self.divisor_code << 4) << self.shift_amount;
            let xor_result = (self.lfsr & 1) ^ ((self.lfsr & 2) >> 1);
            self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
            if self.width_mode {
                self.lfsr &= !(1 << 6);
            } else {
                self.lfsr |= xor_result << 6;
            }
        }
        self.frequency_timer -= 1;
    }

    fn dac_output(&self) -> f32 {
        if self.dac_enabled && self.enabled {
            let ret = (self.lfsr & 1) as f32 * self.current_volume as f32;
            (ret / 7.5) - 1.0
        } else {
            0.0
        }
    }

    fn read_nrxx(&self, x: u16) -> u8 {
        match x {
            2 => (self.initial_volume << 4) | ((self.is_upwards as u8) << 3) | self.period,
            3 => {
                (self.shift_amount as u8) << 4
                    | ((self.width_mode as u8) << 3)
                    | (self.divisor_code as u8)
            }
            4 => ((self.length_enabled as u8) << 6) | 0xBF,
            _ => unreachable!(),
        }
    }

    fn write_nrxx(&mut self, x: u16, val: u8) {
        match x {
            1 => {
                self.length_timer = 64 - (val & 0x3F);
            }
            2 => {
                self.is_upwards = val & 0x08 != 0;
                self.period = val & 0x07;
                self.initial_volume = val >> 4;
                self.dac_enabled = val & 0b11111000 > 0;
                self.enabled &= self.dac_enabled;
            }
            3 => {
                self.shift_amount = (val >> 4) as usize & 0xF;
                self.width_mode = val & 0x8 != 0;
                self.divisor_code = (val & 0x7) as u16;
            }
            4 => {
                self.length_enabled = val & 0x40 != 0;
                if self.length_timer == 0 {
                    self.length_timer = 64;
                }
                let trigger = val & 0x80 != 0;
                if trigger && self.dac_enabled {
                    self.enabled = true;
                }
                if trigger {
                    self.lfsr = 0x7FFF;
                    self.period_timer = self.period;
                    self.current_volume = self.initial_volume;
                }
            }
            _ => unreachable!(),
        }
    }
}
