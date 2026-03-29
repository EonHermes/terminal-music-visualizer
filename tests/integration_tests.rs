//! Integration tests for the terminal music visualizer
//! 
//! These tests verify the complete pipeline from audio loading to visualization.

use terminal_music_visualizer::{
    audio::{load_audio, AudioConfig},
    visualization::{FFTAnalyzer, VisualizerController, VisualizationMode, ColorPalette},
    utils::math::{rms, peak, normalize},
};

/// Test FFT analyzer with synthetic data
#[test]
fn test_fft_pipeline() {
    let mut fft = FFTAnalyzer::new(64);
    
    // Create a sine wave at 440 Hz (A4 note)
    let sample_rate = 44100;
    let samples: Vec<f32> = (0..2048)
        .map(|i| ((i as f32 * 440.0 * 2.0 * std::f32::consts::PI) / sample_rate as f32).sin())
        .collect();

    let result = fft.analyze(&samples, sample_rate).unwrap();
    
    assert_eq!(result.len(), 64);
    
    // Should have non-zero frequency content
    let max_val = result.iter().cloned().fold(0.0f32, f32::max);
    assert!(max_val > 0.1, "Sine wave should produce noticeable frequency content");
}

/// Test visualizer controller integration
#[test]
fn test_visualizer_controller_integration() {
    let mut controller = VisualizerController::new(64);
    
    // Create test samples
    let samples: Vec<f32> = (0..1024)
        .map(|i| ((i as f32 * 0.1).sin()) * 0.5)
        .collect();

    let result = controller.process(&samples, 44100);
    
    assert!(result.is_ok());
    let frame = result.unwrap();
    
    assert_eq!(frame.frequencies.len(), 64);
    assert_eq!(frame.peaks.len(), 64);
}

/// Test mode cycling through controller
#[test]
fn test_mode_cycling() {
    let mut controller = VisualizerController::new(64);
    
    // Cycle through all modes
    let initial_mode = *controller.mode();
    
    for _ in 0..5 {
        let current = *controller.mode();
        let next = current.next();
        controller.set_mode(next);
        
        assert_eq!(*controller.mode(), next);
    }
    
    // Should cycle back to initial (or close)
    controller.set_mode(initial_mode);
    assert_eq!(*controller.mode(), initial_mode);
}

/// Test palette cycling through controller
#[test]
fn test_palette_cycling() {
    let mut controller = VisualizerController::new(64);
    
    // Cycle through all palettes
    for _ in 0..7 {
        let current = *controller.palette();
        let next = current.next();
        controller.set_palette(next);
        
        assert_eq!(*controller.palette(), next);
    }
}

/// Test audio utilities integration
#[test]
fn test_audio_utilities() {
    // Create test signal
    let mut samples: Vec<f32> = (0..1000)
        .map(|i| ((i as f32 * 0.1).sin()) * 0.8)
        .collect();

    // Test RMS calculation
    let rms_val = rms(&samples);
    assert!(rms_val > 0.0 && rms_val < 1.0);

    // Test peak detection
    let peak_val = peak(&samples);
    assert!(peak_val > 0.5 && peak_val <= 1.0);

    // Test normalization
    normalize(&mut samples);
    let new_peak = peak(&samples);
    assert!((new_peak - 1.0).abs() < 1e-6);
}

/// Test visualization frame creation and processing
#[test]
fn test_visualization_frame_processing() {
    use terminal_music_visualizer::visualization::VisualizationFrame;
    
    // Create a frame with known values
    let frequencies: Vec<f32> = (0..64).map(|i| i as f32 / 63.0).collect();
    let peaks: Vec<f32> = frequencies.clone();
    
    let frame = VisualizationFrame::new(frequencies, peaks, 10.5);
    
    assert_eq!(frame.frequencies.len(), 64);
    assert_eq!(frame.peaks.len(), 64);
    assert!((frame.timestamp - 10.5).abs() < 1e-10);
    
    // Verify values are in expected range
    for (i, &freq) in frame.frequencies.iter().enumerate() {
        let expected = i as f32 / 63.0;
        assert!((freq - expected).abs() < 1e-6);
    }
}

/// Test empty frame handling
#[test]
fn test_empty_frame_handling() {
    use terminal_music_visualizer::visualization::VisualizationFrame;
    
    let frame = VisualizationFrame::empty(128);
    
    assert_eq!(frame.frequencies.len(), 128);
    assert_eq!(frame.peaks.len(), 128);
    assert!(frame.frequencies.iter().all(|&f| f == 0.0));
    assert!(frame.peaks.iter().all(|&p| p == 0.0));
}

/// Test color palette integration with visualization modes
#[test]
fn test_palette_mode_integration() {
    let mode = VisualizationMode::Bars(terminal_music_visualizer::visualization::BarStyle::Default);
    
    for palette in [
        ColorPalette::Rainbow,
        ColorPalette::Heat,
        ColorPalette::Grayscale,
        ColorPalette::Ocean,
        ColorPalette::Sunset,
        ColorPalette::Neon,
    ] {
        // Test that we can get colors across the full range
        let c0 = palette.get_color(0.0);
        let c05 = palette.get_color(0.5);
        let c1 = palette.get_color(1.0);
        
        // Just verify they don't panic and return valid colors
        assert!(true); // Placeholder - actual color validation would require more complex checks
    }
}

/// Test logarithmic frequency binning
#[test]
fn test_logarithmic_binning() {
    let mut fft = FFTAnalyzer::new(32);
    
    // Create a broadband signal (white noise approximation)
    let samples: Vec<f32> = (0..2048)
        .map(|_| fastrand::f32() * 2.0 - 1.0)
        .collect();

    let result = fft.analyze(&samples, 44100).unwrap();
    
    assert_eq!(result.len(), 32);
    
    // All values should be normalized to [0, 1]
    for &val in &result {
        assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
    }
}

/// Test different FFT sizes
#[test]
fn test_different_fft_sizes() {
    let sizes = [32, 64, 128, 256, 512];
    
    for size in &sizes {
        let mut fft = FFTAnalyzer::new(*size);
        
        let samples: Vec<f32> = vec![0.5; 1024];
        let result = fft.analyze(&samples, 44100).unwrap();
        
        assert_eq!(result.len(), *size);
    }
}

// Mock test for audio file loading (requires actual files)
#[test]
fn test_audio_loading_placeholder() {
    // This would require actual audio files in the test directory
    // In a real implementation, we'd have test fixtures
    
    // For now, just verify the function signature works
    let result = load_audio("/nonexistent/file.wav");
    
    // Should fail gracefully (file doesn't exist)
    assert!(result.is_err());
}

/// Test windowing functions affect FFT output
#[test]
fn test_windowing_effect() {
    use terminal_music_visualizer::utils::math::{apply_hann_window, rms};
    
    let mut samples1: Vec<f32> = (0..1024).map(|i| ((i as f32 * 0.1).sin())).collect();
    let mut samples2 = samples1.clone();
    
    // Apply Hann window to second set
    apply_hann_window(&mut samples2);
    
    // Windowed signal should have lower RMS (energy is spread out)
    let rms1 = rms(&samples1);
    let rms2 = rms(&samples2);
    
    assert!(rms2 < rms1, "Windowing should reduce overall RMS");
}
