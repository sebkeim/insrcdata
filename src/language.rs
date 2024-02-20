// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// target language abstraction
//

use crate::basetype::BaseType;
use crate::{aperror, langc, langrust, langswift, table};
use std::path::Path;
use std::{fs, io};

pub trait Language {
    fn emit(&self, project: &table::Project) -> aperror::Result<()>;

    fn extension(&self) -> String;

    fn dst_modified(&self, project: &table::Project) -> aperror::Result<std::time::SystemTime> {
        let metadata = fs::metadata(&project.dst_path)?;
        let modified = metadata.modified()?;
        Ok(modified)
    }
    // support tolabel for label format
    fn to_label(&self) -> bool {
        false
    }

    fn emit_enum(&self, _typ: &BaseType, _label: &str) -> String {
        "TO LABEL UNSUPORTED".to_string()
    }

    // for language that use C binding
    fn c_binding_header(
        &self,
        _project: &table::Project,
        _output: &mut dyn io::Write,
    ) -> io::Result<()> {
        Ok(())
    }
    fn c_binding_impl(
        &self,
        _project: &table::Project,
        _output: &mut dyn io::Write,
    ) -> io::Result<()> {
        Ok(())
    }
}

// compute target language from dest filename extension
pub fn language_for_dest(filename: &Path) -> &'static dyn Language {
    match filename
        // .as_path()
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
    {
        "rs" => langrust::RUST,
        "c" => langc::LANG_C,
        "swift" => langswift::LANG_SWIFT,
        _ => langrust::RUST,
    }
}

pub fn digits(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let t = n as f64;
        t.log10().ceil() as usize
    }
}

pub fn file_notice() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    format!("generated by insrcdata version {}", VERSION)
}

// write doc comment : prefix each line with comment mark
pub fn write_help(
    output: &mut dyn io::Write,
    prefix: &str,
    doc: &Option<String>,
) -> io::Result<()> {
    if let Some(doc) = doc {
        for row in doc.split('\n') {
            writeln!(output, "{prefix}{row}")?;
        }
    }
    Ok(())
}
