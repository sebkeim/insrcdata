// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// object data type column
//

use crate::language::Language;
use crate::table;
use crate::table::ColumnConfig;
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

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
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
            "template must contain exactly one '{}' placeholder",
        );
    }

    fn fill_import(&self, out: &mut HashSet<String>) {
        if !self.import.is_empty() {
            out.insert(self.import.to_string());
        }
    }
}

impl ColObject {
    pub fn new(
        config: ColumnConfig,
        values: Vec<String>,
        objtype: &str,
        template: &str,
        import: &str,
    ) -> ColObject {
        ColObject {
            info: table::ColumnInfo {
                config,
                len: values.len(),
                interface_type: basetype::BaseType::Object {
                    objtype: objtype.to_string(),
                },
                table_type: basetype::BaseType::Object {
                    objtype: objtype.to_string(),
                },
            },
            values,
            template: template.to_string(),
            import: import.to_string(),
        }
    }
}
