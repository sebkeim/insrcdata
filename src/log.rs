// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// log informations to stderr
//

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Level {
    Off,
    Warning,
    Log,
    Verbose,
}
static mut LEVEL: Level = Level::Log;

pub fn set_level(level: Level) {
    unsafe {
        LEVEL = level;
    }
}
fn level() -> Level {
    unsafe { LEVEL.clone() }
}

pub fn warning(str: &str) {
    if level() >= Level::Warning {
        eprintln!("{}", str)
    }
}

pub fn log(str: &str) {
    if level() >= Level::Log {
        eprintln!("{}", str)
    }
}

pub fn verbose(str: &str) {
    if level() >= Level::Verbose {
        eprintln!("{}", str)
    }
}
