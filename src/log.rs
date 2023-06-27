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
    Standard,
    Verbose,
}
static mut LEVEL: Level = Level::Standard;

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
    if level() >= Level::Standard {
        eprintln!("{}", str)
    }
}

pub fn verbose(str: &str) {
    if level() >= Level::Verbose {
        eprintln!("{}", str)
    }
}
