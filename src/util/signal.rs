use riff_wave::*;
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;
use std::f32::consts::PI;
use super::*;

pub fn lanczos_interp(s: &[f32], r: f32, a: usize) -> Vec<f32> {
    let sx = |x: f32| {
        let mut acc = 0.0;
        let lo = (x.floor() - a as f32 + 1.0).max(0.0) as usize;
        let hi = ((x.floor() + a as f32) as usize).min(s.len() - 1);
        for i in lo..hi {
            acc += s[i]*lanczos_kernel(x-i as f32, a);
        }
        acc
    };

    let len_out = (s.len() as f32 * r).ceil() as usize;
    let mut b = vec![0.0f32; len_out];
    for j in 0..len_out {
        let t = (j as f32 / len_out as f32) * s.len() as f32;
        b[j] = sx(t);
    }
    b
}

// a typ 2 or 3
fn lanczos_kernel(x: f32, a: usize) -> f32 {
    if x == 0.0 {
        1.0
    } else if x.abs() < a as f32 {
        let af = a as f32;
        let pi_x = PI*x;
        af*(pi_x.sin())*((pi_x/af).sin())/(pi_x*pi_x)
    } else {
        0.0
    }
}

pub trait Wav {
    fn load(path: &str) -> (u32, Self);
    fn save(&self, fs: u32, path: &str);
}

impl Wav for Vec<Vec2> {
    fn load(path: &str) -> (u32, Self) {
        let (sample_rate, samples) = load_wav(path).expect("failed to load path");
        (sample_rate, samples.into_iter().map(|x| vec2(x, 0.0)).collect())
    }
    fn save(&self, fs: u32,  path: &str) {
        write_wav(path, fs, self.clone().into_iter().map(|x| x.x)) // assume cart?
    }
}

pub fn load_wav(path: &str) -> Option<(u32, Vec<f32>)> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut wav_data = Vec::new();

    let wave_reader = riff_wave::WaveReader::new(&mut file).ok()?;
    let num_channels = wave_reader.pcm_format.num_channels;
    let sample_rate = wave_reader.pcm_format.sample_rate;
    let bits_per_sample = wave_reader.pcm_format.bits_per_sample;

    if num_channels == 1 {
        if bits_per_sample == 16 {
            let mut buffer = [0; 2]; // 16-bit buffer
            while let Ok(()) = file.read_exact(&mut buffer) {
                let sample_i16 = i16::from_le_bytes(buffer); // Convert little-endian bytes to i16
                let sample_f32 = sample_i16 as f32 / i16::MAX as f32;
                wav_data.push(sample_f32);
            }
            return Some((sample_rate, wav_data));
        }
    }
    None
}

pub fn write_wav(outfile: &str, sample_rate: u32, samples: impl Iterator<Item = f32>) {
    let file = File::create(outfile).unwrap();
	let writer = BufWriter::new(file);
	let mut wave_writer = WaveWriter::new(1, sample_rate, 16, writer).unwrap();
    for s in samples {
        wave_writer.write_sample_i16((s * i16::MAX as f32) as i16).unwrap();
    }
}

#[test]
pub fn test_save_load() {
    let fs = 44100u32;
    let samples: Vec<_> = (0..1000).map(|x| x as f32 / fs as f32).collect();
    write_wav("test_save_load.wav", fs, samples.clone().into_iter());
    std::thread::sleep(std::time::Duration::from_millis(10));
    let samples2 = load_wav("test_save_load.wav");
    assert!(samples2.is_some());
    let (sample_rate, samples2) = samples2.unwrap();
    assert_eq!(samples.len(), samples2.len());
    for i in 0..samples.len() {
        assert_eq!(samples[i], samples2[i]);
    }
}