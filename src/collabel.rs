// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// define row labels
//

use crate::table;
use crate::{basetype, lint};
use heck::ToShoutySnakeCase;
use std::collections::HashSet;

pub struct ColLabel {
    info: table::ColumnInfo,
    labels: Vec<String>,
}

impl table::Column for ColLabel {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, _row: usize) -> String {
        "EMIT_TABLE_CELL_UNSUPORTED".to_string()
    }
    fn emit_label(&self, row: usize) -> String {
        self.labels[row].to_string()
    }

    fn indexes(&self) -> Vec<usize> {
        vec![]
    }
    fn lint(&self, linter: &lint::Linter) {
        let mut labels: HashSet<String> = HashSet::new();
        for (i, v) in self.labels.iter().enumerate() {
            if !v.is_empty() {
                let upper = v.to_shouty_snake_case();
                linter.row(i, |lt| {
                    lt.err(lint::label(&upper), &format!("invalid label  {}", v));
                    lt.err(
                        !labels.contains(&upper),
                        &format!("duplicate label {} ({})", v, upper),
                    );
                });
                labels.insert(upper);
            }
        }
    }
}

impl ColLabel {
    pub fn new(namespace: &str, labels: Vec<String>) -> ColLabel {
        ColLabel {
            info: table::ColumnInfo {
                name: "".to_string(),
                len: labels.len(),
                interface_type: basetype::BaseType::Label {
                    name: namespace.to_string(),
                },
                table_type: basetype::BaseType::Label {
                    name: namespace.to_string(),
                },
                iterable: false,
            },
            labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::Column;

    #[test]
    fn label_ok() {
        let c = ColLabel::new("", vec!["hello".to_string()]);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 0);
    }

    #[test]
    fn label_invalid() {
        let c = ColLabel::new("", vec!["0hello".to_string()]);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn label_duplicate() {
        let c = ColLabel::new("", vec!["hello".to_string(), "HELLO".to_string()]);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }
}
