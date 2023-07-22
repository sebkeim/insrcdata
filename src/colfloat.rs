// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// floating point number data type column
//

use crate::language::Language;
use crate::{aperror, table};
use crate::{basetype, lint};

// ================================================================================================
// F64
// ================================================================================================
pub struct ColF64 {
    info: table::ColumnInfo,
    values: Vec<f64>,
}

impl table::Column for ColF64 {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        let v = &self.values[row];
        format!("{:?}", v)
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = Vec::from_iter(0..values.len());
        indexes.sort_by(|a, b| values[*a].total_cmp(&values[*b]));
        indexes
    }

    fn lint(&self, _linter: &lint::Linter) {}
}

impl ColF64 {
    pub fn parse(
        name: &str,
        strvals: &[String],
        iterable: bool,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let values = table::parse_vec::<f64>(strvals)?;
        Ok(Box::new(ColF64 {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::F64,
                table_type: basetype::BaseType::F64,
                iterable,
            },
            values,
        }))
    }
}

// ================================================================================================
// F32
// ================================================================================================
pub struct ColF32 {
    info: table::ColumnInfo,
    values: Vec<f32>,
}

impl table::Column for ColF32 {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        let v = &self.values[row];
        format!("{:?}", v)
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = Vec::from_iter(0..values.len());
        indexes.sort_by(|a, b| values[*a].total_cmp(&values[*b]));
        indexes
    }

    fn lint(&self, _linter: &lint::Linter) {}
}

impl ColF32 {
    pub fn parse(
        name: &str,
        strvals: &[String],
        iterable: bool,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let values = table::parse_vec::<f32>(strvals)?;
        Ok(Box::new(ColF32 {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::F32,
                table_type: basetype::BaseType::F32,
                iterable,
            },
            values,
        }))
    }
}
