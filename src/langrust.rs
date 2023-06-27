// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// target implementation for Rust language
//

use crate::{aperror, basetype, language, log, table};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::{fs, io};

struct Rust {}

// rust data types
fn strtype(typ: &basetype::BaseType) -> String {
    String::from(match typ {
        basetype::BaseType::Label => "",
        basetype::BaseType::I8 => "i8",
        basetype::BaseType::I16 => "i16",
        basetype::BaseType::I32 => "i32",
        basetype::BaseType::I64 => "i64",
        basetype::BaseType::U8 => "u8",
        basetype::BaseType::U16 => "u16",
        basetype::BaseType::U32 => "u32",
        basetype::BaseType::U64 => "u64",
        basetype::BaseType::Str => "&'static str",
        basetype::BaseType::Join {
            strname: _,
            mincard: _,
            maxcard: _,
        } => "TODO",
    })
}
fn argtype(typ: &basetype::BaseType) -> String {
    match typ {
        basetype::BaseType::Str => "& str".to_string(),
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

fn write_iter_index_struct(strname: &String, output: &mut dyn io::Write) -> io::Result<()> {
    writeln!(output, "
use std::mem;

pub struct IndexIter {{
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
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
fn use_pub(table: &table::Table) -> &str {
    if table.itrait.is_empty() {
        "pub "
    } else {
        ""
    }
}
fn cast_to_interface_type(info: &table::ColumnInfo) -> String {
    if info.interface_type == info.table_type {
        String::from("")
    } else {
        format!(" as {}", strtype(&info.interface_type))
    }
}

fn trait_getter_col(col: &dyn table::Column, output: &mut dyn io::Write) -> io::Result<()> {
    let info = col.info();
    let field = &info.name;

    match &info.interface_type.type_impl() {
        basetype::TypeImpl::Label => {
            log::verbose("unexpected getter_col for Label type");
        }
        basetype::TypeImpl::Join01 => {
            let outtype = struct_name(&info.join_table());
            writeln!(
                output,
                "    fn {field}(&self) -> Option<&'static {outtype}>;"
            )?;
        }
        basetype::TypeImpl::Join11 => {
            let outtype = struct_name(&info.join_table());
            writeln!(output, "    fn {field}(&self) -> &'static {outtype};",)?;
        }

        basetype::TypeImpl::Scalar => {
            let outtype = strtype(&info.interface_type);
            writeln!(output, "    fn {field}(&self) -> {outtype} ;",)?;
        }
    }
    Ok(())
}

fn impl_getter_col(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let field = &info.name;
    let usepub = use_pub(table);
    match &info.interface_type.type_impl() {
        basetype::TypeImpl::Label => {
            log::verbose("unexpected getter_col for Label type");
        }
        basetype::TypeImpl::Join01 => {
            let outtype = struct_name(&info.join_table());
            let jointable = table_name(&outtype);
            writeln!(
                output,
                "    {usepub}fn {field}(&self) -> Option<&'static {outtype}> {{
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
                "    {usepub}fn {field}(&self) -> &'static {outtype} {{
        &{jointable}[self.{field}_ as usize]
    }}"
            )?;
        }

        basetype::TypeImpl::Scalar => {
            let outtype = strtype(&info.interface_type);
            let cast = cast_to_interface_type(info);
            writeln!(
                output,
                "    {usepub}fn {field}(&self) -> {outtype} {{ self.{field}_{cast} }}",
            )?;
        }
    }
    Ok(())
}

// ================================================================================================
// Range iterator
// ================================================================================================

fn trait_iter_col(
    project: &table::Project,
    strname: &str,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let field = &info.name;
    let argtype = strtype(&info.interface_type);
    let projectname = project.name();
    let srcstruct = struct_name(strname);

    writeln!(
        output,
        "    fn {field}_range(start:{argtype}, stop:{argtype}) -> {projectname}::{srcstruct}Iter "
    )
}

fn impl_iter_col(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();

    let usepub = use_pub(table);
    let field = &info.name;
    let argtype = argtype(&info.interface_type);
    let modname = mod_name(&table.name);
    let tablename = table_name(&table.name);
    let indexname = index_name(&table.name, &info.name);
    let cast = cast_to_interface_type(info);

    writeln!(
        output,
        "
    {usepub}fn {field}_range(start:{argtype}, stop:{argtype}) -> {modname}::IndexIter {{
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
fn trait_reverse_join(
    project: &table::Project,
    srcname: &str,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let reverse = col.reverse_name();
    let projectname = project.name();
    let srcstruct = struct_name(srcname);

    writeln!(
        output,
        "    fn {reverse}(&self) -> {projectname}::{srcstruct}Iter;"
    )
}

fn impl_reverse_join(
    table: &table::Table,
    srccol: &dyn table::Column,
    srcname: &str,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    if !table.has_data() {
        log::warning(&format!("{} will crash if used", srccol.reverse_name()));
    }

    let info = srccol.info();
    let usepub = use_pub(table);
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
    {usepub}fn {reverse}(&self) -> {srcstruct}Iter {{
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
fn impl_col_labels(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let enumname = struct_name(&info.name);
    write!(output, "#[derive(Clone, Copy)]\npub enum {enumname} {{\n")?;

    for row in 0..info.len {
        let label = col.emit_table_cell(row);
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
}}"
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
fn trait_table(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);

    writeln!(output, "trait {} {{", table.itrait)?;

    // data columns
    let datacols: Vec<&dyn table::Column> = table.data_columns();
    for col in datacols {
        trait_getter_col(col, output)?;

        if col.info().has_iter_range() {
            trait_iter_col(project, &strname, col, output)?;
        }
    }

    // reverse table join
    let reverse_join = project.join_to_columns(table);
    for (join, col) in reverse_join {
        trait_reverse_join(project, &join.name, col, output)?;
    }

    writeln!(output, "}}")
}

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
        let typ = strtype(&info.table_type);
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

fn impl_table_data(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);

    // define structure
    let datacols: Vec<&dyn table::Column> = table.data_columns();

    writeln!(output, "pub struct {} {{", strname)?;
    for col in &datacols {
        let info = col.info();
        let fieldtype = strtype(&info.table_type);
        writeln!(output, "    {}_ : {},", info.name, fieldtype)?;
    }
    writeln!(output, "}}")?;

    // stucture implementation
    let use_trait = !table.itrait.is_empty();
    if use_trait {
        writeln!(output, "impl {} for {} {{", table.itrait, strname)?;
    } else {
        writeln!(output, "impl {} {{", strname)?;
    }

    // data column
    for col in &datacols {
        impl_getter_col(table, *col, output)?;
        if col.info().has_iter_range() {
            impl_iter_col(table, *col, output)?;
        }
    }

    let reverse_join = project.join_to_columns(table);
    for (join, col) in reverse_join {
        impl_reverse_join(table, col, &join.name, output)?;
    }

    if table.get_array {
        if use_trait {
            writeln!(output, "}}\n impl {}  {{", strname)?;
        }
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
    write!(output, "mod {} {{\n\n", modname)?;

    if project.table_need_iter(table) {
        write_iter_index_struct(&strname, output)?;
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

fn impl_table(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    // Labels
    let labelcols: Vec<&dyn table::Column> = table.label_columns();
    for col in labelcols {
        impl_col_labels(table, col, output)?;
    }
    if table.has_data() {
        impl_table_data(project, table, output)?;
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

#![allow(dead_code)]\n#![allow(unused_variables)]" // TODO : remove this
        )?;

        if project.has_deref_labels() {
            writeln!(output, "use std::ops::Deref;")?;
        }

        for table in &project.tables {
            impl_table(project, table, output)?;
        }
        Ok(())
    }

    fn interface(&self, project: &table::Project) -> aperror::Result<()> {
        let output = &mut io::stdout() as &mut dyn io::Write;

        for table in &project.tables {
            if !table.itrait.is_empty() {
                trait_table(project, table, output)?;
            }
        }
        Ok(())
    }

    fn extension(&self) -> String {
        "rs".to_string()
    }
}

const RUST_: Rust = Rust {};
pub const RUST: &'static dyn language::Language = &RUST_;
