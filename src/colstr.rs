// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// string data type column
//

use crate::language::Language;
use crate::{aperror, table};
use crate::{basetype, lint};

pub struct ColStr {
    info: table::ColumnInfo,
    values: Vec<String>,
}

impl table::Column for ColStr {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
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
    pub fn parse(
        name: &str,
        values: &Vec<String>,
        iterable: bool,
    ) -> aperror::Result<Box<dyn table::Column>> {
        Ok(Box::new(ColStr {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::Str,
                table_type: basetype::BaseType::Str,
                iterable,
                optional: false,
            },
            values: values.to_owned(),
        }))
    }
}
