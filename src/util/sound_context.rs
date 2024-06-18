use ringbuf::*;
use cpal::traits::*;
use std::path::Path;
use crate::util::*;
use riff_wave::*;
use std::io::Read;

pub fn load_wav(path: &Path) -> Option<(u32, Vec<f32>)> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut wav_data = Vec::new();

    let wave_reader = riff_wave::WaveReader::new(&mut file).ok()?;
    let num_channels = wave_reader.pcm_format.num_channels;
    let sample_rate = wave_reader.pcm_format.sample_rate;
    let bits_per_sample = wave_reader.pcm_format.bits_per_sample;

    // if num_channels == 2 {
        if bits_per_sample == 16 {
            let mut buffer = [0; 2]; // 16-bit buffer
            while let Ok(()) = file.read_exact(&mut buffer) {
                let sample_i16 = i16::from_le_bytes(buffer); // Convert little-endian bytes to i16
                let sample_f32 = sample_i16 as f32 / i16::MAX as f32;
                wav_data.push(sample_f32);
            }
            return Some((sample_rate, wav_data));
        }
    // }
    None
}

#[derive(Debug)]
pub struct PlayCommand {
    pub asset: usize,
    pub limit: usize,
    pub vol: f32,
    pub music: bool, // currently ignored
    pub clear_all: bool,
}
impl Default for PlayCommand {
    fn default() -> Self {
        Self {
            asset: 0,
            limit: 0,
            vol: 1.0,
            music: false,
            clear_all: false,
        }
    }
}
pub struct PlaybackContext {
    n: usize,
    free: bool,
    com: PlayCommand,
}
struct AudioThread {
    sounds: Vec<Vec<f32>>,
    mixer: Vec<PlaybackContext>,
    cons: Consumer<PlayCommand>,
    cons_samp: Consumer<f32>,
    n: i64,
}
impl AudioThread {
    pub fn new(sounds: Vec<Vec<f32>>, cons: Consumer<PlayCommand>, cons_samp: Consumer<f32>) -> Self {
        Self {
            sounds,
            cons,
            cons_samp,
            mixer: vec![],
            n: 0,
        }
    }
    // in stereo
    pub fn fill_samples_buffer(&mut self, buf: &mut[f32], info: &cpal::OutputCallbackInfo) {
        self.mix();
        // for chunk in buf.chunks_exact_mut(2) {
            // self.n += 1;
            // for x in chunk {
            for x in buf {
                let x_mix = self.tick_mix();
                let x_buf = self.cons_samp.next().unwrap_or(0.0);
                *x = x_mix*0.5 + x_buf*0.5;
            }
        // };
    }
    pub fn tick_mix(&mut self) -> f32 {
        let mut acc = 0.0;
        self.mixer.iter_mut().for_each(|chan| {
            if !chan.free {
                if let Some(samp) = self.sounds[chan.com.asset].get(chan.n) {
                    acc += samp * chan.com.vol
                } else {
                    chan.free = true;
                }
                chan.n += 1
            }
        });
        acc*0.1 // or do le exp mapping lol. does that do the right thing on sign and is it sign? still dont know
    }
    pub fn mix(&mut self) {
        while let Some(com) = self.cons.next() {
            if com.clear_all {
                self.mixer.clear();
            }
            if com.limit == 0 || com.limit > self.mixer.iter().filter(|x| x.com.asset == com.asset && !x.free).count() {
                // find bucket
                if let Some(n) = self.mixer.iter().position(|x| x.free) {
                    self.mixer[n] = PlaybackContext {
                        n: 0,
                        free: false,
                        com,
                    }
                } else {
                    self.mixer.push(PlaybackContext { 
                        n: 0, 
                        free: false, 
                        com 
                    })
                }
            }
        }
    }
}

pub struct SoundContext {
    prod: Producer<PlayCommand>,
    prod_samp: Producer<f32>,
    stream: cpal::Stream,
}

impl SoundContext {
    pub fn new(device: Option<cpal::Device>, sounds: Vec<Vec<f32>>) -> Self {
        let (prod, cons) = RingBuffer::<PlayCommand>::new(200).split();
        let (prod_samp, cons_samp) = RingBuffer::<f32>::new(48000).split();
        let mut audio_thread = AudioThread::new(sounds, cons, cons_samp);
        let host = cpal::default_host();
        let device = device.unwrap_or(host.default_output_device().expect("Failed to retrieve default output device"));
        println!("Output device : {}", device.name().expect("couldnt get device name (??? idk)"));
        let config = device.default_output_config().expect("failed to get default output config");
        println!("Default output config : {:?}", config);
        let sample_rate = config.sample_rate().0;
        let sample_format = config.sample_format();
        let channels = config.channels();
        let output_callback = move |output: &mut [f32], info: &cpal::OutputCallbackInfo| {
            audio_thread.fill_samples_buffer(output, info);
            // output.chunks_exact_mut(2).for_each(|chunk| {
            //     let x = audio_thread.tick();
            //     chunk[0] = x;
            //     chunk[1] = x;
            // })
        };
        let config = cpal::StreamConfig {
            channels: channels,
            sample_rate: config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };
        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(&config, output_callback, |_| panic!("error"), None),
            _ => panic!("unsupported"),
        }.expect("failed to make stream");
        stream.play().expect("failed to play stream");
        Self {
            prod,
            prod_samp,
            stream,
        }
    }
    pub fn play(&mut self, com: PlayCommand) {
        self.prod.push(com).expect("play failed");
    }
    // send pairs for stereo pls
    pub fn send_samples(&mut self, samples: impl Iterator<Item = f32>) {
        for sample in samples {
            self.prod_samp.push(sample);
        }
    }
}

pub fn load_folder(path: &Path) -> Vec<Vec<f32>> {
    let mut paths = vec![];
    dir_traverse(path, &mut |path| {
        if path.extension().unwrap() == "wav" {
            paths.push(path.to_owned())
        }
    }).expect_with(|| path.to_string_lossy());
    paths.sort();
    dbg!(&paths);
    let bufs: Vec<_> = paths.iter().filter_map(|p| {
        let wav = load_wav(p);
        wav.map(|x| x.1)
    }).collect();
    dbg!(bufs.len());
    bufs
}