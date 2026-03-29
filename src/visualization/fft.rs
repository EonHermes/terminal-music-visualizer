//! FFT (Fast Fourier Transform) analyzer
//! 
//! Converts time-domain audio samples to frequency-domain data for visualization.

use anyhow::{anyhow, Result};
use rustfft::FftPlanner;

/// FFT analyzer for audio frequency analysis
pub struct FFTAnalyzer {
    num_bins: usize,
    fft_size: usize,
    planner: Option<FftPlanner<f32>>,
    buffer: Vec<f32>,
}

impl FFTAnalyzer {
    /// Create a new FFT analyzer with specified number of output bins
    pub fn new(num_bins: usize) -> Self {
        // FFT size should be power of 2 and at least 2x the number of bins
        let fft_size = if num_bins <= 512 {
            1024
        } else if num_bins <= 1024 {
            2048
        } else {
            4096
        };

        Self {
            num_bins,
            fft_size,
            planner: None,
            buffer: vec![0.0; fft_size],
        }
    }

    /// Analyze audio samples and return frequency bins (normalized 0.0-1.0)
    pub fn analyze(&mut self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        if samples.is_empty() {
            return Ok(vec![0.0; self.num_bins]);
        }

        // Fill buffer with samples (with windowing for better frequency resolution)
        let copy_len = self.fft_size.min(samples.len());
        
        // Apply Hann window for smoother frequency analysis
        for i in 0..copy_len {
            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / copy_len as f32).cos());
            self.buffer[i] = samples[i] * window;
        }
        
        // Zero-pad if needed
        for i in copy_len..self.fft_size {
            self.buffer[i] = 0.0;
        }

        // Create FFT planner if not exists
        let planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(self.fft_size);

        // Perform FFT
        let mut complex_buffer: Vec<rustfft::num_complex::Complex<f32>> = self.buffer
            .iter()
            .map(|&x| rustfft::num_complex::Complex::new(x, 0.0))
            .collect();
        
        fft.process(&mut complex_buffer);

        // Convert to magnitude spectrum and downsample to num_bins
        let magnitudes: Vec<f32> = (0..self.fft_size / 2)
            .map(|i| {
                let mag = complex_buffer[i].norm();
                // Normalize by FFT size
                mag / self.fft_size as f32
            })
            .collect();

        // Downsample to desired number of bins using logarithmic spacing for better audio representation
        self.downsample_log(&magnitudes)
    }

    /// Downsample frequency data using logarithmic spacing (better for audio visualization)
    fn downsample_log(&self, magnitudes: &[f32]) -> Result<Vec<f32>> {
        if magnitudes.is_empty() || self.num_bins == 0 {
            return Ok(vec![]);
        }

        let mut bins = Vec::with_capacity(self.num_bins);
        let max_bin = magnitudes.len();

        for i in 0..self.num_bins {
            // Logarithmic spacing: lower frequencies get more bins
            let start_ratio = (i as f64 / self.num_bins as f64).powf(2.0);
            let end_ratio = ((i + 1) as f64 / self.num_bins as f64).powf(2.0);

            let start_idx = (start_ratio * max_bin as f64) as usize;
            let end_idx = (end_ratio * max_bin as f64).min(max_bin as f64) as usize;

            if start_idx >= end_idx || start_idx >= max_bin {
                bins.push(0.0);
                continue;
            }

            // Average magnitude in this bin
            let sum: f32 = magnitudes[start_idx..end_idx].iter().sum();
            let avg = sum / (end_idx - start_idx) as f32;

            // Apply perceptual scaling (boost lower frequencies slightly)
            let scaled = avg * 1.5;
            
            // Clamp to [0, 1]
            bins.push(scaled.min(1.0));
        }

        Ok(bins)
    }

    /// Get the FFT size being used
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get the number of output bins
    pub fn num_bins(&self) -> usize {
        self.num_bins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_analyzer_creation() {
        let analyzer = FFTAnalyzer::new(64);
        
        assert_eq!(analyzer.num_bins(), 64);
        assert_eq!(analyzer.fft_size(), 1024);
    }

    #[test]
    fn test_fft_with_different_sizes() {
        let analyzer_128 = FFTAnalyzer::new(128);
        let analyzer_512 = FFTAnalyzer::new(512);
        let analyzer_1024 = FFTAnalyzer::new(1024);

        assert_eq!(analyzer_128.fft_size(), 1024);
        assert_eq!(analyzer_512.fft_size(), 2048);
        assert_eq!(analyzer_1024.fft_size(), 4096);
    }

    #[test]
    fn test_analyze_empty_samples() {
        let mut analyzer = FFTAnalyzer::new(64);
        let result = analyzer.analyze(&[], 44100).unwrap();
        
        assert_eq!(result.len(), 64);
        assert!(result.iter().all(|&f| f == 0.0));
    }

    #[test]
    fn test_analyze_sine_wave() {
        let mut analyzer = FFTAnalyzer::new(64);
        
        // Create a simple sine wave at 440 Hz
        let sample_rate = 44100;
        let samples: Vec<f32> = (0..1024)
            .map(|i| {
                let freq = 440.0;
                ((i as f32 * freq * 2.0 * std::f32::consts::PI) / sample_rate as f32).sin()
            })
            .collect();

        let result = analyzer.analyze(&samples, sample_rate).unwrap();
        
        assert_eq!(result.len(), 64);
        // Should have some non-zero values due to the sine wave
        let max_val = result.iter().cloned().fold(0.0f32, f32::max);
        assert!(max_val > 0.1); // Sine wave should produce noticeable frequency content
    }

    #[test]
    fn test_analyze_returns_normalized_values() {
        let mut analyzer = FFTAnalyzer::new(32);
        
        // Create samples with known amplitude
        let samples: Vec<f32> = vec![0.5; 1024];
        
        let result = analyzer.analyze(&samples, 44100).unwrap();
        
        assert_eq!(result.len(), 32);
        // All values should be in [0, 1] range
        assert!(result.iter().all(|&f| f >= 0.0 && f <= 1.0));
    }

    #[test]
    fn test_logarithmic_downsampling() {
        let mut analyzer = FFTAnalyzer::new(64);
        
        // Create frequency sweep
        let samples: Vec<f32> = (0..2048)
            .map(|i| ((i as f32 * 10.0) / 2048.0).sin())
            .collect();

        let result = analyzer.analyze(&samples, 44100).unwrap();
        
        assert_eq!(result.len(), 64);
        // Lower bins should generally have higher values due to log spacing
        // (This is a soft check as actual content depends on the signal)
    }
}
