//! Color palettes for visualization
//! 
//! Predefined color schemes for different visual moods.

use crossterm::style::Color;

/// Available color palettes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorPalette {
    /// Rainbow gradient (red -> yellow -> green -> cyan -> blue -> magenta)
    Rainbow,
    /// Heat map (blue -> cyan -> yellow -> red)
    Heat,
    /// Monochrome gray scale
    Grayscale,
    /// Ocean theme (blues and cyans)
    Ocean,
    /// Sunset theme (oranges, pinks, purples)
    Sunset,
    /// Neon theme (bright colors on dark)
    Neon,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::Rainbow
    }
}

impl ColorPalette {
    /// Get display name for the palette
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rainbow => "Rainbow",
            Self::Heat => "Heat",
            Self::Grayscale => "Grayscale",
            Self::Ocean => "Ocean",
            Self::Sunset => "Sunset",
            Self::Neon => "Neon",
        }
    }

    /// Cycle to next palette
    pub fn next(&self) -> Self {
        match self {
            Self::Rainbow => Self::Heat,
            Self::Heat => Self::Grayscale,
            Self::Grayscale => Self::Ocean,
            Self::Ocean => Self::Sunset,
            Self::Sunset => Self::Neon,
            Self::Neon => Self::Rainbow,
        }
    }

    /// Get color for a value (0.0-1.0) based on this palette
    pub fn get_color(&self, value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match self {
            Self::Rainbow => Self::rainbow_color(v),
            Self::Heat => Self::heat_color(v),
            Self::Grayscale => Self::grayscale_color(v),
            Self::Ocean => Self::ocean_color(v),
            Self::Sunset => Self::sunset_color(v),
            Self::Neon => Self::neon_color(v),
        }
    }

    /// Rainbow gradient color
    fn rainbow_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 6.0) as u8 {
            0 => Self::interpolate_color(v * 6.0, Color::Red, Color::Yellow),
            1 => Self::interpolate_color((v * 6.0 - 1.0), Color::Yellow, Color::Green),
            2 => Self::interpolate_color((v * 6.0 - 2.0), Color::Green, Color::Cyan),
            3 => Self::interpolate_color((v * 6.0 - 3.0), Color::Cyan, Color::Blue),
            4 => Self::interpolate_color((v * 6.0 - 4.0), Color::Blue, Color::Magenta),
            _ => Self::interpolate_color((v * 6.0 - 5.0), Color::Magenta, Color::Red),
        }
    }

    /// Heat map color (blue -> cyan -> yellow -> red)
    fn heat_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 4.0) as u8 {
            0 => Self::interpolate_color(v * 4.0, Color::Blue, Color::Cyan),
            1 => Self::interpolate_color((v * 4.0 - 1.0), Color::Cyan, Color::White),
            2 => Self::interpolate_color((v * 4.0 - 2.0), Color::White, Color::Yellow),
            _ => Self::interpolate_color((v * 4.0 - 3.0), Color::Yellow, Color::Red),
        }
    }

    /// Grayscale color
    fn grayscale_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        // Simple mapping to available gray colors
        match (v * 7.0) as u8 {
            0 => Color::Black,
            1 => Color::DarkGrey,
            2 => Color::DarkGrey,
            3 => Color::Grey,
            4 => Color::Grey,
            5 => Color::White,
            6 => Color::White,
            _ => Color::White,
        }
    }

    /// Ocean theme (blues and cyans)
    fn ocean_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 4.0) as u8 {
            0 => Color::DarkBlue,
            1 => Color::Blue,
            2 => Color::Cyan,
            _ => Color::LightCyan,
        }
    }

    /// Sunset theme (oranges, pinks, purples)
    fn sunset_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 4.0) as u8 {
            0 => Color::Yellow,
            1 => Color::Red,
            2 => Color::Magenta,
            _ => Color::LightMagenta,
        }
    }

    /// Neon theme (bright saturated colors)
    fn neon_color(value: f32) -> Color {
        let v = value.clamp(0.0, 1.0);
        
        match (v * 6.0) as u8 {
            0 => Color::LightRed,
            1 => Color::Yellow,
            2 => Color::LightGreen,
            3 => Color::LightCyan,
            4 => Color::LightBlue,
            _ => Color::LightMagenta,
        }
    }

    /// Interpolate between two colors (simplified for terminal)
    fn interpolate_color(t: f32, from: Color, to: Color) -> Color {
        // Simplified interpolation - just pick based on threshold
        if t < 0.5 {
            from
        } else {
            to
        }
    }

    /// Get background color suggestion for this palette
    pub fn background(&self) -> Option<Color> {
        match self {
            Self::Grayscale => Some(Color::White),
            _ => None, // Assume dark background
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_names() {
        assert_eq!(ColorPalette::Rainbow.name(), "Rainbow");
        assert_eq!(ColorPalette::Heat.name(), "Heat");
        assert_eq!(ColorPalette::Grayscale.name(), "Grayscale");
        assert_eq!(ColorPalette::Ocean.name(), "Ocean");
        assert_eq!(ColorPalette::Sunset.name(), "Sunset");
        assert_eq!(ColorPalette::Neon.name(), "Neon");
    }

    #[test]
    fn test_palette_cycling() {
        let mut palette = ColorPalette::Rainbow;
        
        for expected in [
            ColorPalette::Heat,
            ColorPalette::Grayscale,
            ColorPalette::Ocean,
            ColorPalette::Sunset,
            ColorPalette::Neon,
            ColorPalette::Rainbow,
        ] {
            palette = palette.next();
            assert_eq!(palette, expected);
        }
    }

    #[test]
    fn test_palette_colors() {
        let rainbow = ColorPalette::Rainbow;
        
        // Test that colors are returned for different values
        let _c1 = rainbow.get_color(0.0);
        let _c2 = rainbow.get_color(0.5);
        let _c3 = rainbow.get_color(1.0);
        
        // Test clamping
        let _c4 = rainbow.get_color(-0.5);  // Should clamp to 0.0
        let _c5 = rainbow.get_color(1.5);   // Should clamp to 1.0
    }

    #[test]
    fn test_default_palette() {
        let default: ColorPalette = Default::default();
        assert_eq!(default, ColorPalette::Rainbow);
    }

    #[test]
    fn test_background_colors() {
        assert!(ColorPalette::Grayscale.background().is_some());
        assert!(ColorPalette::Rainbow.background().is_none());
        assert!(ColorPalette::Heat.background().is_none());
    }
}
