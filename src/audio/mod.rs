//! Audio processing module
//! 
//! Handles loading and decoding audio files (WAV, MP3) and providing
//! raw sample data for FFT analysis.

mod decoder;
mod resampler;

pub use decoder::{AudioDecoder, AudioFormat};
pub use resampler::Resampler;

use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::path::Path;

/// Configuration for audio processing
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 2048,
        }
    }
}

/// Stream of audio samples for real-time processing
pub struct AudioStream {
    receiver: Receiver<Vec<f32>>,
    config: AudioConfig,
}

impl AudioStream {
    pub fn new(receiver: Receiver<Vec<f32>>, config: AudioConfig) -> Self {
        Self { receiver, config }
    }

    pub fn receive(&self) -> Option<Vec<f32>> {
        self.receiver.recv().ok()
    }

    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
}

/// Load audio file and return samples
pub fn load_audio<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, AudioConfig)> {
    let decoder = AudioDecoder::new(path.as_ref())?;
    let config = decoder.config().clone();
    let samples = decoder.read_all()?;
    
    Ok((samples, config))
}

/// Start streaming audio from file in a background thread
pub fn stream_audio<P: AsRef<Path> + Send + 'static>(
    path: P,
) -> Result<AudioStream> {
    let (sender, receiver) = bounded(10);
    let config = AudioConfig::default();
    
    std::thread::spawn(move || {
        if let Ok(decoder) = AudioDecoder::new(path.as_ref()) {
            let buffer_size = config.buffer_size;
            let mut buffer = vec![0.0f32; buffer_size];
            
            loop {
                match decoder.read_samples(&mut buffer) {
                    Ok(count) if count > 0 => {
                        let samples: Vec<f32> = buffer[..count].to_vec();
                        if sender.send(samples).is_err() {
                            break;
                        }
                    }
                    Ok(_) => break, // End of file
                    Err(_) => break,
                }
            }
        }
    });

    Ok(AudioStream::new(receiver, config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_config_default() {
        let config = AudioConfig::default();
        assert_eq!(config.sample_rate, 44100);
        assert_eq!(config.channels, 2);
        assert_eq!(config.buffer_size, 2048);
    }

    #[test]
    fn test_audio_config_custom() {
        let config = AudioConfig {
            sample_rate: 48000,
            channels: 1,
            buffer_size: 4096,
        };
        assert_eq!(config.sample_rate, 48000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.buffer_size, 4096);
    }
}
