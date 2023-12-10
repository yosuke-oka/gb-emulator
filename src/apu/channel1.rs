use super::{Channel, WAVE_DUTY};
use std::cmp::min;

#[derive(Default)]
struct Channel1 {
    dac_enabled: bool,
    frequency: u16,
    frequency_timer: u16,
    wave_duty_position: usize,
    wave_duty_pattern: u8,
    length_timer: u8,
    length_enabled: bool,
    enabled: bool,
    period: u8,
    period_timer: u8,
    current_volume: u8,
    is_upwards: bool,
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_timer: u8,
    sweep_shift: u8,
    is_decrementing: bool,
    shadow_frequency: u16,
    initial_volume: u8,
}

impl Channel1 {
    // 一定時間後にチャンネルを無効にする
    fn length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            self.enabled &= self.length_timer > 0;
        }
    }

    // 一定時間ごとに振幅を増減させる
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

    // 一転時間ごとに周波数を増減させる
    fn sweep(&mut self) {
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }

        if self.sweep_timer == 0 {
            self.sweep_timer = self.sweep_period;
            if self.sweep_enabled {
                self.frequency = self.calculate_frequency();
                self.shadow_frequency = self.frequency;
            }
        }
    }

    fn calculate_frequency(&mut self) -> u16 {
        if self.is_decrementing {
            if self.shadow_frequency >= (self.shadow_frequency >> self.sweep_shift) {
                self.shadow_frequency - (self.shadow_frequency >> self.sweep_shift)
            } else {
                0
            }
        } else {
            min(
                0x3FF,
                self.shadow_frequency + (self.shadow_frequency >> self.sweep_shift),
            )
        }
    }
}
impl Channel for Channel1 {
    fn emulate_t_cycle(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 4;
            self.wave_duty_position = (self.wave_duty_position + 1) & 7;
        }
        self.frequency_timer -= 1;
    }

    fn dac_output(&self) -> f32 {
        if self.dac_enabled && self.enabled {
            let ret = WAVE_DUTY[self.wave_duty_pattern as usize][self.wave_duty_position as usize]
                * self.current_volume as f32;
            (ret / 7.5) - 1.0
        } else {
            0.0
        }
    }

    fn read_nrxx(&self, x: u16) -> u8 {
        match x {
            0 => {
                (self.sweep_period << 4)
                    | ((self.is_decrementing as u8) << 3)
                    | self.sweep_shift
                    | 0x80
            }
            1 => (self.wave_duty_pattern << 6) | 0b0011_1111,
            2 => (self.initial_volume << 4) | ((self.is_upwards as u8) << 3) | self.period,
            3 => 0xFF,
            4 => ((self.length_enabled as u8) << 6) | 0b1011_1111,
            _ => unreachable!(),
        }
    }
    fn write_nrxx(&mut self, x: u16, val: u8) {
        match x {
            0 => {
                self.sweep_period = (val >> 4) & 0x07;
                self.is_decrementing = val & 0x08 > 0;
                self.sweep_shift = val & 0x07;
            }
            1 => {
                self.wave_duty_pattern = (val >> 6) & 0b11;
                self.length_timer = 64 - (val & 0x3f);
            }
            2 => {
                self.is_upwards = val & 0x08 > 0;
                self.initial_volume = val >> 4;
                self.period = val & 0x07;
                self.dac_enabled = val & 0b11111000 > 0;
                self.enabled &= self.dac_enabled;
            }
            3 => {
                self.frequency = (self.frequency & 0x0700) | val as u16;
            }
            4 => {
                self.frequency = (self.frequency & 0xFF) | (((val & 0x07) as u16) << 8);
                self.length_enabled = val & 0x40 > 0;
                if self.length_timer == 0 {
                    self.length_timer = 64;
                }
                let trigger = val & 0x80 > 0;
                if trigger && self.dac_enabled {
                    self.enabled = true;
                    self.period_timer = self.period;
                    self.current_volume = self.initial_volume;
                    self.shadow_frequency = self.frequency;
                    self.sweep_timer = self.sweep_period;
                    self.sweep_enabled = self.sweep_period > 0 || self.sweep_shift > 0;
                }
            }
            _ => unreachable!(),
        }
    }
}
