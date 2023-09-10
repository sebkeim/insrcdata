// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// define row labels
//

use crate::language::Language;
use crate::{aperror, table};
use crate::{basetype, lint};
use heck::ToShoutySnakeCase;
use std::collections::HashSet;

pub struct ColLabel {
    info: table::ColumnInfo,
    labels: Vec<String>,
    label_helps: Vec<String>,
}

impl table::Column for ColLabel {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, lang: &dyn Language) -> String {
        let label = &self.labels[row];
        lang.emit_enum(&self.info.table_type, label)
    }

    fn emit_label(&self, row: usize) -> String {
        self.labels[row].to_string()
    }
    fn emit_label_help(&self, row: usize) -> String {
        self.label_helps[row].to_string()
    }

    fn indexes(&self) -> Vec<usize> {
        vec![]
    }
    fn lint(&self, linter: &lint::Linter) {
        let allow_empty = self.info.name.is_empty();
        let mut labels: HashSet<String> = HashSet::new();
        for (i, v) in self.labels.iter().enumerate() {
            if v.is_empty() {
                linter.row(i, |lt| {
                    lt.err(allow_empty, "empty values disallowed by as_label option");
                });
            } else {
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
        linter.err(
            self.labels.len() == self.label_helps.len(),
            "invalid number of label helps",
        );
    }
}

impl ColLabel {
    pub fn parse(
        namespace: &str,
        labels: &[String],
        tolabel: &str,
        label_helps: &[String],
    ) -> aperror::Result<Box<dyn table::Column>> {
        Ok(Box::new(ColLabel {
            info: table::ColumnInfo {
                name: tolabel.to_string(),
                len: labels.len(),
                interface_type: basetype::BaseType::Label {
                    name: namespace.to_string(),
                },
                table_type: basetype::BaseType::Label {
                    name: namespace.to_string(),
                },
                iterable: false,
                optional: false,
            },
            labels: labels.to_owned(),
            label_helps: label_helps.to_owned(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_ok() {
        let c =
            ColLabel::parse("", &vec!["hello".to_string()], "", &vec!["".to_string()]).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 0);
    }

    #[test]
    fn label_invalid() {
        let c =
            ColLabel::parse("", &vec!["0hello".to_string()], "", &vec!["".to_string()]).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn label_duplicate() {
        let c = ColLabel::parse(
            "",
            &vec!["hello".to_string(), "HELLO".to_string()],
            "",
            &vec!["".to_string(), "".to_string()],
        )
        .expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn label_empty() {
        let c = ColLabel::parse(
            "",
            &vec!["hello".to_string(), "".to_string()],
            "",
            &vec!["".to_string(), "".to_string()],
        )
        .expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 0);
    }

    #[test]
    fn label_empty_as_label() {
        let c = ColLabel::parse(
            "",
            &vec!["hello".to_string(), "".to_string()],
            "as_label",
            &vec!["".to_string(), "".to_string()],
        )
        .expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn label_missing_help() {
        let c = ColLabel::parse("", &vec!["hello".to_string()], "", &vec![]).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }
}
