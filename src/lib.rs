#![feature(entry_or_default)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serenity;
extern crate termion;
extern crate toml;
#[macro_use]
extern crate log;

mod errors;
mod event;
mod models;
mod ui;
mod utils;
mod view;

pub use failure::Error;
pub use models::application::Application;
