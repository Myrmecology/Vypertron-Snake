🐍 Vypertron Snake
A modern twist on the classic Snake game, built with Rust and featuring dynamic AI opponents, progressive difficulty, and an immersive audiovisual experience.

🎮 Features

Classic Snake Gameplay with modern enhancements
AI Opponents - CPU-controlled snakes that increase in number as you progress
Infinite Levels - Progressive difficulty with speed increases and more CPU snakes
Dynamic Themes - Each level features unique color schemes
Original Soundtrack - Custom title screen and gameplay music
Responsive Grid - Large 40x30 playing field
Score Tracking - Track your tails collected and see your progress

🚀 Getting Started
Prerequisites

Rust (latest stable version)
Cargo (comes with Rust)

Installation

Clone the repository:

bashgit clone https://github.com/Myrmecology/Vypertron-Snake.git
cd vypertron-snake

Build and run the game:

bashcargo run --release
The --release flag is recommended for optimal performance.
🎯 How to Play
Controls

Arrow Keys - Control your snake's direction
SPACE - Start the game from the title screen

Objective

Eat the red food to grow your snake
Avoid hitting walls or your own tail
Navigate around CPU snakes (they won't hurt you, but add to the challenge!)
Collect 5 food items to advance to the next level

Level Progression

Levels 1-4: 1 CPU snake
Levels 5-9: 2 CPU snakes
Levels 10-14: 3 CPU snakes
Levels 15-19: 4 CPU snakes
Level 20+: 5 CPU snakes (maximum)

Each level increases your snake's speed using a logarithmic curve, ensuring the game remains challenging but playable at higher levels.
🛠️ Built With

Rust - Systems programming language focusing on safety and performance
Macroquad - Simple and easy to use game library for Rust
rand - Random number generation for CPU snake behavior

📁 Project Structure
vypertron-snake/
├── src/
│   ├── main.rs          # Game loop and state management
│   ├── snake.rs         # Player snake logic
│   ├── cpu_snake.rs     # AI opponent logic
│   ├── food.rs          # Food spawning system
│   ├── grid.rs          # Game grid and rendering
│   ├── themes.rs        # Color themes for each level
│   ├── level.rs         # Level progression system
│   └── effects.rs       # Visual effects
├── assets/
│   ├── snake_head.png   # Title screen graphic
│   ├── Snake_title.wav  # Title screen music
│   └── snake_game.wav   # Gameplay music
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
🎨 Features in Detail
Dynamic Difficulty
The game implements a sophisticated difficulty curve:

Snake speed increases logarithmically with each level
Maximum speed is capped at 3x the original for playability
CPU snakes become faster and more numerous

Visual Themes
Each level features a unique color palette:

Classic green snake (Level 1)
Sunset orange (Level 2)
Cyberpunk purple (Level 3)
Arctic ice (Level 4)
And 6 more unique themes that cycle every 10 levels

Audio System

Original soundtrack created in BeepBox
Seamless music transitions between menus and gameplay
Volume-balanced audio mixing

🤝 Contributing
Contributions are welcome! Feel free to:

Report bugs
Suggest new features
Submit pull requests

📝 License
This project is open source and available under the MIT License.
🙏 Acknowledgments

Inspired by the classic Snake game
Built with the amazing Macroquad game engine
Special thanks to the Rust community


Enjoy playing Vypertron Snake! 🐍🎮

You can see this project in action at my YouTube channel https://www.youtube.com/watch?v=RWg3_3ZnAQw&t=2s

Happy coding