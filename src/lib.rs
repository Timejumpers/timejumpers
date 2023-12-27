#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(
    clippy::needless_return,
    clippy::multiple_crate_versions,
    clippy::too_many_arguments
)]
#![warn(dead_code)]

pub mod animation;
pub mod assets;
pub mod control;
pub mod enemy;
pub mod entities;
pub mod multiplayer;
pub mod player;
pub mod game;
pub mod camera;
