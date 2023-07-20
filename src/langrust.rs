// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// target implementation for Rust language
//

use crate::basetype::BaseType;
use crate::{aperror, basetype, language, log, table};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::{fs, io};

struct Rust {}

// rust data types
fn strtype(typ: &basetype::BaseType) -> String {
    String::from(match typ {
        BaseType::Label { name } => return struct_name(name),
        BaseType::I8 => "i8",
        BaseType::I16 => "i16",
        BaseType::I32 => "i32",
        BaseType::I64 => "i64",
        BaseType::U8 => "u8",
        BaseType::U16 => "u16",
        BaseType::U32 => "u32",
        BaseType::U64 => "u64",
        BaseType::Str => "&'static str",
        BaseType::Join {
            strname: _,
            mincard: _,
            maxcard: _,
        } => "TODO",
        BaseType::Object { objtype } => objtype,
        BaseType::Bool => "bool",
        BaseType::F32 => "f32",
        BaseType::F64 => "f64",
    })
}
fn argtype(typ: &basetype::BaseType) -> String {
    match typ {
        basetype::BaseType::Str => "& str".to_string(),
        _ => strtype(typ),
    }
}
fn modtype(typ: &basetype::BaseType) -> String {
    match typ {
        basetype::BaseType::Label { name } => format!("super::{}", struct_name(name)),
        _ => strtype(typ),
    }
}
// ================================================================================================
// format name to rust conventions
// ================================================================================================
fn struct_name(table_name: &str) -> String {
    table_name.to_upper_camel_case()
}
fn const_name(table_name: &str) -> String {
    table_name.to_shouty_snake_case()
}
fn mod_name(table_name: &str) -> String {
    table_name.to_snake_case()
}
fn table_name(strname: &str) -> String {
    format!("{}::TABLE", mod_name(strname))
}
fn index_name(strname: &str, field: &str) -> String {
    format!("{}::{}_INDEX", mod_name(strname), const_name(field))
}

// ================================================================================================
// Struct definition
// ================================================================================================

fn write_iter_index_struct(
    table: &table::Table,
    strname: &String,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let indextype = strtype(&table.index_type());

    writeln!(output, "
use std::mem;

pub struct IndexIter {{
    pub indexes : Box<dyn Iterator<Item=&'static {indextype}>>,
}}

impl Iterator for IndexIter {{
    type Item = & 'static super::{strname};

    fn next(&mut self) -> Option<&'static super::{strname}> {{
        let idx = self.indexes.next();
        match idx {{
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }}
    }}
}}

pub fn index_of(fic:&super::{strname}) -> usize {{
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / mem::size_of::<super::{strname}>()
}}
")
}

// ================================================================================================
// Getters
// ================================================================================================
fn cast_to_interface_type(info: &table::ColumnInfo) -> String {
    if info.interface_type == info.table_type {
        String::from("")
    } else {
        format!(" as {}", strtype(&info.interface_type))
    }
}

fn getter_col(col: &dyn table::Column, output: &mut dyn io::Write) -> io::Result<()> {
    let info = col.info();
    let field = &info.name;
    match &info.interface_type.type_impl() {
        basetype::TypeImpl::Label => {
            log::verbose("unexpected getter_col for Label type");
        }
        basetype::TypeImpl::Join01 => {
            let outtype = struct_name(&info.join_table());
            let jointable = table_name(&outtype);
            writeln!(
                output,
                "    pub fn {field}(&self) -> Option<&'static {outtype}> {{
        let index = self.{field}_;
        if index==0 {{ None }} else {{ Some(&{jointable}[index as usize -1]) }}
    }}"
            )?;
        }
        basetype::TypeImpl::Join11 => {
            let outtype = struct_name(&info.join_table());
            let jointable = table_name(&outtype);
            writeln!(
                output,
                "    pub fn {field}(&self) -> &'static {outtype} {{
        &{jointable}[self.{field}_ as usize]
    }}"
            )?;
        }

        basetype::TypeImpl::Scalar => {
            let outtype = strtype(&info.interface_type);
            let cast = cast_to_interface_type(info);
            writeln!(
                output,
                "    pub fn {field}(&self) -> {outtype} {{ self.{field}_{cast} }}",
            )?;
        }
    }
    Ok(())
}

// ================================================================================================
// Range iterator
// ================================================================================================
fn iter_col(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();

    let field = &info.name;
    let argtype = argtype(&info.interface_type);
    let modname = mod_name(&table.name);
    let tablename = table_name(&table.name);
    let indexname = index_name(&table.name, &info.name);
    let cast = cast_to_interface_type(info);

    writeln!(
        output,
        "
    pub fn {field}_range(start:{argtype}, stop:{argtype}) -> {modname}::IndexIter {{
        let mut lo = 0;
        let mut hi = {indexname}.len();
        while lo < hi {{
            let mid = (lo + hi) / 2;
            if start > {tablename}[{indexname}[mid] as usize].{field}_{cast} {{
                 lo = mid + 1;
            }} else {{
                 hi = mid;
            }}
        }}

        let begin = lo;
        hi = {indexname}.len();
        while lo < hi {{
            let mid = (lo + hi) / 2;
            if stop < {tablename}[{indexname}[mid] as usize].{field}_{cast} {{
                hi = mid;
            }} else {{
                lo = mid + 1;
            }}
        }}
        {modname}::IndexIter {{
            indexes: Box::new({indexname}[begin..lo].iter()),
        }}
    }}"
    )
}

// ================================================================================================
// Reverse join
// ================================================================================================
fn reverse_join(
    table: &table::Table,
    srccol: &dyn table::Column,
    srcname: &str,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    if !table.has_data() {
        log::warning(&format!("{} will crash if used", srccol.reverse_name()));
    }

    let info = srccol.info();
    let field = &info.name;
    let reverse = srccol.reverse_name();
    let srcmod = mod_name(srcname);
    let srcstruct = struct_name(srcname);
    let joinmod = mod_name(&table.name);
    let tabletype = strtype(&info.table_type);
    let offset = if info.mincard0() { " + 1" } else { "" };
    let indexname = index_name(srcname, &info.name);

    writeln!(
        output,
        "
    pub fn {reverse}(&self) -> {srcstruct}Iter {{
        let cons = {joinmod}::index_of(self) as {tabletype}{offset};

        // bissect left
        let mut lo = 0;
        let mut hi = {indexname}.len();
        while lo < hi {{
            let mid = (lo + hi) / 2;
            if cons > {srcmod}::TABLE[{indexname}[mid] as usize].{field}_ {{
                lo = mid + 1;
            }} else {{
                hi = mid;
            }}
        }}
        let start = lo;

        // bissect-right
        hi = {indexname}.len();
        while lo < hi {{
            let mid = (lo + hi) / 2;
            if cons < {srcmod}::TABLE[{indexname}[mid] as usize].{field}_  {{
                hi = mid;
            }} else {{
                lo = mid + 1;
            }}
        }}

        {srcmod}::IndexIter {{
            indexes: Box::new({indexname}[start..lo].iter()),
        }}
    }}"
    )
}
// ================================================================================================
// Labels
// ================================================================================================
fn col_labels(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let enumname = strtype(&info.interface_type);
    write!(
        output,
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub enum {enumname} {{\n"
    )?;

    for row in 0..info.len {
        let label = col.emit_label(row);
        if !label.is_empty() {
            let camel = label.to_upper_camel_case();
            writeln!(output, "    {label} = {row},", label = camel, row = row)?;
        }
    }
    writeln!(output, "}}")?;

    let strname = struct_name(&table.name);
    let modname = mod_name(&table.name);
    if table.has_data() {
        writeln!(
            output,
            "impl Deref for {enumname} {{
    type Target =  {strname};
    fn deref(&self) -> &'static {strname} {{
        &{modname}::TABLE[*self as usize]
    }}
}}
impl PartialEq<&{strname}> for {enumname} {{
    fn eq(&self, other: &&{strname}) -> bool {{
        std::ptr::eq(self as &{strname}, *other)
    }}
}}
"
        )?;
    }
    Ok(())
}

// ================================================================================================
// Indexes
// ================================================================================================
fn write_index(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let indexes = col.indexes();
    let uperfield = const_name(&col.info().name);
    let indextype = strtype(&table.index_type());
    let len = indexes.len();

    write!(
        output,
        "pub static {uperfield}_INDEX : [ {indextype} ; {len} ] = ["
    )?;

    let width = language::digits(indexes.len());
    for (i, v) in indexes.iter().enumerate() {
        if i % 20 == 0 {
            write!(output, "\n    ")?;
        }
        write!(output, "{:width$}, ", v)?;
    }
    write!(output, "\n];\n")
}

// ================================================================================================
// Table
// ================================================================================================
// define ctor fuction  : const fn r(hello:u8, ) -> Table1 { return Table1{hello_:hello, }; }
fn write_ctor_function(
    strname: &String,
    outcols: &Vec<&dyn table::Column>,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    // interface
    write!(output, "const fn r(")?;
    for col in outcols {
        let info = col.info();
        let typ = modtype(&info.table_type);
        write!(output, "{}:{}, ", info.name, typ)?;
    }
    write!(output, ") -> super::{} ", strname)?;

    // body
    write!(output, "{{\n    super::{}{{", strname)?;
    for col in outcols {
        let info = col.info();
        write!(output, "{}_:{}, ", info.name, info.name)?;
    }
    write!(output, "}}\n}}\n\n")
}

fn table_data(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);

    // define structure
    let datacols: Vec<&dyn table::Column> = table.data_columns();

    writeln!(output, "pub struct {strname} {{")?;
    for col in &datacols {
        let info = col.info();
        let fieldtype = strtype(&info.table_type);
        writeln!(output, "    {}_ : {},", info.name, fieldtype)?;
    }
    writeln!(
        output,
        "}}
impl PartialEq<Self> for {strname} {{
    fn eq(&self, other: &Self) -> bool {{
        std::ptr::eq(self, other)
    }}
}}"
    )?;

    // stucture implementation
    writeln!(output, "impl {} {{", strname)?;

    // data column
    for col in &datacols {
        getter_col(*col, output)?;
        if col.info().has_iter_range() {
            iter_col(table, *col, output)?;
        }
    }

    let joins_to = project.join_to_columns(table);
    for (join, col) in joins_to {
        reverse_join(table, col, &join.name, output)?;
    }

    if table.get_array {
        let srcstruct = struct_name(&table.name);
        let tablelen = table.len;
        let srcmod = mod_name(&table.name);
        writeln!(
            output,
            "    pub fn array() -> &'static [{srcstruct}; {tablelen}]  {{ &{srcmod}::TABLE }}",
        )?;
    }
    write!(output, "}}\n\n")?;

    // begin module private
    let modname = mod_name(&strname);
    writeln!(output, "mod {} {{", modname)?;

    for import in table.imports() {
        writeln!(output, "use {import};")?;
    }
    writeln!(output)?;

    if project.table_need_iter(table) {
        write_iter_index_struct(table, &strname, output)?;
    }
    write_ctor_function(&strname, &datacols, output)?;

    // table data
    writeln!(
        output,
        "pub static TABLE : [ super::{} ; {} ] = [",
        strname, table.len
    )?;
    for row in 0..table.len {
        write!(output, "   {{r(")?;
        for col in &datacols {
            write!(output, "{}, ", col.emit_table_cell(row))?;
        }
        writeln!(output, ")}},")?;
    }
    writeln!(output, "];")?;

    // indexes
    for col in datacols {
        if col.info().iterable {
            write_index(table, col, output)?;
        }
    }
    writeln!(output, "\n}} // mod {}\n", modname)?;

    //export
    if project.table_need_iter(table) {
        writeln!(output, "pub use {modname}::IndexIter as {strname}Iter;",)?;
    }

    Ok(())
}

fn emit_table(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    // Labels
    let labelcols: Vec<&dyn table::Column> = table.label_columns();
    for col in labelcols {
        col_labels(table, col, output)?;
    }
    if table.has_data() {
        table_data(project, table, output)?;
    }
    Ok(())
}

// ================================================================================================
// Entry point
// ================================================================================================
impl language::Language for Rust {
    fn emit(&self, project: &table::Project) -> aperror::Result<()> {
        let mut outfile =
            aperror::io_error_result(fs::File::create(&project.dst_path), &project.dst_path)?;
        let output = (&mut outfile) as &mut dyn io::Write;
        let notice = language::file_notice();
        writeln!(
            output,
            "// {notice}

#![allow(dead_code)]
#![allow(unused_variables)]"
        )?;
        // TODO : remove allow(dead_code)

        if project.has_deref_labels() {
            writeln!(output, "use std::ops::Deref;")?;
        }

        for table in &project.tables {
            emit_table(project, table, output)?;
        }
        Ok(())
    }

    fn extension(&self) -> String {
        "rs".to_string()
    }
}

const RUST_: Rust = Rust {};
pub const RUST: &'static dyn language::Language = &RUST_;
