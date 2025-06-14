# 🐍⚡ Vypertron-Snake

**A premium Snake game that redefines the classic - built with 100% Rust and Bevy Engine!**

## 🎮 Game Features

### 🏠 **Epic Home Screen**
- **Classic NES-inspired design** with retro aesthetics
- **Animated snake wrapped around the game title**
- **Press SPACEBAR to start your adventure**

### 🦎 **Character Selection**
- **4 unique snake characters** to choose from
- **Distinct colors and visual styles** for each character
- **Personalize your gameplay experience**

### 🎯 **10 Immersive Levels**
- **Unique themes** for each level
- **Progressive difficulty** that keeps you engaged
- **Beautiful environments** that evolve as you progress

### 🎬 **Cinematic Experience**
- **Cutscenes between levels** for narrative flow
- **Smooth animations** throughout gameplay
- **Explosive death effects** when your snake meets its end

### 🎵 **Custom Audio Experience**
- **Original soundtrack** created in BeepBox
- **Rust-generated sound effects** for authentic retro feel
- **Immersive audio** that responds to gameplay

### 🏆 **Advanced Features**
- **High score tracking** with persistent storage
- **Pause and resume** functionality (SPACEBAR)
- **Smooth performance** on both desktop and web
- **Professional game architecture** with clean code

## 🎮 Controls

| Key | Action |
|-----|--------|
| **SPACEBAR** | Start Game / Pause / Resume |
| **↑ Arrow** | Move Up |
| **↓ Arrow** | Move Down |
| **← Arrow** | Move Left |
| **→ Arrow** | Move Right |

## 🛠️ Built With

- **🦀 Rust** - 100% memory-safe systems programming
- **🎨 Bevy Engine** - Modern, data-driven game engine
- **🌐 WebAssembly** - Browser deployment ready
- **🎵 Custom Audio** - BeepBox compositions + Rust SFX

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ installed
- Modern web browser (for WASM deployment)

### Desktop Build
```bash
cargo run --features desktop
```

### Web Build
```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build for web
cargo build --release --target wasm32-unknown-unknown --features web

# Serve locally (you'll need a local server)
# Example with Python:
# python -m http.server 8000
```

## 🎯 Project Structure

```
Vypertron-Snake/
├── assets/           # Game assets
│   ├── audio/        # Music and sound effects
│   ├── textures/     # Sprites and images
│   └── fonts/        # Typography
├── src/
│   ├── states/       # Game state management
│   ├── systems/      # Game logic systems
│   ├── components/   # ECS components
│   ├── resources/    # Global resources
│   ├── levels/       # Level definitions
│   ├── audio/        # Audio systems
│   └── utils/        # Helper functions
└── README.md
```

## 🎨 Game Design Philosophy

**Vypertron-Snake** takes the timeless Snake concept and elevates it to premium indie game standards. Every aspect has been carefully crafted:

- **Visual Excellence**: Clean, modern graphics with retro charm
- **Audio Immersion**: Custom music and procedural sound effects
- **Progressive Gameplay**: 10 levels that challenge and delight
- **Technical Excellence**: Built with Rust for performance and safety
- **Accessibility**: Runs smoothly in browsers and on desktop

## 🌟 What Makes It Special

This isn't just another Snake game. **Vypertron-Snake** showcases:

- **Modern Rust game development** with Bevy ECS architecture
- **Cross-platform deployment** (desktop + web browser)
- **Professional game development practices**
- **Custom asset pipeline** with original music and graphics
- **Immersive storytelling** through cutscenes and progression
- **Polished user experience** from start to finish

## 🎯 Development Goals

- ✅ **100% Rust implementation** - Memory safety and performance
- ✅ **Browser deployment** - Accessible to everyone
- ✅ **Premium feel** - AAA polish on a classic concept
- ✅ **Educational value** - Showcase modern game development
- ✅ **Open source** - Learn from and contribute to the codebase

## 🤝 Contributing

This project demonstrates advanced Rust game development techniques. Feel free to:

- **Study the code architecture**
- **Suggest improvements**
- **Report issues**
- **Share your own Snake variants**

## 📜 License

MIT License - Feel free to learn from and build upon this project!

## 🎮 Play Now

**Ready to experience Snake like never before?**

*Coming soon to browsers everywhere!*

---

**Built with ❤️ and 🦀 Rust**