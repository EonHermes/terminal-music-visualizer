//! Mathematical utilities for audio processing

/// Calculate RMS (Root Mean Square) of a slice of samples
pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    
    let sum: f32 = samples.iter().map(|&s| s * s).sum();
    (sum / samples.len() as f32).sqrt()
}

/// Calculate peak amplitude of a slice of samples
pub fn peak(samples: &[f32]) -> f32 {
    samples.iter().map(|&s| s.abs()).fold(0.0, f32::max)
}

/// Apply Hann window to a slice of samples (in place)
pub fn apply_hann_window(samples: &mut [f32]) {
    let n = samples.len();
    if n == 0 {
        return;
    }
    
    for i in 0..n {
        let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos());
        samples[i] *= window;
    }
}

/// Apply Hamming window to a slice of samples (in place)
pub fn apply_hamming_window(samples: &mut [f32]) {
    let n = samples.len();
    if n == 0 {
        return;
    }
    
    for i in 0..n {
        let window = 0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos();
        samples[i] *= window;
    }
}

/// Convert linear amplitude to decibels
pub fn linear_to_db(amplitude: f32) -> f32 {
    if amplitude <= 0.0 {
        return -f32::INFINITY;
    }
    
    20.0 * amplitude.log10()
}

/// Convert decibels to linear amplitude
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Normalize a slice of samples to [-1, 1] range
pub fn normalize(samples: &mut [f32]) {
    let peak = peak(samples);
    
    if peak > 0.0 {
        for sample in samples.iter_mut() {
            *sample /= peak;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rms_empty() {
        assert_eq!(rms(&[]), 0.0);
    }

    #[test]
    fn test_rms_constant() {
        let samples = vec![1.0, 1.0, 1.0, 1.0];
        assert!((rms(&samples) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rms_sine_wave() {
        // RMS of a sine wave with amplitude 1 should be 1/sqrt(2) ≈ 0.707
        let samples: Vec<f32> = (0..1000)
            .map(|i| ((i as f32 * 0.1).sin()))
            .collect();
        
        let rms_val = rms(&samples);
        assert!((rms_val - 0.707).abs() < 0.05); // Allow some tolerance
    }

    #[test]
    fn test_peak_empty() {
        assert_eq!(peak(&[]), 0.0);
    }

    #[test]
    fn test_peak_simple() {
        let samples = vec![0.5, -0.8, 0.3, -0.2];
        assert_eq!(peak(&samples), 0.8);
    }

    #[test]
    fn test_window_functions() {
        let mut samples = vec![1.0; 100];
        
        // Apply Hann window
        apply_hann_window(&mut samples);
        
        // First and last samples should be close to 0 (window effect)
        assert!(samples[0] < 0.01);
        assert!(samples[samples.len() - 1] < 0.01);
        
        // Middle samples should be close to 1
        let mid = samples.len() / 2;
        assert!(samples[mid] > 0.95);
    }

    #[test]
    fn test_linear_to_db() {
        assert!((linear_to_db(1.0) - 0.0).abs() < 1e-6); // 1.0 = 0 dB
        assert!(linear_to_db(0.0).is_infinite()); // 0 = -inf dB
    }

    #[test]
    fn test_db_to_linear() {
        assert!((db_to_linear(0.0) - 1.0).abs() < 1e-6); // 0 dB = 1.0
        assert!((db_to_linear(-20.0) - 0.1).abs() < 1e-6); // -20 dB = 0.1
    }

    #[test]
    fn test_normalize_empty() {
        let mut samples: Vec<f32> = vec![];
        normalize(&mut samples);
        assert!(samples.is_empty());
    }

    #[test]
    fn test_normalize_simple() {
        let mut samples = vec![0.5, -1.0, 0.5];
        normalize(&mut samples);
        
        // After normalization, peak should be 1.0
        assert_eq!(peak(&samples), 1.0);
    }

    #[test]
    fn test_normalize_zero() {
        let mut samples = vec![0.0, 0.0, 0.0];
        normalize(&mut samples);
        
        // Should remain all zeros (no division by zero)
        assert!(samples.iter().all(|&s| s == 0.0));
    }
}
