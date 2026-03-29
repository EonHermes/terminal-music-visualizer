//! Sample rate conversion utilities
//! 
//! Handles resampling audio to a target sample rate for consistent processing.

use super::AudioConfig;

/// Resampler for converting between sample rates
pub struct Resampler {
    source_rate: u32,
    target_rate: u32,
    ratio: f64,
}

impl Resampler {
    /// Create a new resampler from source to target sample rate
    pub fn new(source_rate: u32, target_rate: u32) -> Self {
        Self {
            source_rate,
            target_rate,
            ratio: target_rate as f64 / source_rate as f64,
        }
    }

    /// Resample a slice of audio samples
    pub fn resample(&self, input: &[f32]) -> Vec<f32> {
        let output_len = (input.len() as f64 * self.ratio).round() as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let src_pos = i as f64 / self.ratio;
            let src_idx = src_pos as usize;
            
            if src_idx >= input.len() {
                break;
            }

            // Simple linear interpolation
            let frac = src_pos - src_idx as f64;
            let next_idx = (src_idx + 1).min(input.len() - 1);
            
            let sample = input[src_idx] * (1.0 - frac) + input[next_idx] * frac;
            output.push(sample as f32);
        }

        output
    }

    /// Resample and return with updated config
    pub fn resample_with_config(&self, samples: &[f32], original_config: &AudioConfig) -> (Vec<f32>, AudioConfig) {
        let resampled = self.resample(samples);
        let new_config = AudioConfig {
            sample_rate: self.target_rate,
            channels: original_config.channels,
            buffer_size: original_config.buffer_size,
        };

        (resampled, new_config)
    }

    /// Get the conversion ratio
    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    /// Check if resampling is needed
    pub fn needs_resampling(&self) -> bool {
        self.source_rate != self.target_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resampler_creation() {
        let resampler = Resampler::new(44100, 48000);
        assert_eq!(resampler.source_rate, 44100);
        assert_eq!(resampler.target_rate, 48000);
        assert!((resampler.ratio - 48000.0/44100.0).abs() < 1e-10);
    }

    #[test]
    fn test_resampler_needs_resampling() {
        let resampler = Resampler::new(44100, 48000);
        assert!(resampler.needs_resampling());

        let no_resample = Resampler::new(44100, 44100);
        assert!(!no_resample.needs_resampling());
    }

    #[test]
    fn test_simple_resampling() {
        // Create a simple sine wave at 44100 Hz sample rate
        let input: Vec<f32> = (0..100)
            .map(|i| (i as f64 * 0.1).sin() as f32)
            .collect();

        // Downsample to half the rate
        let resampler = Resampler::new(44100, 22050);
        let output = resampler.resample(&input);

        // Output should be roughly half the size
        assert!(output.len() < input.len());
        assert!(output.len() > input.len() / 3); // Allow for interpolation
    }

    #[test]
    fn test_resample_with_config() {
        let samples = vec![0.5, -0.5, 0.5, -0.5];
        let config = AudioConfig {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 2048,
        };

        let resampler = Resampler::new(44100, 48000);
        let (resampled, new_config) = resampler.resample_with_config(&samples, &config);

        assert_eq!(new_config.sample_rate, 48000);
        assert_eq!(new_config.channels, 2);
        assert!(!resampled.is_empty());
    }
}
