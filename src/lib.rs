use chrono::{Local, SubsecRound};
use std::thread;
use async_std::task;

pub mod wiki;

pub fn debug_with_context(str: &str) {
    println!("{}: {} {:?} {:?}", Local::now().time().round_subsecs(3), str, thread::current(), task::current());
}