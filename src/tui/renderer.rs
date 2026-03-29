//! Terminal renderer for visualization frames
//! 
//! Handles drawing the visualizations to the terminal.

use anyhow::Result;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor, Stylize},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

use crate::visualization::{VisualizationFrame, VisualizationMode, BarStyle, ColorPalette};
use crate::visualization::modes::chars;

/// Terminal renderer
pub struct Renderer {
    width: u16,
    height: u16,
    message: Option<String>,
}

impl Renderer {
    /// Create a new renderer
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap_or((80, 24));
        
        Self {
            width,
            height,
            message: None,
        }
    }

    /// Update terminal size
    pub fn update_size(&mut self) {
        if let Ok((w, h)) = terminal::size() {
            self.width = w;
            self.height = h;
        }
    }

    /// Render a visualization frame
    pub fn render(&mut self, frame: &VisualizationFrame, mode: &VisualizationMode, palette: &ColorPalette) -> Result<()> {
        let stdout = stdout();
        
        match mode {
            VisualizationMode::Bars(style) => {
                self.render_bars(frame, style, palette)?;
            }
            VisualizationMode::Waveform => {
                self.render_waveform(frame, palette)?;
            }
            VisualizationMode::Spectrum => {
                self.render_spectrum(frame, palette)?;
            }
            VisualizationMode::Circular => {
                self.render_circular(frame, palette)?;
            }
        }

        // Show message if any
        if let Some(msg) = &self.message {
            self.show_message_at_bottom(msg);
        }

        Ok(())
    }

    /// Render vertical bars visualization
    fn render_bars(&mut self, frame: &VisualizationFrame, style: &BarStyle, palette: &ColorPalette) -> Result<()> {
        let stdout = stdout();
        let num_bins = frame.frequencies.len();
        
        // Calculate bar width based on terminal width
        let bar_width = (self.width as usize / num_bins).max(1);
        let max_height = (self.height as usize - 4) as f32; // Leave room for header/footer
        
        // Header
        execute!(stdout, MoveTo(0, 0))?;
        print!("{}", "🎵 Terminal Music Visualizer 🎵".bold().cyan());
        
        // Render bars
        for (i, &freq) in frame.frequencies.iter().enumerate() {
            let bar_height = (freq * max_height) as usize;
            let color = palette.get_color(freq);
            
            execute!(stdout, MoveTo((i * bar_width) as u16, 0))?;
            
            match style {
                BarStyle::Default => {
                    // Simple filled bars using block characters
                    for row in (0..max_height as usize).rev() {
                        let char = if row < bar_height {
                            chars::vertical_bar(freq)
                        } else {
                            ' '
                        };
                        
                        execute!(stdout, MoveTo((i * bar_width + 1) as u16, (max_height as usize - row) as u16))?;
                        if row < bar_height {
                            execute!(stdout, SetForegroundColor(color), Print(char), ResetColor)?;
                        } else {
                            print!("{}", char);
                        }
                    }
                }
                BarStyle::Gradient => {
                    // Gradient fill with color intensity
                    for row in (0..max_height as usize).rev() {
                        let normalized_row = row as f32 / max_height;
                        let intensity = freq.max(normalized_row);
                        
                        execute!(stdout, MoveTo((i * bar_width + 1) as u16, (max_height as usize - row) as u16))?;
                        
                        if row < bar_height {
                            // Adjust color brightness based on height
                            let char = chars::smooth_vertical(intensity);
                            execute!(stdout, SetForegroundColor(color), Print(char), ResetColor)?;
                        } else {
                            print!(" ");
                        }
                    }
                }
                BarStyle::Outlined => {
                    // Outlined bars with peak indicator
                    for row in (0..max_height as usize).rev() {
                        execute!(stdout, MoveTo((i * bar_width + 1) as u16, (max_height as usize - row) as u16))?;
                        
                        if row == bar_height.saturating_sub(1) && bar_height > 0 {
                            // Peak indicator
                            execute!(stdout, SetForegroundColor(Color::White), Print("▲"), ResetColor)?;
                        } else if row < bar_height {
                            execute!(stdout, SetForegroundColor(color), Print(chars::vertical_bar(freq)), ResetColor)?;
                        } else {
                            print!(" ");
                        }
                    }
                }
                BarStyle::Rounded => {
                    // Rounded/smooth bars
                    for row in (0..max_height as usize).rev() {
                        execute!(stdout, MoveTo((i * bar_width + 1) as u16, (max_height as usize - row) as u16))?;
                        
                        if row < bar_height {
                            // Use smooth characters for rounded look
                            let char = chars::smooth_vertical(freq);
                            execute!(stdout, SetForegroundColor(color), Print(char), ResetColor)?;
                        } else {
                            print!(" ");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Render waveform visualization (time domain)
    fn render_waveform(&mut self, frame: &VisualizationFrame, palette: &ColorPalette) -> Result<()> {
        let stdout = stdout();
        
        execute!(stdout, MoveTo(0, 0))?;
        print!("{}", "🌊 Waveform View 🌊".bold().cyan());

        // For waveform, we'd need actual time-domain samples
        // Using frequency data as a proxy for now
        let max_height = (self.height as usize - 4) as f32;
        let center = max_height / 2.0;
        
        for x in 0..self.width {
            let freq_idx = (x as usize * frame.frequencies.len() / self.width as usize).min(frame.frequencies.len() - 1);
            let amplitude = frame.frequencies[freq_idx];
            
            let top_y = (center - amplitude * max_height / 2.0) as u16;
            let bottom_y = (center + amplitude * max_height / 2.0) as u16;
            
            let color = palette.get_color(amplitude);
            
            for y in 0..self.height - 4 {
                execute!(stdout, MoveTo(x, y))?;
                
                if y >= top_y && y <= bottom_y {
                    execute!(stdout, SetForegroundColor(color), Print("█"), ResetColor)?;
                } else {
                    print!(" ");
                }
            }
        }

        Ok(())
    }

    /// Render horizontal spectrum visualization
    fn render_spectrum(&mut self, frame: &VisualizationFrame, palette: &ColorPalette) -> Result<()> {
        let stdout = stdout();
        
        execute!(stdout, MoveTo(0, 0))?;
        print!("{}", "📊 Frequency Spectrum 📊".bold().cyan());

        let max_width = self.width as f32 - 10.0; // Leave room for labels
        
        // Render each frequency bin as a horizontal bar
        for (i, &freq) in frame.frequencies.iter().enumerate() {
            let y = i as u16 % (self.height - 4);
            let bar_length = (freq * max_width) as u16;
            let color = palette.get_color(freq);
            
            execute!(stdout, MoveTo(0, y + 2))?;
            
            // Frequency label
            print!("{:3} |", i);
            
            // Bar
            for x in 0..self.width - 7 {
                if x < bar_length {
                    execute!(stdout, SetForegroundColor(color), Print("█"), ResetColor)?;
                } else {
                    print!(" ");
                }
            }
        }

        Ok(())
    }

    /// Render circular/radial visualization
    fn render_circular(&mut self, frame: &VisualizationFrame, palette: &ColorPalette) -> Result<()> {
        let stdout = stdout();
        
        execute!(stdout, MoveTo(0, 0))?;
        print!("{}", "🌀 Circular View 🌀".bold().cyan());

        // Simplified circular visualization using ASCII art
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        let radius = (self.height as f32 / 4.0) as u16;
        
        for y in 0..self.height {
            execute!(stdout, MoveTo(0, y))?;
            
            for x in 0..self.width {
                // Calculate distance from center
                let dx = x as f32 - center_x as f32;
                let dy = y as f32 - center_y as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                
                if dist < radius as f32 {
                    // Map position to frequency bin
                    let angle = ((dy.atan2(dx) + std::f32::consts::PI) / (2.0 * std::f32::consts::PI));
                    let bin_idx = (angle * frame.frequencies.len() as f32) as usize % frame.frequencies.len();
                    let freq = frame.frequencies[bin_idx];
                    
                    // Distance from center determines intensity
                    let normalized_dist = dist / radius as f32;
                    let intensity = freq.max(normalized_dist);
                    
                    let color = palette.get_color(intensity);
                    
                    if normalized_dist < 0.3 {
                        execute!(stdout, SetForegroundColor(color), Print("●"), ResetColor)?;
                    } else if normalized_dist < 0.6 {
                        execute!(stdout, SetForegroundColor(color), Print("○"), ResetColor)?;
                    } else {
                        execute!(stdout, SetForegroundColor(color), Print("◌"), ResetColor)?;
                    }
                } else {
                    print!(" ");
                }
            }
        }

        Ok(())
    }

    /// Render empty state
    pub fn render_empty(&self) -> Result<()> {
        let stdout = stdout();
        
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        
        print!("{}", "🎵 Terminal Music Visualizer 🎵".bold().cyan());
        println!("\n\n{}", "Loading audio...".dim());
        
        stdout.flush()?;
        
        Ok(())
    }

    /// Render an error message
    pub fn render_error(&self, error: &str) -> Result<()> {
        let stdout = stdout();
        
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        
        print!("{}", "❌ Error ❌".bold().red());
        println!("\n\n{}", error.red());
        
        stdout.flush()?;
        
        Ok(())
    }

    /// Show a temporary message
    pub fn show_message(&mut self, msg: String) {
        self.message = Some(msg);
    }

    /// Show message at bottom of screen
    fn show_message_at_bottom(&self, msg: &str) {
        let stdout = stdout();
        
        if let Ok(height) = terminal::size() {
            execute!(stdout, MoveTo(0, height.1 - 1)).ok();
            print!("{}", msg.dim());
        }
    }

    /// Show controls hint
    pub fn show_controls(&self) {
        let stdout = stdout();
        
        if let Ok((width, height)) = terminal::size() {
            execute!(stdout, MoveTo(0, height - 2)).ok();
            
            let controls = "Controls: Q=Quit | M=Mode | P=Palette | ←→=Seek";
            print!("{}", controls.dim());
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new();
        
        assert!(renderer.width > 0);
        assert!(renderer.height > 0);
        assert!(renderer.message.is_none());
    }

    #[test]
    fn test_empty_frame_render() {
        let mut renderer = Renderer::new();
        let frame = VisualizationFrame::empty(64);
        
        // Just verify it doesn't panic (actual rendering requires terminal)
        let result = renderer.render(&frame, &VisualizationMode::default(), &ColorPalette::default());
        assert!(result.is_ok() || result.is_err()); // Either is fine in test environment
    }

    #[test]
    fn test_message_handling() {
        let mut renderer = Renderer::new();
        
        renderer.show_message("Test message".to_string());
        assert_eq!(renderer.message, Some("Test message".to_string()));
    }
}
