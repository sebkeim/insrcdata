// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// jointure column
//
use crate::language::Language;
use crate::table;
use crate::table::ColumnConfig;
use crate::{basetype, lint};
use std::collections::HashMap;

pub struct ColJoin {
    info: table::ColumnInfo,
    values: Vec<usize>,
    reverse_name: String,
}

impl table::Column for ColJoin {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        let v = &self.values[row];
        if self.optional() {
            v.to_string()
        } else {
            (v - 1).to_string()
        }
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = if self.optional() {
            Vec::from_iter((0..values.len()).filter(|a| values[*a] != 0))
        } else {
            Vec::from_iter(0..values.len())
        };
        indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
        indexes
    }

    fn lint(&self, linter: &lint::Linter) {
        linter.err(
            lint::label(&self.info.join_table()),
            "invalid table name for join",
        );
        if !self.optional() {
            for (i, v) in self.values.iter().enumerate() {
                linter.row(i, |lt| {
                    lt.err(*v != 0, "undefined link");
                });
            }
        }
    }

    fn reverse_name(&self) -> String {
        self.reverse_name.to_string()
    }
}

impl ColJoin {
    pub fn new(
        mut config: ColumnConfig,
        values: &Vec<String>,
        dest_table: &str,
        dest_keys: &Vec<String>,

        reverse: &str,
    ) -> ColJoin {
        let mut keyindexes = HashMap::new();
        for (i, x) in dest_keys.iter().enumerate() {
            keyindexes.insert(x, i + 1);
        }
        let indexes: Vec<usize> = values
            .iter()
            .map(|x| *keyindexes.get(x).unwrap_or(&0))
            .collect();

        let max = dest_keys.len() + (config.optional as usize);

        config.iterable = !reverse.is_empty();
        ColJoin {
            info: table::ColumnInfo {
                config,
                len: values.len(),
                interface_type: basetype::BaseType::Join {
                    strname: dest_table.to_string(),
                },
                table_type: basetype::int_type_for_range(0..=max as i64),
            },
            values: indexes,
            reverse_name: reverse.to_string(),
        }
    }
}
