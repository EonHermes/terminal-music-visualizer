# Terminal Music Visualizer 🎵

A beautiful Rust TUI music visualizer with real-time FFT analysis and multiple visualization modes.

## Features

- **Real-time audio visualization** using Fast Fourier Transform (FFT)
- **Multiple visualization modes**: spectrum analyzer, waterfall, circular
- **Color palettes** for different moods and styles
- **Supports multiple audio formats**: WAV, MP3, OGG, FLAC
- **Pure terminal UI** with crossterm - no external dependencies needed
- **Configurable** via TOML configuration file

## Tech Stack

- **Language**: Rust 2021 edition
- **Terminal UI**: crossterm
- **Audio Processing**: rodio, hound
- **FFT**: rustfft
- **Configuration**: serde + toml

## Installation

```bash
cargo build --release
./target/release/terminal_music_visualizer [audio-file]
```

## Usage

```bash
# Play and visualize a music file
cargo run -- release your-song.mp3

# Use specific visualization mode
cargo run -- --mode waterfall your-song.wav

# Custom color palette
cargo run -- --palette neon your-song.ogg
```

## Project Structure

```
src/
├── main.rs          # Entry point and CLI parsing
├── lib.rs           # Library exports
├── audio/           # Audio decoding and processing
│   ├── decoder.rs   # Format-specific decoders
│   └── resampler.rs # Sample rate conversion
├── visualization/   # FFT and rendering
│   ├── fft.rs       # Fast Fourier Transform implementation
│   ├── modes.rs     # Different visualization algorithms
│   └── palette.rs   # Color scheme definitions
└── tui/             # Terminal UI components
```

## Testing

Run the test suite:

```bash
cargo test
```

Benchmark FFT performance:

```bash
cargo bench
```

## Future Enhancements

- [ ] Microphone input support
- [ ] Video output mode
- [ ] Plugin system for custom visualizations
- [ ] Multi-device synchronization

---

Built with ❤️ using Rust and love for terminal aesthetics.