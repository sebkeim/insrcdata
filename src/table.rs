use std::collections::HashSet;
// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Abstract type for project
//
use crate::basetype::BaseType;
use crate::language::Language;
use crate::{aperror, basetype, language, lint};
use std::path::PathBuf;
use std::str::FromStr;

// ================================================================================================
// Column
// ================================================================================================
pub enum TypeImpl {
    Label,
    Join,
    JoinOptional,
    Scalar,
    Variant,
}

pub struct ColumnInfo {
    pub name: String,
    pub len: usize,
    pub interface_type: basetype::BaseType, // for public API
    pub table_type: basetype::BaseType,     // for data table
    pub iterable: bool,
    pub optional: bool,
}

impl ColumnInfo {
    pub fn type_impl(&self) -> TypeImpl {
        match self.interface_type {
            BaseType::Label { .. } => TypeImpl::Label,
            BaseType::Bool
            | BaseType::F32
            | BaseType::F64
            | BaseType::I8
            | BaseType::I16
            | BaseType::I32
            | BaseType::I64
            | BaseType::U8
            | BaseType::U16
            | BaseType::U32
            | BaseType::U64
            | BaseType::Str
            | BaseType::Object { .. } => TypeImpl::Scalar,
            BaseType::Join { .. } => {
                if self.optional {
                    TypeImpl::JoinOptional
                } else {
                    TypeImpl::Join
                }
            }
            BaseType::Variant => TypeImpl::Variant,
        }
    }

    pub fn join_table(&self) -> String {
        match &self.interface_type {
            basetype::BaseType::Join { strname: name, .. } => name.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn has_iter_range(&self) -> bool {
        match self.interface_type {
            basetype::BaseType::Join { .. } | basetype::BaseType::Variant => false, // implemented target Table by col_reverse_join
            _ => self.iterable,
        }
    }
}

pub struct Variant {
    pub name: String, // dest table name
    pub index: usize,
    pub count: usize,
    pub reverse: String, // getter name for reverse join
    pub is_none: bool,   // null value placeholder for optional vatiants
}

pub trait Column {
    fn info(&self) -> &ColumnInfo;

    // cell value
    fn emit_table_cell(&self, row: usize, lang: &dyn Language) -> String;
    fn emit_label(&self, _row: usize) -> String {
        "EMIT_LABEL_UNSUPORTED".to_string()
    }

    // for indexed lookup
    fn indexes(&self) -> Vec<usize>;

    // check validity of input params
    fn lint(&self, linter: &lint::Linter);

    // used by reverse join
    fn reverse_name(&self) -> String {
        "".to_string()
    }

    // import dependancies for object data type column
    fn fill_import(&self, _out: &mut HashSet<String>) {}

    fn variants(&self) -> Option<&Vec<Variant>> {
        None
    }
}

// ================================================================================================
// Table
// ================================================================================================

pub struct Table {
    pub name: String,
    pub len: usize,
    pub columns: Vec<Box<dyn Column>>,
    pub get_array: bool,

    pub outcol_indexes: Vec<usize>, // value columns
    pub labcol_indexes: Vec<usize>, // label columns
}

impl Table {
    // create Table structure
    pub fn new(name: &str, columns: Vec<Box<dyn Column>>, get_array: bool) -> Table {
        let mut outcol_indexes: Vec<usize> = Vec::new();
        let mut labcol_indexes: Vec<usize> = Vec::new();

        let len = match columns.first() {
            None => 0,
            Some(c) => c.info().len,
        };

        for (index, col) in columns.iter().enumerate() {
            let info = col.info();

            match info.table_type {
                basetype::BaseType::Label { .. } => {
                    if !info.name.is_empty() {
                        // tolabel option is set
                        outcol_indexes.push(index)
                    }
                    labcol_indexes.push(index)
                }
                _ => outcol_indexes.push(index),
            }
        }

        Table {
            name: name.to_owned(),
            len,
            columns,
            get_array,
            outcol_indexes,
            labcol_indexes,
        }
    }

    // check table configuration
    fn lint(&self, linter: &lint::Linter) {
        linter.context(&self.name, |lt_table| {
            lt_table.err(lint::label(&self.name), "invalid table name");

            lt_table.err(
                self.has_data() || !self.get_array,
                "unable to use array on table without values",
            );

            // check columns
            let mut colnames = HashSet::<&String>::new();
            for col in &self.data_columns() {
                let info = col.info();
                lt_table.context(&info.name, |lt_col| {
                    lt_col.err(lint::label(&info.name), "invalid column name");
                    lt_col.err(!colnames.contains(&info.name), "duplicated column name");
                    lt_col.err(self.len == info.len, "mismatched number of rows");
                    col.lint(lt_col)
                });
                colnames.insert(&info.name);
            }
        })
    }

    /// Data type columns
    pub fn data_columns(&self) -> Vec<&dyn Column> {
        self.outcol_indexes
            .iter()
            .map(|i| self.columns[*i].as_ref())
            .collect()
    }

    /// Has at least one data type column
    pub fn has_data(&self) -> bool {
        !self.outcol_indexes.is_empty()
    }

    /// Label type columns
    pub fn label_columns(&self) -> Vec<&dyn Column> {
        self.labcol_indexes
            .iter()
            .map(|i| self.columns[*i].as_ref())
            .collect()
    }

    pub fn index_type(&self) -> basetype::BaseType {
        basetype::int_type_for_range(0..=self.len as i64)
    }

    pub fn imports(&self) -> HashSet<String> {
        let mut imports: HashSet<String> = HashSet::new();
        for col in &self.columns {
            col.fill_import(&mut imports);
        }
        imports
    }
}

// ================================================================================================
// Project
// ================================================================================================

pub struct JoinTo<'a> {
    pub table: &'a Table,
    pub col: &'a dyn Column,
    pub offset: usize,
    pub reverse_name: String,
}

impl<'a> JoinTo<'a> {
    fn from_join(join: &'a Table, col: &'a dyn Column) -> JoinTo<'a> {
        JoinTo {
            table: join,
            col,
            offset: col.info().optional as usize,
            reverse_name: col.reverse_name().to_string(),
        }
    }
    fn from_variant(join: &'a Table, col: &'a dyn Column, vrt: &Variant) -> JoinTo<'a> {
        JoinTo {
            table: join,
            col,
            offset: vrt.index,
            reverse_name: vrt.reverse.to_string(),
        }
    }
}

pub struct Project {
    pub dst_path: PathBuf,
    pub lang: &'static dyn language::Language,
    pub tables: Vec<Table>,
    pub src_modified: std::time::SystemTime,
}

impl Project {
    pub fn lint(&self, linter: &lint::Linter) {
        let projectname = self.name();
        linter.context(&projectname, |lt| {
            lt.err(lint::label(&projectname), "invalid project name");

            let mut tblnames = HashSet::<&String>::new();

            for table in &self.tables {
                lt.err(!tblnames.contains(&table.name), "duplicated table name");
                table.lint(lt);

                lt.err(
                    table.has_data() || self.join_to_columns(table).is_empty(),
                    &format!(
                        "unable to create reverse join to table without values {}",
                        table.name
                    ),
                );
                tblnames.insert(&table.name);
            }
        })
    }

    pub fn modified(&self) -> bool {
        let dst_modified = self
            .lang
            .dst_modified(self)
            .unwrap_or(std::time::UNIX_EPOCH);
        dst_modified < self.src_modified
    }

    pub fn emit(&self) -> aperror::Result<()> {
        self.lang.emit(self)
    }

    pub fn name(&self) -> String {
        match self.dst_path.file_stem() {
            Some(name) => name.to_os_string().into_string().unwrap(),
            None => "unnamed".to_string(),
        }
    }

    // check if iterator datatype must be declared
    pub fn table_need_iter(&self, table: &Table) -> bool {
        for col in &table.columns {
            if col.info().iterable {
                return true;
            }
        }
        !self.join_to_columns(table).is_empty()
    }

    // reverse join to table
    pub fn join_to_columns(&self, table: &Table) -> Vec<JoinTo> {
        let mut columns = Vec::<JoinTo>::new();

        for join in &self.tables {
            if join.name == table.name {
                continue;
            }
            for col in &join.columns {
                let info = col.info();
                match &info.interface_type {
                    basetype::BaseType::Join { strname, .. } => {
                        if strname == &table.name && info.iterable {
                            columns.push(JoinTo::from_join(join, col.as_ref()));
                        }
                    }
                    basetype::BaseType::Variant => {
                        for vrt in col.variants().expect("variant expected") {
                            if vrt.name == table.name && !vrt.reverse.is_empty() {
                                columns.push(JoinTo::from_variant(join, col.as_ref(), vrt));
                            }
                        }
                    }
                    _ => {}
                };
            }
        }
        columns
    }

    pub fn imports(&self) -> HashSet<String> {
        let mut imports: HashSet<String> = HashSet::new();
        for table in &self.tables {
            for col in &table.columns {
                col.fill_import(&mut imports);
            }
        }
        imports
    }
}

// parse a vector of string
pub fn parse_vec<T: FromStr>(strvals: &[String]) -> aperror::Result<Vec<T>> {
    let mut vals: Vec<T> = vec![];

    for (i, s) in strvals.iter().enumerate() {
        let Ok(v) =  s.parse::<T>() else {
            return Err(aperror::Error::new(&format!("{} not a number at row {}", s, i)));
        };
        vals.push(v);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colstr::ColStr;
    use crate::langrust;
    use crate::lint::test_linter;
    use std::time::SystemTime;

    #[test]
    fn duplicate_table_name() {
        let t1 = Table::new("mytable", vec![], false);
        let t2 = Table::new("mytable", vec![], false);

        let project = Project {
            dst_path: PathBuf::new(),
            lang: langrust::RUST,
            tables: vec![t1, t2],
            src_modified: SystemTime::now(),
        };

        let linter = test_linter();
        project.lint(&linter);
        assert!(linter.errors() == 1);
    }

    #[test]
    fn duplicate_col_name() {
        let a1 = ColStr::parse("mycol", &vec![], false).unwrap();
        let a2 = ColStr::parse("mycol", &vec![], false).unwrap();

        let t = Table::new("table", vec![a1, a2], false);

        let linter = test_linter();
        t.lint(&linter);
        assert!(linter.errors() == 1);
    }
}
