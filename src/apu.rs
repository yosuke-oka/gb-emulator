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
