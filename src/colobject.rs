// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// object data type column
//

use crate::table;
use crate::{basetype, lint};
use std::collections::HashSet;

pub struct ColObject {
    info: table::ColumnInfo,
    values: Vec<String>,
    template: String,
    import: String,
}

impl table::Column for ColObject {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize) -> String {
        // TODO : stability of String.Debug trait implementation is not guaranteed
        let v = &self.values[row];
        self.template.replace("{}", v)
    }

    fn indexes(&self) -> Vec<usize> {
        vec![]
    }

    fn lint(&self, linter: &lint::Linter) {
        linter.err(
            self.template.matches("{}").count() == 1,
            "template must ontain exactly one '{}' placeholder",
        );
    }

    fn fill_import(&self, out: &mut HashSet<String>) {
        if self.import.len() > 0 {
            out.insert(self.import.to_string());
        }
    }
}

impl ColObject {
    pub fn new(
        name: &str,
        values: Vec<String>,
        objtype: &str,
        template: &str,
        import: &str,
    ) -> ColObject {
        ColObject {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::Object {
                    objtype: objtype.to_string(),
                },
                table_type: basetype::BaseType::Object {
                    objtype: objtype.to_string(),
                },
                iterable: false,
            },
            values: values,
            template: template.to_string(),
            import: import.to_string(),
        }
    }
}
