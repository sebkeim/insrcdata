// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// integer data type column
//

use crate::table;
use crate::{basetype, lint};
use std::cmp;

pub struct ColInt {
    info: table::ColumnInfo,
    values: Vec<i64>,

    // stats
    min: i64, // minimal value
    max: i64, // maximal value
}

impl table::Column for ColInt {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize) -> String {
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
    pub fn new(
        interface_type: basetype::BaseType,
        name: &str,
        values: Vec<i64>,
        iterable: bool,
    ) -> ColInt {
        let mut min = values[0]; // minimal value
        let mut max = values[0]; // maximal value

        for v in &values {
            min = cmp::min(min, *v);
            max = cmp::max(max, *v);
        }

        ColInt {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type,
                table_type: basetype::int_type_for_range(min, max),
                iterable,
            },
            values,

            min, // minimal value
            max, // maximal value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::Column;

    #[test]
    fn u8_no_oveflow() {
        let c = ColInt::new(basetype::BaseType::U8, "", vec![123], false);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 0);
    }

    #[test]
    fn u8_oveflow() {
        let c = ColInt::new(basetype::BaseType::U8, "", vec![300], false);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn u8_underflow() {
        let c = ColInt::new(basetype::BaseType::U8, "", vec![-1], false);
        let linter = lint::test_linter();
        c.lint(&linter);
        assert!(linter.errors() == 1);
    }
}