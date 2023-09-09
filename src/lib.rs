#[macro_use]
extern crate serde_derive;
extern crate heck;
extern crate toml;

use std::env;
use std::path::Path;

mod aperror;
mod basetype;
mod colbool;
mod colfloat;
mod colint;
mod coljoin;
mod collabel;
mod colobject;
mod colstr;
mod colvariant;
mod config;
mod index;
mod langc;
mod langrust;
mod language;
mod lint;
pub mod log;
mod table;

pub use aperror::Result;

fn emit(runtime: config::Runtime) -> Result<()> {
    let dest = env::var("OUT_DIR").unwrap_or("".to_string());

    let runtime = runtime
        .indir("".to_string()) // FIXME: this code is only used to silent dead code warning
        .dest(dest + "/");
    let project = runtime.into_project()?;

    // Tell Cargo that if the given file changes, to rerun the build script.
    for path in &project.src_paths {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap_or("."));
    }

    // check modification
    if !project.modified() {
        log::warning("unneeded insrcdata call");
    }

    // generate code
    project.emit()?;

    Ok(())
}

/// Hight level API for build.rs scripts
///   https://doc.rust-lang.org/cargo/reference/build-scripts.html
pub fn read(path: &str) -> Result<()> {
    let runtime = config::Runtime::read(Path::new(path));
    emit(runtime)
}

pub fn build(toml: &str) -> Result<()> {
    let runtime = config::Runtime::from_toml(toml.to_string(), Path::new("."));

    emit(runtime)
}
