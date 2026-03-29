use anyhow::Result;
use terminal_music_visualizer::{audio, tui, visualization};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <audio_file>", args[0]);
        eprintln!("\nSupported formats: WAV, MP3");
        eprintln!("Example: {} music.mp3", args[0]);
        std::process::exit(1);
    }

    let audio_path = &args[1];
    
    // Initialize the visualizer
    let mut visualizer = tui::Visualizer::new(audio_path)?;
    
    // Run the main loop
    visualizer.run()
}
