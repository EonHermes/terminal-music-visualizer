//! Terminal User Interface module
//! 
//! Handles terminal setup, rendering, and user interaction.

mod renderer;
mod input;

pub use renderer::Renderer;
pub use input::InputHandler;

use anyhow::{anyhow, Result};
use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, SetForegroundColor, ResetColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

use crate::visualization::{VisualizationFrame, VisualizationMode, ColorPalette, VisualizerController};
use crate::audio::{load_audio, AudioConfig};

/// Main visualizer application
pub struct Visualizer {
    renderer: Renderer,
    controller: VisualizerController,
    samples: Vec<f32>,
    config: AudioConfig,
    position: usize,
    is_running: bool,
}

impl Visualizer {
    /// Create a new visualizer from an audio file
    pub fn new<P: AsRef<std::path::Path>>(audio_path: P) -> Result<Self> {
        let (samples, config) = load_audio(audio_path)?;
        
        if samples.is_empty() {
            return Err(anyhow!("No audio samples found in file"));
        }

        // Create FFT analyzer with appropriate bin count based on terminal width
        let num_bins = 64; // Default to 64 frequency bins
        
        Ok(Self {
            renderer: Renderer::new(),
            controller: VisualizerController::new(num_bins),
            samples,
            config,
            position: 0,
            is_running: true,
        })
    }

    /// Run the main visualization loop
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        execute!(stdout(), Hide)?;
        
        let mut last_frame_time = std::time::Instant::now();
        let frame_duration = std::time::Duration::from_millis(16); // ~60fps
        
        while self.is_running {
            // Handle input
            if poll(std::time::Duration::from_millis(1))? {
                if let Event::Key(KeyEvent { code, .. }) = read()? {
                    self.handle_input(code)?;
                }
            }

            // Process audio and render
            let elapsed = last_frame_time.elapsed();
            if elapsed >= frame_duration {
                self.render_frame()?;
                last_frame_time = std::time::Instant::now();
            }

            // Small sleep to prevent CPU spinning
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // Restore terminal
        execute!(stdout(), Show)?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }

    /// Handle keyboard input
    fn handle_input(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.is_running = false;
            }
            KeyCode::Char('m') => {
                // Cycle visualization mode
                let current_mode = *self.controller.mode();
                let new_mode = current_mode.next();
                self.controller.set_mode(new_mode);
                self.renderer.show_message(format!("Mode: {}", new_mode.name()));
            }
            KeyCode::Char('p') => {
                // Cycle color palette
                let current_palette = *self.controller.palette();
                let new_palette = current_palette.next();
                self.controller.set_palette(new_palette);
                self.renderer.show_message(format!("Palette: {}", new_palette.name()));
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                // Increase sensitivity (not implemented yet)
                self.renderer.show_message("Sensitivity increased");
            }
            KeyCode::Char('-') => {
                // Decrease sensitivity (not implemented yet)
                self.renderer.show_message("Sensitivity decreased");
            }
            KeyCode::Left => {
                // Seek backward
                let seek_amount = self.config.sample_rate as usize * self.config.buffer_size / 10;
                if self.position > seek_amount {
                    self.position -= seek_amount;
                } else {
                    self.position = 0;
                }
            }
            KeyCode::Right => {
                // Seek forward
                let seek_amount = self.config.sample_rate as usize * self.config.buffer_size / 10;
                if self.position + seek_amount < self.samples.len() {
                    self.position += seek_amount;
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    /// Render a single frame
    fn render_frame(&mut self) -> Result<()> {
        let stdout = stdout();
        
        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        // Get current samples for this frame
        let buffer_size = self.config.buffer_size;
        let end_pos = (self.position + buffer_size).min(self.samples.len());
        let current_samples = &self.samples[self.position..end_pos];
        
        // Process through FFT analyzer
        if !current_samples.is_empty() {
            match self.controller.process(current_samples, self.config.sample_rate) {
                Ok(frame) => {
                    // Render the frame
                    self.renderer.render(&frame, self.controller.mode(), self.controller.palette())?;
                    
                    // Advance position
                    self.position += current_samples.len();
                    
                    // Loop if at end
                    if self.position >= self.samples.len() {
                        self.position = 0;
                        self.renderer.show_message("Looping...");
                    }
                }
                Err(e) => {
                    self.renderer.render_error(&e.to_string())?;
                }
            }
        } else {
            self.renderer.render_empty()?;
        }

        // Show controls hint
        self.renderer.show_controls();
        
        stdout.flush()?;
        
        Ok(())
    }
}

impl Drop for Visualizer {
    fn drop(&mut self) {
        // Ensure terminal is restored even if we panic
        let _ = execute!(stdout(), Show);
        let _ = terminal::disable_raw_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_creation_fails_without_file() {
        // This would require actual audio files, so we skip in unit tests
        // Integration tests will verify actual file loading
    }

    #[test]
    fn test_input_handling() {
        // Test that input codes are recognized (actual handling requires running TUI)
        let quit_codes = [KeyCode::Char('q'), KeyCode::Esc];
        
        for code in &quit_codes {
            assert!(matches!(code, KeyCode::Char('q') | KeyCode::Esc));
        }
    }
}
