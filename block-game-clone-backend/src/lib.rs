#![no_std]
//! # Block Game Clone Backend
//!
//! Implements game logic for [block_game_clone](https://github.com/ethanSE/block_game_clone)
//!
//! Defines game state, player actions, logic for updating game state
//!
//! Generates WebAssembly code and TypeScript types for incorporating in web front end

pub mod board;
pub mod board_state;
pub mod game_mode;
pub mod game_state;
pub mod piece;
pub mod player;
pub mod player_hand_state;
pub mod player_state;
pub mod ts_interop;
