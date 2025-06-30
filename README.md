# 💼 Super Deal

**Super Deal** is a Rust-based CLI implementation of the classic game show *Deal or No Deal*. Choose a mystery case, eliminate others, and negotiate with the banker to walk away with the highest offer — or risk everything to see what your case holds.

---

## 🎮 Gameplay Features

- 26 mystery cases with randomized monetary values
- Interactive CLI interface with styled output using `colored`
- Round-based progression with bank offers every few cases
- Player decisions: accept or reject offers
- Dynamic win evaluation at game end

---

## 🧠 Core Concepts & Design

This project demonstrates:
- **Rust ownership and borrowing** through game state management
- **Modular design** using `Player`, `Cases`, and `Banker` structs
- **CLI user interaction** with colored prompts and formatted banners
- **Finite state machine (FSM)**-style control flow using a custom `Action` enum
- **Error handling and input validation** for a smooth user experience

---

## 🧱 Architecture

The game is structured around a clean separation of logic and presentation:


Planned architecture uses a `GameDisplay` trait for CLI/HTML/GUI decoupling.

---

## 🛠️ Tech Stack

- 🦀 [Rust](https://www.rust-lang.org/)
- 🎨 [`colored`](https://crates.io/crates/colored) — CLI text styling
- 🎲 [`rand`](https://crates.io/crates/rand) — for case shuffling
- (Planned) 🎶 [`rodio`](https://crates.io/crates/rodio) — music playback
- (Planned) 🧠 `serde` for account storage

---

## 🚀 Getting Started

### 🔧 Prerequisites

- Rust (≥ 1.70): [Install Rust](https://www.rust-lang.org/tools/install)

### 📦 Build and Run

```bash
git clone https://github.com/yourusername/super-deal.git
cd super-deal
cargo run
