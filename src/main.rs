use crate::oss::Config;

pub mod oss;

fn main() {
    let mut oss = Config::new("/dev/dsp");
    oss.configure();
    let samples = oss.chsamples.try_into().unwrap();
    let v = vec![0 as i32; samples];

    println!("channels = {}", oss.channels);
    println!("bytes = {}", oss.bytes);
    println!("samples = {}", oss.samples);
    println!("chsamples = {}", oss.chsamples);
    println!("vec size = {}", v.len());
}
