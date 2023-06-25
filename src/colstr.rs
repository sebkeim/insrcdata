// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// string data type column
//

use crate::table;
use crate::{basetype, lint};

pub struct ColStr {
    info: table::ColumnInfo,
    values: Vec<String>,
}

impl table::Column for ColStr {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize) -> String {
        // TODO : stability of String.Debug trait implementation is not guaranteed
        let v = &self.values[row];
        format!("{:?}", v)
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = Vec::from_iter(0..values.len());

        indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
        indexes
    }

    fn lint(&self, _linter: &lint::Linter) {}
}

impl ColStr {
    pub fn new(name: &str, values: Vec<String>, iterable: bool) -> ColStr {
        ColStr {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::Str,
                table_type: basetype::BaseType::Str,
                iterable,
            },
            values,
        }
    }
}
