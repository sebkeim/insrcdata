// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// integer data type column
//

use crate::language::Language;
use crate::{aperror, table};
use crate::{basetype, lint};

pub struct ColInt {
    info: table::ColumnInfo,
    values: Vec<i64>,
    min: i64, // minimal value
    max: i64, // maximal value
}

impl table::Column for ColInt {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        let v = self.values[row];
        v.to_string()
    }

    fn indexes(&self) -> Vec<usize> {
        let values = &self.values;
        let mut indexes = Vec::from_iter(0..values.len());

        indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
        indexes
    }

    fn lint(&self, linter: &lint::Linter) {
        linter.err(
            self.max <= self.info.interface_type.max() as i64,
            &format!(
                "value overflow {} for interface type {}",
                self.max, self.info.interface_type
            ),
        );
        linter.err(
            self.min >= self.info.interface_type.min() as i64,
            &format!(
                "value undeflow {} for interface type {}",
                self.min, self.info.interface_type
            ),
        );
    }
}

impl ColInt {
    pub fn parse(
        name: &str,
        strvals: &[String],
        iterable: bool,
        interface_type: basetype::BaseType,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let values = table::parse_vec::<i64>(strvals)?;

        let min = *values.iter().min().unwrap_or(&0); // minimal value
        let max = *values.iter().max().unwrap_or(&0); // maximal value

        Ok(Box::new(ColInt {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type,
                table_type: basetype::int_type_for_range(min..=max),
                iterable,
                optional: false,
            },
            values,
            min, // minimal value
            max, // maximal value
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_no_oveflow() {
        let c =
            ColInt::parse("", &vec!["123".to_string()], false, basetype::BaseType::U8).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 0);
    }

    #[test]
    fn u8_oveflow() {
        let c =
            ColInt::parse("", &vec!["300".to_string()], false, basetype::BaseType::U8).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn u8_underflow() {
        let c =
            ColInt::parse("", &vec!["-1".to_string()], false, basetype::BaseType::U8).expect("");
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }
}
