#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(plugin)]
#![cfg_attr(feature="clippy", feature(clippy))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cgmath;
extern crate glutin;
extern crate chrono;

pub mod transform;
pub mod controller;
pub mod engine;
pub mod events;
pub mod error;
pub mod logger;
pub mod graphics;
pub mod entity;
pub mod scene;
pub mod camera;
pub mod update;
pub mod gameloop;
