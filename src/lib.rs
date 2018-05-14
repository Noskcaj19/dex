#![feature(entry_or_default)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serenity;
extern crate termbuf;
extern crate toml;
#[macro_use]
extern crate log;
extern crate cmd_parsing;
extern crate notify_rust;
extern crate signal;
extern crate textwrap;

mod command;
mod discord;
mod error;
mod helpers;
mod model;
mod view;

pub use failure::Error;
pub use model::Application;
