use crate::language::Language;
use crate::{aperror, basetype, index, lint, table};

use crate::table::Variant;
use std::cmp::max;
use std::collections::HashMap;

pub struct Dest<'a> {
    /// name of of table for join target
    table: String,
    /// generate accessor for reverse join
    reverse: String,

    max: usize,

    toindex: HashMap<&'a String, usize>,
}
impl<'a> Dest<'a> {
    pub fn new(values: &'a [String], table: String, reverse: String) -> Dest {
        let mut keyindexes = HashMap::<&String, usize>::new();
        for (i, x) in values.iter().enumerate() {
            keyindexes.insert(x, i);
        }
        Dest {
            table,
            reverse,
            max: 0,
            toindex: keyindexes,
        }
    }
}

struct TabRow {
    table: usize,
    row: usize,
}

pub struct ColVariant {
    info: table::ColumnInfo,
    values: Vec<usize>,

    tables: Vec<Variant>,
}

impl table::Column for ColVariant {
    fn info(&self) -> &table::ColumnInfo {
        &self.info
    }

    fn emit_table_cell(&self, row: usize, _lang: &dyn Language) -> String {
        let v = self.values[row];
        v.to_string()
    }

    fn indexes(&self) -> Vec<usize> {
        if self.info.optional {
            index::filter_index(&self.values, 0)
        } else {
            index::index(&self.values)
        }
    }

    fn lint(&self, _linter: &lint::Linter) {}

    fn variants(&self) -> Option<&Vec<Variant>> {
        Some(&self.tables)
    }
}

impl ColVariant {
    pub fn parse(
        name: &str,
        values: &[String],
        optional: bool,
        dests: &mut Vec<Dest>,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let mut tabrow = Vec::<TabRow>::new();

        'row: for (row, value) in values.iter().enumerate() {
            if optional && value.is_empty() {
                tabrow.push(TabRow { table: 0, row: 0 });
                continue;
            }

            for (table, dest) in &mut dests.iter_mut().enumerate() {
                if let Some(row) = dest.toindex.get(value) {
                    dest.max = max(dest.max, row + 1);

                    tabrow.push(TabRow {
                        table: table + optional as usize,
                        row: *row,
                    });
                    continue 'row;
                }
            }

            // print!("*******{:?}{:?}", optional, value);

            return Err(aperror::Error::new(&format!(
                "column {} : dest not found for {}  at row {}",
                name, value, row
            )));
        }

        // compute table offsets
        let (tables, max) = col_offsets(dests, optional);
        let values = row_values(&tabrow, &tables);

        //
        Ok(Box::new(ColVariant {
            info: table::ColumnInfo {
                name: name.to_string(),
                len: values.len(),
                interface_type: basetype::BaseType::Variant,
                table_type: basetype::int_type_for_range(0..=max as i64),
                iterable: dests.iter().any(|c| !c.reverse.is_empty()),
                optional,
            },
            values,
            tables,
        }))
    }
}

// compute offsets
fn col_offsets(dests: &Vec<Dest>, optional: bool) -> (Vec<Variant>, usize) {
    let mut tables = Vec::<Variant>::new();
    let mut index = 0;

    // None variant for optional values
    if optional {
        tables.push(Variant {
            name: "NONE".to_string(),
            index: 0,
            count: 1,
            reverse: "".to_string(),
            is_none: true,
        });
        index = 1;
    }

    for dest in dests {
        tables.push(Variant {
            name: dest.table.to_string(),
            index,
            count: dest.max,
            reverse: dest.reverse.to_string(),
            is_none: false,
        });
        index += dest.max;
    }
    (tables, index)
}

// compute indexes
fn row_values(tabrow: &[TabRow], tables: &[Variant]) -> Vec<usize> {
    tabrow
        .iter()
        .map(|tr| tables[tr.table].index + tr.row)
        .collect()
}
