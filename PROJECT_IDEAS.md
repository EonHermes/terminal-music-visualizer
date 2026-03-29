# Project Ideas 🚀

_A living list of projects to build. Status: TODO | WIP | DONE_

## Current Projects

### 1. **Terminal Music Visualizer** 🎵
A Rust terminal app that visualizes music playback in real-time using ASCII art and color gradients. Reads audio from system or file, displays frequency spectrum with smooth animations.
- **Status:** DONE ✅
- **Tech:** Rust, crossterm, rodio
- **Why:** Beautiful, practical, showcases terminal UI skills
- **Repo:** https://github.com/EonHermes/terminal-music-visualizer

### 2. **Smart Clipboard Manager** 📋
Desktop app that learns what you copy and suggests contextually relevant clips. AI-powered search, automatic categorization, cross-device sync.
- **Status:** DONE ✅
- **Tech:** Python/FastAPI, SQLAlchemy, scikit-learn, sentence-transformers
- **Why:** Daily utility with AI twist
- **Repo:** https://github.com/EonHermes/smart-clipboard-manager

### 3. **Code Review Bot** 🤖
GitHub bot that automatically reviews PRs for common issues, suggests improvements, checks test coverage. Learns from your codebase patterns.
- **Status:** WIP 🔨
- **Tech:** Rust, GitHub API, ML for pattern recognition
- **Why:** Automates tedious work, improves code quality

### 4. **Personal Knowledge Graph** 🧠
Visual network graph of your notes, bookmarks, and thoughts. Auto-links related concepts, suggests connections you might have missed.
- **Status:** DONE ✅
- **Tech:** React/TypeScript frontend (vis-network), Rust/Axum backend, SQLite
- **Why:** Second brain with visual insights
- **Repo:** https://github.com/EonHermes/personal-knowledge-graph
- **Features:** Interactive graph visualization, CRUD for notes/bookmarks, auto-suggested connections, search, tagging system

### 5. **Automated Music Curator** 🎶
ML system that analyzes your listening habits and automatically creates playlists for different moods/activities. Discovers new music based on patterns.
- **Status:** TODO
- **Tech:** Python/Rust, Spotify API, ML models
- **Why:** Personalized music discovery

### 6. **Terminal Dashboard** 📊
Customizable terminal dashboard showing weather, calendar, system stats, GitHub activity, todo list. All in one beautiful TUI.
- **Status:** DONE ✅
- **Tech:** Rust, ratatui, crossterm, reqwest, sysinfo, chrono
- **Why:** Productivity hub in your terminal - brings all essential info to one place
- **Repo:** https://github.com/EonHermes/terminal-dashboard
- **Features:** Tabbed TUI interface with weather widget (Open-Meteo API), system stats gauges (CPU/memory/disk), GitHub activity feed, calendar events manager, todo list, auto-refreshing data, keyboard-driven controls, comprehensive test suite (27 tests passing)
- **Tech:** Rust, ratatui, multiple APIs
- **Why:** Productivity hub in your terminal

### 7. **Smart Home Energy Monitor** ⚡
Track and optimize home energy usage. Predicts consumption patterns, suggests optimizations, integrates with smart devices.
- **Status:** DONE ✅
- **Tech:** Rust, Axum, SQLite, ML/statrs for forecasting
- **Why:** Practical, saves money, environmental impact
- **Repo:** https://github.com/EonHermes/smart-home-energy-monitor
- **Features:** RESTful API for consumption tracking, ML-powered 24-hour forecasts using linear regression, optimization suggestions engine, anomaly detection, comprehensive test suite, GitHub Actions CI/CD

### 8. **Code Snippet Knowledge Base** 💾
Local-first snippet manager with semantic search. Tags automatically based on code content, suggests snippets when you're coding.
- **Status:** TODO
- **Tech:** Rust, SQLite, embeddings for semantic search
- **Why:** Never lose good code again

### 9. **Automated Documentation Generator** 📝
Analyzes your codebase and generates beautiful, up-to-date documentation. Includes examples, diagrams, and API references.
- **Status:** TODO
- **Tech:** Rust/Python, AST parsing, React docs site
- **Why:** Saves hours of tedious work

### 10. **Personal Finance Tracker** 💰
Automated expense tracking from bank statements/receipts. Categorizes spending, predicts future expenses, suggests savings opportunities.
- **Status:** TODO
- **Tech:** Rust, OCR for receipts, ML for categorization
- **Why:** Financial clarity without manual entry

### 11. **Terminal Chat Client** 💬
Beautiful terminal-based chat client supporting multiple platforms (Discord, Slack, etc.). Privacy-focused, keyboard-driven.
- **Status:** TODO
- **Tech:** Rust, various APIs, TUI framework
- **Why:** Productivity without leaving terminal

### 12. **AI Pair Programming Assistant** 👥
Local LLM that understands your codebase and helps with refactoring, debugging, and feature suggestions. Privacy-first alternative to GitHub Copilot.
- **Status:** DONE ✅
- **Tech:** Rust/Python, local LLM, IDE integration
- **Why:** Coding superpower without data leaving your machine
- **Repo:** https://github.com/EonHermes/ai-pair-programmer

### 13. **Automated Testing Framework** ✅
Intelligent test generator that analyzes code and creates comprehensive test suites. Learns from your testing patterns.
- **Status:** DONE ✅
- **Tech:** Python, AST analysis, pytest
- **Why:** Better coverage, less manual work
- **Repo:** https://github.com/EonHermes/automated-testing-framework
- **Features:** AST-based code analysis, automatic test generation (unit, edge cases, exceptions), coverage tracking and reporting, comprehensive test suite, CLI interface

### 14. **Music Production Assistant** 🎹
AI tool that helps with music production - suggests chord progressions, melodies, mixing tips based on your style.
- **Status:** TODO
- **Tech:** Python/Rust, audio processing, ML models
- **Why:** Creative tool for musicians

### 15. **System Health Predictor** 🔮
ML model that predicts system failures/issues before they happen. Analyzes logs, resource usage, patterns to warn you proactively.
- **Status:** DONE ✅
- **Tech:** Rust, tokio, ratatui, ML/statrs, notify
- **Why:** Prevents downtime and data loss - combines ML with computer engineering
- **Repo:** https://github.com/EonHermes/system-health-predictor
- **Features:** Real-time metrics collection (CPU/memory/disk/network/processes), log file analysis for error patterns, ML-based anomaly detection using statistical methods, trend analysis with linear regression, predictive failure forecasting, beautiful TUI with 5 tabs (Overview/Metrics/Alerts/Predictions/History), configurable alert thresholds, webhook/email/desktop notifications, headless mode for background service, comprehensive test suite, CI/CD with GitHub Actions
- **Tech:** Rust/Python, time-series ML, system monitoring
- **Why:** Prevent problems before they occur

### 16. **Recipe Generator from Pantry** 🍳
Input what ingredients you have, get recipe suggestions. Tracks pantry inventory, suggests shopping lists, learns your preferences.
- **Status:** TODO
- **Tech:** React mobile app, Rust backend, ML for recommendations
- **Why:** Practical daily utility

### 17. **Learning Path Generator** 📚
AI that creates personalized learning paths based on your goals, current level, and learning style. Tracks progress, adapts to pace.
- **Status:** TODO
- **Tech:** React frontend, Rust backend, ML for personalization
- **Why:** Accelerate skill development

---

## Notes

- Projects marked WIP are actively being developed
- DONE projects can be revisited for improvements
- New ideas should maintain diversity (terminal, desktop, mobile, web, ML)
- All projects should have: solid tests, good README, clean architecture

---

### 23. **Privacy-First Password Breach Monitor** 🔒
Real-time monitoring of your passwords against known data breaches without sending plaintext passwords to any server. Uses k-anonymity API (like HaveIBeenPwned), local hashing, and secure alerts. Integrates with password managers.
- **Status:** TODO
- **Tech:** Rust, cryptographic hashing (SHA-256, k-anonymity), React frontend, optional browser extension
- **Why:** Critical security utility, privacy-focused approach to breach monitoring, practical daily protection

### 24. **Distributed Compute Orchestrator** ⚡
Lightweight cluster manager for distributed computing tasks. Schedule CPU/GPU workloads across multiple machines in your home/lab. Perfect for rendering, ML training, or batch processing. Web UI + CLI.
- **Status:** DONE ✅
- **Tech:** Rust (Axum, Tokio, Bollard), React/TypeScript, Docker integration, Prometheus metrics
- **Why:** Fills gap in home lab tooling, practical for ML/compute-heavy workflows, showcases distributed systems skills
- **Repo:** https://github.com/EonHermes/distributed-compute-orchestrator
- **Features:** Priority-based task scheduling, resource-aware node placement, real-time React dashboard, Docker container execution, RESTful API, Prometheus metrics export, health monitoring, comprehensive test suite (7 tests), Docker deployment support
- **Tech:** Rust (Tokio, gRPC), React dashboard, Docker integration, Kubernetes-like scheduling
- **Why:** Fills gap in home lab tooling, practical for ML/compute-heavy workflows, showcases distributed systems skills

### 25. **Smart Notification Aggregator** 🔔
Unified notification hub that intelligently filters and prioritizes alerts from all your services (GitHub, email, calendar, monitoring). Learns what matters, suppresses noise, delivers critical stuff only.
- **Status:** TODO
- **Tech:** Rust backend, React mobile/web, ML for priority classification, integrations with major platforms
- **Why:** Solves notification fatigue, practical daily utility, combines ML with real-world problem

---

## Fresh Ideas Added 🆕

### 18. **Generative Art NFT Generator** 🎨
Create unique generative art pieces using procedural algorithms. Export as high-res images, metadata for NFTs, or print-ready files. Interactive canvas with real-time parameter tweaking.
- **Status:** TODO
- **Tech:** Rust (wgpu for GPU acceleration), React frontend, WebAssembly
- **Why:** Creative + technical, explores generative art trends, showcases GPU programming

### 19. **Real-Time Collaborative Code Whiteboard** 🤝
Browser-based collaborative whiteboard specifically for code discussions. Real-time sync, syntax highlighting, diagram drawing, voice chat integration. Perfect for remote pair programming sessions.
- **Tech:** React/TypeScript, Rust/WebSocket backend, CRDT for conflict-free sync
- **Why:** Fills gap in real-time collaboration tools, practical for remote work

### 20. **Biohacking Dashboard** 💪
Track health metrics (sleep, exercise, nutrition, supplements) with ML-powered insights. Integrates with wearables, suggests optimizations based on patterns. Privacy-first local storage option.
- **Status:** TODO
- **Tech:** React mobile/web, Rust backend, ML for pattern recognition, optional IoT integration
- **Why:** Combines health tech + ML + practical daily utility, growing market

### 21. **Local LLM Model Zoo Manager** 🦙
GUI tool to download, manage, and benchmark local LLM models. One-click model switching, performance comparison, hardware optimization suggestions. Perfect companion for the AI Pair Programmer.
- **Status:** DONE ✅
- **Tech:** Rust/Tauri backend, React/TypeScript frontend
- **Why:** Practical tool for the growing local LLM ecosystem, complements existing projects
- **Repo:** https://github.com/EonHermes/local-llm-zoo
- **Features:** Model discovery from Hugging Face/Ollama, comprehensive benchmarking (tokens/sec, memory usage), hardware detection with GPU support, intelligent model recommendations based on system specs, beautiful dark-themed UI, exportable benchmark results
- **Tech:** Rust/Tauri desktop app, HuggingFace API, ML model benchmarking
- **Why:** Practical tool for the growing local LLM ecosystem, complements existing projects

### 22. **Ephemeral File Sharing Network** 📡
Peer-to-peer file sharing without central servers. End-to-end encrypted, self-destructing links, QR code generation for quick transfers. Works even behind NAT with relay fallback.
- **Status:** WIP 🔨
- **Tech:** Rust (libp2p), WebRTC, React frontend, encryption crates
- **Why:** Privacy-focused, explores P2P networking, practical for secure transfers
- **Repo:** https://github.com/EonHermes/ephemeral-file-share
- **Progress:** Backend complete with all tests passing. Features: Axum HTTP API, ChaCha20-Poly1305 encryption, in-memory storage with auto-expiration, libp2p P2P layer, QR code generation, React frontend UI. 11 unit tests passing.

---