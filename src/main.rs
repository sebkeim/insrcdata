// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_use]
extern crate serde_derive;
extern crate heck;
extern crate toml;

mod aperror;
mod basetype;
mod colint;
mod coljoin;
mod collabel;
mod colstr;
mod config;

mod colobject;
mod langc;
mod langrust;
mod language;
mod lint;
mod log;
mod table;

use clap::{Parser, ValueEnum};
use std::path::Path;

// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// project file path
    path: String,

    /// data source directory
    #[arg(short, long, default_value = "")]
    indir: String,

    /// destination file
    #[arg(short, long, default_value = "")]
    dest: String,

    /// check without writing
    #[arg(long)]
    lint: bool,

    /// print source code template
    #[arg(long)]
    interface: bool,

    /// force rebuild even if modifications detected
    #[arg(long)]
    rebuild: bool,

    /// log level
    #[arg(value_enum, default_value = "warning")]
    log: LogLevel,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevel {
    /// don't output logs
    Silent,
    /// only major notifications
    Warning,
    /// all potentially useful information  
    Standard,
    /// log anything
    Verbose,
}

fn main() -> aperror::Result<()> {
    let args = Args::parse();

    // log level
    log::set_level(match args.log {
        LogLevel::Silent => log::Level::Off,
        LogLevel::Warning => log::Level::Warning,
        LogLevel::Standard => log::Level::Standard,
        LogLevel::Verbose => log::Level::Verbose,
    });

    // parse other arguments

    let projectpath = Path::new(&args.path);

    let linter = lint::Linter::new();
    let runtime = config::Runtime {
        projectpath,
        indir: args.indir,
        dest: args.dest,
        linter: &linter,
    };

    let project = config::read(&runtime)?;

    project.lint(&linter);
    if linter.errors() > 0 {
        return Err(aperror::Error::new(&format!(
            "{} lint failures",
            linter.errors(),
        )));
    }

    // generate
    if args.interface {
        project.interface()?;
    } else if !args.lint && (args.rebuild || project.modified()) {
        project.emit()?;
    }

    Ok(())
}
