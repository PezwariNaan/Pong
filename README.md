# Pong Game in Rust with Tetra

This repository hosts a classic Pong game implemented in Rust using the [Tetra](https://tetra.seventeencups.net/) game development framework. The game features basic paddle and ball mechanics, along with collision-based "spin" for a challenging gameplay experience.

## Features

- **Simple Controls**: Move paddles to block and reflect the ball.
- **Collision Spin**: Ball spin based on paddle-ball collision position.
- **Single or Multiplayer**: Play against another player locally.
- **Win Condition**: If the ball passes a paddle, the other player wins.
- **Customizable Speeds**: Ball speed and acceleration parameters are easily adjustable.

## Project Structure

- **`main.rs`**: Main game logic, including paddle and ball movement, collision detection, and win condition.
- **`resources/`**: Contains images for the paddle and ball sprites.

## Gameplay

**Controls**:
- **Player 1**: `W` (up), `S` (down)
- **Player 2**: `Up Arrow` (up), `Down Arrow` (down)

**Win Condition**: If the ball goes past a paddle, the other player wins and the game exits with a win message in the console.

## How to Run

### Prerequisites

To compile and run this project, you’ll need:
- Rust (latest stable version)
- Tetra library

### Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/username/repository-name
   cd repository-name
   cargo -r