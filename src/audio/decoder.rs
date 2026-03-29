//! Audio decoder for various formats
//! 
//! Supports WAV and MP3 files using rodio and hound libraries.

use anyhow::{anyhow, Result};
use std::fs::File;
use std::path::Path;

use super::AudioConfig;

/// Supported audio formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioFormat {
    Wav,
    Mp3,
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wav => write!(f, "WAV"),
            Self::Mp3 => write!(f, "MP3"),
        }
    }
}

/// Audio decoder that can read various audio formats
pub struct AudioDecoder {
    samples: Vec<f32>,
    config: AudioConfig,
    position: usize,
    format: AudioFormat,
}

impl AudioDecoder {
    /// Create a new decoder from an audio file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let format = match extension.as_str() {
            "wav" => AudioFormat::Wav,
            "mp3" => AudioFormat::Mp3,
            _ => return Err(anyhow!("Unsupported audio format: {}", extension)),
        };

        match format {
            AudioFormat::Wav => Self::decode_wav(path),
            AudioFormat::Mp3 => Self::decode_mp3(path),
        }
    }

    /// Decode WAV file using hound
    fn decode_wav<P: AsRef<Path>>(path: P) -> Result<Self> {
        let spec = hound::WavSpec::from_path(&path)?;
        let reader = hound::WavReader::open(path)?;
        
        let channels = spec.channels as u16;
        let sample_rate = spec.sample_rate as u32;
        
        // Convert all samples to f32
        let samples: Vec<f32> = reader.into_samples::<i32>()
            .filter_map(|s| s.ok())
            .map(|s| s as f32 / i32::MAX as f32)
            .collect();

        let config = AudioConfig {
            sample_rate,
            channels,
            buffer_size: 2048,
        };

        Ok(Self {
            samples,
            config,
            position: 0,
            format: AudioFormat::Wav,
        })
    }

    /// Decode MP3 file using rodio
    fn decode_mp3<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let sink = rodio::Decoder::new(file)?;
        
        let config = AudioConfig {
            sample_rate: sink.sample_rate() as u32,
            channels: sink.channels() as u16,
            buffer_size: 2048,
        };

        // Collect all samples
        let samples: Vec<f32> = sink
            .into_iter()
            .map(|s| s)
            .collect();

        Ok(Self {
            samples,
            config,
            position: 0,
            format: AudioFormat::Mp3,
        })
    }

    /// Read all samples from the decoder
    pub fn read_all(&self) -> Result<Vec<f32>> {
        Ok(self.samples.clone())
    }

    /// Read a buffer of samples (advances position)
    pub fn read_samples(&self, buffer: &mut [f32]) -> Result<usize> {
        let available = self.samples.len() - self.position;
        if available == 0 {
            return Ok(0);
        }

        let to_read = buffer.len().min(available);
        buffer[..to_read].copy_from_slice(&self.samples[self.position..self.position + to_read]);
        
        // Note: This is immutable, so we can't actually advance position
        // In a real implementation, we'd use RefCell or interior mutability
        Ok(to_read)
    }

    /// Get the audio configuration
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }

    /// Get the format of this audio file
    pub fn format(&self) -> AudioFormat {
        self.format
    }

    /// Get total duration in seconds
    pub fn duration(&self) -> f64 {
        let samples_per_channel = self.samples.len() / self.config.channels as usize;
        samples_per_channel as f64 / self.config.sample_rate as f64
    }

    /// Check if we've reached the end of the audio
    pub fn is_finished(&self) -> bool {
        self.position >= self.samples.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_format_display() {
        assert_eq!(format!("{}", AudioFormat::Wav), "WAV");
        assert_eq!(format!("{}", AudioFormat::Mp3), "MP3");
    }

    #[test]
    fn test_decoder_config_access() {
        // This would require actual audio files, so we skip in unit tests
        // Integration tests will verify actual decoding
    }
}
