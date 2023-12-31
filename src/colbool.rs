// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// bolean data type column
//

use crate::language::Language;
use crate::table::ColumnConfig;
use crate::{aperror, table};
use crate::{basetype, lint};

pub struct ColBool {
    info: table::ColumnInfo,
    values: Vec<bool>,
}

impl table::Column for ColBool {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        if self.values[row] {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = Vec::from_iter(0..values.len());

        indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
        indexes
    }

    fn lint(&self, _linter: &lint::Linter) {}
}

pub fn parse_vec_bool(strvals: &[String]) -> aperror::Result<Vec<bool>> {
    let mut vals: Vec<bool> = vec![];

    for (i, str) in strvals.iter().enumerate() {
        match str.to_lowercase().as_str() {
            "1" | "true" | "yes" => vals.push(true),
            "0" | "false" | "no" => vals.push(false),
            x => {
                return Err(aperror::Error::new(&format!(
                    "invalid bool value {} at row {}",
                    x, i
                )))
            }
        }
    }
    Ok(vals)
}

impl ColBool {
    pub fn parse(
        config: ColumnConfig,
        strvals: &[String],
    ) -> aperror::Result<Box<dyn table::Column>> {
        let values = parse_vec_bool(strvals)?;
        Ok(Box::new(ColBool {
            info: table::ColumnInfo {
                config,
                len: values.len(),
                interface_type: basetype::BaseType::Bool,
                table_type: basetype::BaseType::Bool,
            },
            values,
        }))
    }
}
