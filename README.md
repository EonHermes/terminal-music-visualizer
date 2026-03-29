# Terminal Music Visualizer 🎵

A beautiful, real-time music visualizer built in Rust that brings your terminal to life with colorful frequency visualizations.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux%2FmacOS-lightgrey.svg)

## Features

- 🎨 **Multiple Visualization Modes**: Bars, Waveform, Spectrum, and Circular views
- 🌈 **Color Palettes**: Rainbow, Heat, Grayscale, Ocean, Sunset, and Neon themes
- 📁 **Multi-format Support**: WAV and MP3 audio files
- ⚡ **Real-time FFT Analysis**: Fast Fourier Transform for frequency decomposition
- 🎛️ **Interactive Controls**: Keyboard-driven navigation and customization
- 🔄 **Looping Playback**: Automatic looping when audio ends
- 🎯 **Smooth Animations**: ~60fps rendering with peak decay effects

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (Rust package manager)
- **System dependencies** (for audio support):
  - Linux: `pkg-config`, `libasound2-dev` (ALSA development files)
  - macOS: Xcode command line tools
  - Windows: Visual Studio Build Tools

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Linux (Ubuntu/Debian)
sudo apt install pkg-config libasound2-dev

# macOS
xcode-select --install

# Windows
# Install Visual Studio Build Tools with C++ workload
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/EonHermes/terminal-music-visualizer.git
cd terminal-music-visualizer

# Build in release mode (recommended for performance)
cargo build --release

# Run the visualizer
./target/release/terminal_music_visualizer <audio_file>
```

### Quick Start

```bash
# Play a music file
cargo run --release -- your-song.mp3

# Or with WAV files
cargo run --release -- soundtrack.wav
```

## Usage

### Basic Command

```bash
terminal_music_visualizer <audio_file>
```

### Supported Formats

- **WAV** (`.wav`) - Uncompressed audio, best quality
- **MP3** (`.mp3`) - Compressed audio, widely available

### Keyboard Controls

| Key | Action |
|-----|--------|
| `Q` / `Esc` | Quit the visualizer |
| `M` | Cycle to next visualization mode |
| `P` | Cycle to next color palette |
| `+` / `=` | Increase sensitivity (coming soon) |
| `-` | Decrease sensitivity (coming soon) |
| `←` | Seek backward 10% |
| `→` | Seek forward 10% |
| `Space` | Pause/Resume (coming soon) |
| `R` | Reset view (coming soon) |

### Visualization Modes

Press **M** to cycle through:

1. **Bars** - Vertical frequency bars with customizable styles
   - Default: Simple filled bars
   - Gradient: Color-intensity gradient fill
   - Outlined: Bars with peak indicators
   - Rounded: Smooth, rounded bar appearance

2. **Waveform** - Time-domain amplitude display

3. **Spectrum** - Horizontal frequency spectrum with labels

4. **Circular** - Radial visualization with concentric rings

### Color Palettes

Press **P** to cycle through:

1. **Rainbow** - Full spectrum gradient (red → yellow → green → cyan → blue → magenta)
2. **Heat** - Temperature-based (blue → cyan → yellow → red)
3. **Grayscale** - Monochrome intensity
4. **Ocean** - Cool blues and cyans
5. **Sunset** - Warm oranges, pinks, and purples
6. **Neon** - Bright saturated colors

## Architecture

```
terminal-music-visualizer/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── lib.rs            # Library exports
│   ├── audio/
│   │   ├── mod.rs        # Audio module interface
│   │   ├── decoder.rs    # WAV/MP3 decoding with hound & rodio
│   │   └── resampler.rs  # Sample rate conversion
│   ├── visualization/
│   │   ├── mod.rs        # Visualization controller
│   │   ├── fft.rs        # FFT frequency analysis (rustfft)
│   │   ├── modes.rs      # Visualization modes & styles
│   │   └── palette.rs    # Color palette definitions
│   ├── tui/
│   │   ├── mod.rs        # Terminal UI main loop
│   │   ├── renderer.rs   # Terminal rendering engine
│   │   └── input.rs      # Keyboard input handling
│   └── utils/
│       ├── mod.rs        # Utility exports
│       ├── math.rs       # Math helpers (RMS, windowing)
│       └── time.rs       # Timing utilities
├── tests/
│   └── integration_tests.rs  # Integration test suite
├── Cargo.toml            # Project dependencies
└── README.md             # This file
```

### Data Flow

```
Audio File → Decoder → Samples → FFT Analyzer → Frequency Bins
                                              ↓
                                    Visualization Controller
                                              ↓
                                      Renderer (TUI)
                                              ↓
                                       Terminal Output
```

## Technical Details

### FFT Analysis

The visualizer uses the `rustfft` crate for Fast Fourier Transform analysis:

- **FFT Size**: Automatically selected based on output bin count (1024, 2048, or 4096)
- **Windowing**: Hann window applied to reduce spectral leakage
- **Binning**: Logarithmic frequency spacing for better audio representation
- **Normalization**: Output values scaled to [0.0, 1.0] range

### Performance

- **Target FPS**: ~60 frames per second
- **Audio Buffer Size**: 2048 samples (configurable)
- **Frequency Bins**: 64 default (adjustable)
- **Rendering**: Direct terminal manipulation via `crossterm`

### Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| crossterm | 0.27 | Terminal handling & TUI |
| rodio | 0.17 | Audio playback & decoding |
| hound | 3.5 | WAV file support |
| rustfft | 6.1 | FFT implementation |
| colored | 2.0 | Color output utilities |
| serde | 1.0 | Configuration serialization |
| toml | 0.8 | Config file parsing |
| thiserror | 1.0 | Error type definitions |
| anyhow | 1.0 | Error handling |
| crossbeam-channel | 0.5 | Thread-safe channels |

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_fft_pipeline

# Run benchmarks (if available)
cargo bench
```

### Test Coverage

- ✅ FFT analysis accuracy
- ✅ Audio decoding (WAV/MP3)
- ✅ Visualization frame generation
- ✅ Color palette cycling
- ✅ Mode switching
- ✅ Mathematical utilities (RMS, peak, normalization)
- ✅ Windowing functions
- ✅ Integration tests for full pipeline

## Examples

### Visualizing Different Music Types

**Electronic Music**: The frequency bars will show strong bass response with dynamic mid/high frequencies.

**Classical Music**: More balanced distribution across all frequency ranges with smooth transitions.

**Vocal Tracks**: Prominent mid-range frequencies (200Hz - 2kHz) where vocals typically sit.

### Custom Visualizations

Create your own color palettes by modifying `src/visualization/palette.rs`:

```rust
pub enum ColorPalette {
    // Add your custom palette here!
    Cyberpunk,  // Pink and cyan neon
    Forest,     // Greens and browns
}
```

## Contributing

Contributions are welcome! Here's how to get started:

1. **Fork** the repository
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with tests where applicable
4. **Run tests**: `cargo test`
5. **Commit**: `git commit -m 'Add amazing feature'`
6. **Push**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Guidelines

- Follow Rust idioms and best practices
- Add tests for new functionality
- Update documentation for public APIs
- Keep commits focused and descriptive
- Use `cargo fmt` and `cargo clippy` before submitting

### Code Style

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for common issues
cargo deny check
```

## Roadmap

- [ ] Add system audio input (microphone/line-in)
- [ ] Implement pause/resume functionality
- [ ] Add sensitivity controls
- [ ] Support more audio formats (FLAC, OGG)
- [ ] Create preset configurations
- [ ] Add save/export visualization snapshots
- [ ] Multi-window support for split views
- [ ] WebAssembly build for browser playback

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **crossterm** team for excellent terminal manipulation library
- **rustfft** for high-performance FFT implementation
- **rodio** and **hound** for audio decoding
- The Rust community for an amazing ecosystem

---

Built with ❤️ by [Eon Hermes](https://github.com/EonHermes)

Enjoy the music! 🎶
