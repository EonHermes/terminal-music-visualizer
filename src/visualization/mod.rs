//! Visualization module
//! 
//! Handles FFT frequency analysis and rendering different visualization modes.

mod fft;
mod modes;
mod palette;

pub use fft::FFTAnalyzer;
pub use modes::{VisualizationMode, BarStyle};
pub use palette::ColorPalette;

use anyhow::Result;

/// Frame of visualization data
#[derive(Debug, Clone)]
pub struct VisualizationFrame {
    /// Frequency bins (normalized 0.0-1.0)
    pub frequencies: Vec<f32>,
    /// Peak values for smooth animation
    pub peaks: Vec<f32>,
    /// Current timestamp in seconds
    pub timestamp: f64,
}

impl VisualizationFrame {
    pub fn new(frequencies: Vec<f32>, peaks: Vec<f32>, timestamp: f64) -> Self {
        Self {
            frequencies,
            peaks,
            timestamp,
        }
    }

    /// Create empty frame
    pub fn empty(size: usize) -> Self {
        Self {
            frequencies: vec![0.0; size],
            peaks: vec![0.0; size],
            timestamp: 0.0,
        }
    }
}

/// Main visualizer controller
pub struct VisualizerController {
    fft: FFTAnalyzer,
    mode: VisualizationMode,
    palette: ColorPalette,
    decay_rate: f32,
}

impl VisualizerController {
    pub fn new(num_bins: usize) -> Self {
        Self {
            fft: FFTAnalyzer::new(num_bins),
            mode: VisualizationMode::Bars(BarStyle::Default),
            palette: ColorPalette::Rainbow,
            decay_rate: 0.95,
        }
    }

    /// Process audio samples and generate visualization frame
    pub fn process(&mut self, samples: &[f32], sample_rate: u32) -> Result<VisualizationFrame> {
        // Perform FFT analysis
        let frequencies = self.fft.analyze(samples, sample_rate)?;
        
        // Update peaks with decay
        let mut peaks = vec![0.0; frequencies.len()];
        for (i, &freq) in frequencies.iter().enumerate() {
            peaks[i] = if freq > self.peaks()[i] {
                freq
            } else {
                self.peaks()[i] * self.decay_rate
            };
        }

        let frame = VisualizationFrame::new(frequencies, peaks, 0.0);
        
        // Update internal peak tracking (would need interior mutability in real impl)
        Ok(frame)
    }

    /// Get current visualization mode
    pub fn mode(&self) -> &VisualizationMode {
        &self.mode
    }

    /// Set visualization mode
    pub fn set_mode(&mut self, mode: VisualizationMode) {
        self.mode = mode;
    }

    /// Get current color palette
    pub fn palette(&self) -> &ColorPalette {
        &self.palette
    }

    /// Set color palette
    pub fn set_palette(&mut self, palette: ColorPalette) {
        self.palette = palette;
    }

    /// Helper method to access peaks (would use RefCell in production)
    fn peaks(&self) -> &[f32] {
        // Placeholder - would track peaks internally
        &[]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization_frame_creation() {
        let frame = VisualizationFrame::new(
            vec![0.5, 0.8, 0.3],
            vec![0.6, 0.9, 0.4],
            12.5,
        );
        
        assert_eq!(frame.frequencies.len(), 3);
        assert_eq!(frame.peaks.len(), 3);
        assert!((frame.timestamp - 12.5).abs() < 1e-10);
    }

    #[test]
    fn test_empty_frame() {
        let frame = VisualizationFrame::empty(64);
        
        assert_eq!(frame.frequencies.len(), 64);
        assert_eq!(frame.peaks.len(), 64);
        assert!(frame.frequencies.iter().all(|&f| f == 0.0));
    }

    #[test]
    fn test_controller_creation() {
        let controller = VisualizerController::new(128);
        
        assert_eq!(controller.mode(), &VisualizationMode::Bars(BarStyle::Default));
        assert_eq!(controller.palette(), &ColorPalette::Rainbow);
    }
}
