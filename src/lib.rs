#[macro_use]
extern crate serde_derive;
extern crate heck;
extern crate toml;

use std::env;

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

/// Hight level API for build.rs scripts
///   https://doc.rust-lang.org/cargo/reference/build-scripts.html
pub fn emit(path: &str) -> Result<()> {
    // cargo build script destination,
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let dest = env::var("OUT_DIR").unwrap_or("".to_string());

    let runtime = config::Runtime::new(path)
        .indir("".to_string()) // FIXME: this code is only used to silent dead code warning
        .dest(dest + "/");
    let project = runtime.into_project()?;

    // Tell Cargo that if the given file changes, to rerun this build script.
    for path in &project.src_paths {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap_or("."));
    }

    // check modification
    if !project.modified() {
        log::warning("unneded insrcdata call");
    }

    // generate code
    project.emit()?;

    Ok(())
}
