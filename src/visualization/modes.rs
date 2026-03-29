//! Visualization modes and styles
//! 
//! Different ways to render the frequency data in the terminal.

use crossterm::style::Color;

/// Available visualization modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisualizationMode {
    /// Vertical bars with customizable style
    Bars(BarStyle),
    /// Waveform display (time domain)
    Waveform,
    /// Frequency spectrum (horizontal bars)
    Spectrum,
    /// Circular/radial visualization
    Circular,
}

impl Default for VisualizationMode {
    fn default() -> Self {
        Self::Bars(BarStyle::Default)
    }
}

/// Bar visualization styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarStyle {
    /// Simple filled bars
    Default,
    /// Bars with gradient fill
    Gradient,
    /// Outlined bars with fill indicator
    Outlined,
    /// Smooth rounded bars
    Rounded,
}

impl VisualizationMode {
    /// Get display name for the mode
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bars(_) => "Bars",
            Self::Waveform => "Waveform",
            Self::Spectrum => "Spectrum",
            Self::Circular => "Circular",
        }
    }

    /// Cycle to next mode
    pub fn next(&self) -> Self {
        match self {
            Self::Bars(_) => Self::Waveform,
            Self::Waveform => Self::Spectrum,
            Self::Spectrum => Self::Circular,
            Self::Circular => Self::Bars(BarStyle::Default),
        }
    }

    /// Check if this mode supports bar styles
    pub fn is_bar_mode(&self) -> bool {
        matches!(self, Self::Bars(_))
    }
}

impl BarStyle {
    /// Get display name for the style
    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Gradient => "Gradient",
            Self::Outlined => "Outlined",
            Self::Rounded => "Rounded",
        }
    }

    /// Cycle to next bar style
    pub fn next(&self) -> Self {
        match self {
            Self::Default => Self::Gradient,
            Self::Gradient => Self::Outlined,
            Self::Outlined => Self::Rounded,
            Self::Rounded => Self::Default,
        }
    }
}

/// Character sets for different bar styles
pub mod chars {
    /// Vertical bar characters (from empty to full)
    pub const VERTICAL_BARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    
    /// Smooth vertical bars with more resolution
    pub const SMOOTH_VERTICAL: &[char] = &[' ', '░', '▒', '▓', '▄', '▅', '▆', '▇', '█'];
    
    /// Horizontal bar characters
    pub const HORIZONTAL_BARS: &[char] = &[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

    /// Get character for value (0.0-1.0) using vertical bars
    pub fn vertical_bar(value: f32) -> char {
        let idx = ((value * 8.0).clamp(0.0, 8.0) as usize).min(VERTICAL_BARS.len() - 1);
        VERTICAL_BARS[idx]
    }

    /// Get character for value using smooth vertical bars
    pub fn smooth_vertical(value: f32) -> char {
        let idx = ((value * 8.0).clamp(0.0, 8.0) as usize).min(SMOOTH_VERTICAL.len() - 1);
        SMOOTH_VERTICAL[idx]
    }

    /// Get character for value using horizontal bars
    pub fn horizontal_bar(value: f32) -> char {
        let idx = ((value * 8.0).clamp(0.0, 8.0) as usize).min(HORIZONTAL_BARS.len() - 1);
        HORIZONTAL_BARS[idx]
    }
}

/// Color utilities for visualization
pub mod colors {
    use crossterm::style::Color;

    /// Get color based on value (0.0-1.0) using rainbow gradient
    pub fn rainbow(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        // Map to hue (0-360 degrees)
        let hue = (v * 360.0) as u8;
        
        // Simple HSV to RGB conversion for terminal colors
        match hue {
            0..=59 => Color::Red,
            60..=119 => Color::Yellow,
            120..=179 => Color::Green,
            180..=239 => Color::Cyan,
            240..=299 => Color::Blue,
            300..=359 => Color::Magenta,
            _ => Color::Red,
        }
    }

    /// Get color based on value using heat map (blue -> red)
    pub fn heatmap(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 4.0) as u8 {
            0 => Color::Blue,
            1 => Color::Cyan,
            2 => Color::Yellow,
            _ => Color::Red,
        }
    }

    /// Get color based on value using monochrome (gray scale)
    pub fn grayscale(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        // Map to gray intensity
        match (v * 7.0) as u8 {
            0 => Color::Grey,
            1 => Color::DarkGrey,
            2 => Color::DarkGrey,
            3 => Color::White,
            _ => Color::White,
        }
    }

    /// Get bright color variant for emphasis
    pub fn bright(value: f32) -> Color {
        match rainbow(value) {
            Color::Red => Color::Red,
            Color::Yellow => Color::Yellow,
            Color::Green => Color::Green,
            Color::Cyan => Color::Cyan,
            Color::Blue => Color::Blue,
            Color::Magenta => Color::Magenta,
            _ => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization_mode_names() {
        assert_eq!(VisualizationMode::Bars(BarStyle::Default).name(), "Bars");
        assert_eq!(VisualizationMode::Waveform.name(), "Waveform");
        assert_eq!(VisualizationMode::Spectrum.name(), "Spectrum");
        assert_eq!(VisualizationMode::Circular.name(), "Circular");
    }

    #[test]
    fn test_mode_cycling() {
        let mut mode = VisualizationMode::Bars(BarStyle::Default);
        
        mode = mode.next();
        assert_eq!(mode, VisualizationMode::Waveform);
        
        mode = mode.next();
        assert_eq!(mode, VisualizationMode::Spectrum);
        
        mode = mode.next();
        assert_eq!(mode, VisualizationMode::Circular);
        
        mode = mode.next();
        assert_eq!(mode, VisualizationMode::Bars(BarStyle::Default));
    }

    #[test]
    fn test_bar_style_names() {
        assert_eq!(BarStyle::Default.name(), "Default");
        assert_eq!(BarStyle::Gradient.name(), "Gradient");
        assert_eq!(BarStyle::Outlined.name(), "Outlined");
        assert_eq!(BarStyle::Rounded.name(), "Rounded");
    }

    #[test]
    fn test_bar_style_cycling() {
        let mut style = BarStyle::Default;
        
        style = style.next();
        assert_eq!(style, BarStyle::Gradient);
        
        style = style.next();
        assert_eq!(style, BarStyle::Outlined);
        
        style = style.next();
        assert_eq!(style, BarStyle::Rounded);
        
        style = style.next();
        assert_eq!(style, BarStyle::Default);
    }

    #[test]
    fn test_is_bar_mode() {
        assert!(VisualizationMode::Bars(BarStyle::Default).is_bar_mode());
        assert!(!VisualizationMode::Waveform.is_bar_mode());
        assert!(!VisualizationMode::Spectrum.is_bar_mode());
        assert!(!VisualizationMode::Circular.is_bar_mode());
    }

    #[test]
    fn test_vertical_bar_chars() {
        use chars::*;
        
        assert_eq!(vertical_bar(0.0), ' ');
        assert_eq!(vertical_bar(1.0), '█');
        assert_eq!(vertical_bar(0.5), '▄'); // Approximately middle
    }

    #[test]
    fn test_color_functions() {
        use colors::*;
        
        // Just verify they don't panic and return valid colors
        let _c1 = rainbow(0.0);
        let _c2 = rainbow(0.5);
        let _c3 = rainbow(1.0);
        
        let _h1 = heatmap(0.0);
        let _h2 = heatmap(0.5);
        let _h3 = heatmap(1.0);
    }
}
