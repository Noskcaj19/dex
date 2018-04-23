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
extern crate cmd_parsing;
extern crate notify_rust;
extern crate signal;
extern crate textwrap;

mod commands;
mod discord;
mod errors;
mod helpers;
mod models;
mod view;

pub use failure::Error;
pub use models::application::Application;
