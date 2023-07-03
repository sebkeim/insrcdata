// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// target implementation for C language
//

use crate::{aperror, basetype, language, log, table};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use std::path::PathBuf;
use std::{fs, io};

struct LangC {}

fn enum_name(col: &dyn table::Column) -> String {
    match &col.info().interface_type {
        basetype::BaseType::Label { name } => name.to_snake_case(),
        _ => "NOT A LABEL".to_string(),
    }
}
// c data types
fn strtype(typ: &basetype::BaseType) -> String {
    String::from(match typ {
        basetype::BaseType::Label { .. } => "",
        basetype::BaseType::I8 => "int8_t",
        basetype::BaseType::I16 => "int16_t",
        basetype::BaseType::I32 => "int32_t",
        basetype::BaseType::I64 => "int64_t",
        basetype::BaseType::U8 => "uint8_t",
        basetype::BaseType::U16 => "uint16_t",
        basetype::BaseType::U32 => "uint32_t",
        basetype::BaseType::U64 => "uint64_t",
        basetype::BaseType::Str => "const char*", //todo check encoding
        basetype::BaseType::Join {
            strname: _,
            mincard: _,
            maxcard: _,
        } => "TODO",
        basetype::BaseType::Object { objtype } => objtype,
    })
}

fn gt(typ: &basetype::BaseType, left: &str, right: &str) -> String {
    match typ {
        basetype::BaseType::Label { .. }
        | basetype::BaseType::Join { .. }
        | basetype::BaseType::Object { .. } => "todo".to_string(),
        basetype::BaseType::I8
        | basetype::BaseType::I16
        | basetype::BaseType::I32
        | basetype::BaseType::I64
        | basetype::BaseType::U8
        | basetype::BaseType::U16
        | basetype::BaseType::U32
        | basetype::BaseType::U64 => format!("{left}>{right}"),
        basetype::BaseType::Str => format!("strcmp({left},{right})>0"),
    }
}

fn lt(typ: &basetype::BaseType, left: &str, right: &str) -> String {
    match typ {
        basetype::BaseType::Label { .. }
        | basetype::BaseType::Join { .. }
        | basetype::BaseType::Object { .. } => "todo".to_string(),
        basetype::BaseType::I8
        | basetype::BaseType::I16
        | basetype::BaseType::I32
        | basetype::BaseType::I64
        | basetype::BaseType::U8
        | basetype::BaseType::U16
        | basetype::BaseType::U32
        | basetype::BaseType::U64 => format!("{left}<{right}"),
        basetype::BaseType::Str => format!("strcmp({left},{right})<0"),
    }
}

// ================================================================================================
// format name to C conventions
// ================================================================================================
fn struct_name(table_name: &str) -> String {
    table_name.to_snake_case()
}
fn table_name(table_name: &str) -> String {
    table_name.to_shouty_snake_case()
}

// ================================================================================================
// Getters
// ================================================================================================
fn header_getter_col(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let strname = struct_name(&table.name);
    let field = &info.name;

    match &info.interface_type.type_impl() {
        basetype::TypeImpl::Label => {
            log::verbose("unexpected getter_col for Label type");
        }
        basetype::TypeImpl::Join01 => {
            let outtype = struct_name(&info.join_table());
            writeln!(
                output,
                "extern bool {strname}_{field}(const {strname}_t* s, const {outtype}_t** ptr);",
            )?;
        }
        basetype::TypeImpl::Join11 => {
            let outtype = struct_name(&info.join_table());
            writeln!(
                output,
                "extern const {outtype}_t* {strname}_{field}(const {strname}_t* s);",
            )?;
        }

        basetype::TypeImpl::Scalar => {
            let outtype = strtype(&info.interface_type);
            writeln!(
                output,
                "static inline {outtype} {strname}_{field}(const {strname}_t* s) {{ return s->{field}_; }}",
            )?;
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
    let strname = struct_name(&table.name);
    let field = &info.name;

    match &info.interface_type.type_impl() {
        basetype::TypeImpl::Label => {}

        basetype::TypeImpl::Join01 => {
            let outtype = struct_name(&info.join_table());
            let jointable = table_name(&outtype);
            writeln!(
                output,
                "bool {strname}_{field}(const {strname}_t* s, const {outtype}_t** ptr) {{
    if( s->{field}_) {{
        *ptr = &{jointable}_TABLE[s->{field}_-1];
        return true;
    }}
    return false;
}}",
            )?;
        }

        basetype::TypeImpl::Join11 => {
            let outtype = struct_name(&info.join_table());
            let jointable = table_name(&outtype);
            writeln!(
output,
"const {outtype}_t* {strname}_{field}(const {strname}_t* s) {{ return &{jointable}_TABLE[s->{field}_];}}",
)?;
        }

        basetype::TypeImpl::Scalar => {}
    }
    Ok(())
}

// ================================================================================================
// Range iterator
// ================================================================================================
fn header_iter_range(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let colname = &col.info().name;
    let argtype = strtype(&col.info().table_type);
    writeln!(
        output,
        "extern {strname}_iter_t  {strname}_{colname}_range( {argtype} start, {argtype} stop);"
    )
}

fn impl_iter_range(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let indextyp = strtype(&table.index_type());

    let strname = struct_name(&table.name);
    let colname = struct_name(&info.name);
    let argtype = strtype(&info.interface_type);
    let strtable = table_name(&table.name);
    let field = table_name(&info.name);

    let right = format!("{strtable}_TABLE[*mid].{colname}_ ");
    let gt = gt(&col.info().table_type, "start", &right);
    let lt = lt(&col.info().table_type, "stop", &right);

    write!(
        output,
        "{strname}_iter_t  {strname}_{colname}_range( {argtype} start, {argtype} stop) {{
    {indextyp}* lo = {strtable}_{field}_INDEX;
    {indextyp}*  hi = {strtable}_{field}_INDEX + {strtable}_{field}_INDEX_COUNT;
    while( lo < hi ){{
        {indextyp}*  mid = lo + ( hi-lo)/2;
        if( {gt} ){{
             lo = mid + 1;
        }} else {{
             hi = mid;
        }}
    }}

    {indextyp}*  begin = lo;
    hi = {strtable}_{field}_INDEX + {strtable}_{field}_INDEX_COUNT;
    while( lo < hi ){{
         {indextyp}* mid = lo + ( hi-lo)/2;
        if( {lt} ){{
            hi = mid;
        }} else {{
            lo = mid + 1;
        }}
    }}

    {strname}_iter_t res = {{  begin,  lo }};
    return res;
}}
"
    )
}

// ================================================================================================
// Reverse join
// ================================================================================================

fn header_reverse_join(
    table: &table::Table,
    srccol: &dyn table::Column,
    srcname: &str,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let reverse = srccol.reverse_name();
    let strsrc = struct_name(srcname);
    writeln!(
        output,
        "extern {strsrc}_iter_t {strname}_{reverse}(const {strname}_t* s);"
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

    let strname = struct_name(&table.name);
    let indextyp = strtype(&table.index_type());

    let info = srccol.info();
    let reverse = srccol.reverse_name();
    let strsrc = struct_name(srcname);
    let tablesrc = table_name(srcname);
    let strtable = table_name(&table.name);

    let field = table_name(&info.name);
    let colname = &info.name;

    let offset = if info.mincard0() { " + 1" } else { "" };
    writeln!(
        output,
        "{strsrc}_iter_t {strname}_{reverse}(const {strname}_t* s) {{
    long cons = s - {strtable}_TABLE{offset};

    // bissect left
    {indextyp}* lo = {tablesrc}_{field}_INDEX;
    {indextyp}* hi = {tablesrc}_{field}_INDEX + {tablesrc}_{field}_INDEX_COUNT;
   
    while( lo < hi ){{
        {indextyp}*  mid =  lo + ( hi-lo)/2;
        if ( cons > {tablesrc}_TABLE[*mid].{colname}_ ) {{
             lo = mid + 1;
        }} else {{
             hi = mid;
        }}
    }}
    {indextyp}* begin = lo;

    // bissect-right
    hi = {tablesrc}_{field}_INDEX +  {tablesrc}_{field}_INDEX_COUNT;
    while( lo < hi ){{
        {indextyp}*  mid =  lo + ( hi-lo)/2;
        if( cons < {tablesrc}_TABLE[*mid].{colname}_ )  {{
            hi = mid;
        }} else {{
            lo = mid + 1;
        }}
     }}

    {strsrc}_iter_t res = {{  begin,  lo }};
    return res;
}}\n"
    )
}

// ================================================================================================
// Labels
// ================================================================================================

fn header_col_labels(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();

    writeln!(output, "typedef enum {{")?;

    let enumname = enum_name(col);
    let prefix = enumname.to_shouty_snake_case();
    for row in 0..info.len {
        let label = col.emit_label(row);
        if !label.is_empty() {
            let camel = label.to_shouty_snake_case();
            writeln!(
                output,
                "    {prefix}_{label} = {row},",
                prefix = prefix,
                label = camel,
                row = row
            )?;
        }
    }
    writeln!(output, "}} {enumname}_t;")?;

    if table.has_data() {
        let strname = struct_name(&table.name);
        writeln!(
            output,
            "const {strname}_t* {strname}_from_{enumname}({enumname}_t label);
{enumname}_t {strname}_{enumname}(const {strname}_t *s);
            "
        )?;
    }
    Ok(())
}

fn impl_col_labels(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let tablename = table_name(&table.name);
    let enumname = enum_name(col);

    if table.has_data() {
        writeln!(
            output,
            "const {strname}_t* {strname}_from_{enumname}({enumname}_t label) {{
    return &{tablename}_TABLE[label];
}}
{enumname}_t {strname}_{enumname}(const {strname}_t *s) {{
    return ({enumname}_t)(s-{tablename}_TABLE);
}}
"
        )?;
    }
    Ok(())
}

// ================================================================================================
// Indexes
// ================================================================================================

fn header_index(table: &table::Table, output: &mut dyn io::Write) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let indextyp = strtype(&table.index_type());

    writeln!(
        output,
        "typedef struct {{ {indextyp}* ptr; {indextyp}* end; }} {strname}_iter_t;
extern const {strname}_t* {strname}_next({strname}_iter_t* idx);"
    )
}

fn impl_index(table: &table::Table, output: &mut dyn io::Write) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let tablename = table_name(&table.name);

    writeln!(output, "const {strname}_t* {strname}_next({strname}_iter_t* idx) {{ return idx->ptr<idx->end ? &{tablename}_TABLE[*idx->ptr++] : NULL; }}
    \n")
}

fn impl_col_index(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let indextyp = strtype(&table.index_type());
    let tablename = table_name(&table.name);
    let indexes = col.indexes();
    let field = table_name(&col.info().name);
    let count = indexes.len();
    write!(
        output,
        "static unsigned const {tablename}_{field}_INDEX_COUNT  =  {count};
static {indextyp} {tablename}_{field}_INDEX   [{tablename}_{field}_INDEX_COUNT] = {{",
    )?;

    let width = language::digits(indexes.len());
    for (i, v) in indexes.iter().enumerate() {
        if i % 20 == 0 {
            write!(output, "\n    ")?;
        }
        write!(output, "{:width$}, ", v)?;
    }
    writeln!(output, "\n}};\n")
}

// ================================================================================================
// Table
// ================================================================================================
fn header_table_types(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    if table.has_data() {
        let tablename = table_name(&table.name);
        let strname = struct_name(&table.name);

        let datacols: Vec<&dyn table::Column> = table.data_columns();
        writeln!(output, "typedef struct  {{")?;
        for col in &datacols {
            let info = col.info();
            let fieldtype = strtype(&info.table_type);
            writeln!(output, "    {} {}_;", fieldtype, info.name)?;
        }
        writeln!(output, "}} {strname}_t;")?;

        if table.get_array {
            let count = table.len;
            writeln!(
                output,
                "static unsigned const {tablename}_TABLE_COUNT = {count};
extern const {strname}_t {tablename}_TABLE[{tablename}_TABLE_COUNT];"
            )?;
        }

        if project.table_need_iter(table) {
            header_index(table, output)?;
        }
        writeln!(output)?;
    }
    Ok(())
}

fn header_table_methods(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    writeln!(output, "\n\n// ------    ")?;

    // Labels
    let labelcols: Vec<&dyn table::Column> = table.label_columns();
    for col in labelcols {
        header_col_labels(table, col, output)?;
    }

    //methods for data column
    let datacols: Vec<&dyn table::Column> = table.data_columns();
    for col in &datacols {
        header_getter_col(table, *col, output)?;
        if col.info().has_iter_range() {
            header_iter_range(table, *col, output)?;
        }
    }

    let reverse_join = project.join_to_columns(table);
    for (join, col) in reverse_join {
        header_reverse_join(table, col, &join.name, output)?;
    }
    Ok(())
}

fn impl_table_data(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let tablename = table_name(&table.name);

    let datacols: Vec<&dyn table::Column> = table.data_columns();

    if !table.get_array {
        let count = table.len;
        write!(
            output,
            "static unsigned const {tablename}_TABLE_COUNT = {count};
static "
        )?;
    }

    writeln!(
        output,
        "const {strname}_t {tablename}_TABLE[{tablename}_TABLE_COUNT] = {{"
    )?;

    for row in 0..table.len {
        write!(output, "   {{")?;
        for col in &datacols {
            write!(output, "{}, ", col.emit_table_cell(row))?;
        }
        writeln!(output, "}},")?;
    }
    write!(output, "}};\n\n")?;

    // indexes
    if project.table_need_iter(table) {
        impl_index(table, output)?;
    }
    for col in &datacols {
        if col.info().iterable {
            impl_col_index(table, *col, output)?;
        }
    }

    // Labels
    let labelcols: Vec<&dyn table::Column> = table.label_columns();
    for col in labelcols {
        impl_col_labels(table, col, output)?;
    }

    Ok(())
}

fn impl_table_methods(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    // data column
    let datacols: Vec<&dyn table::Column> = table.data_columns();

    for col in &datacols {
        impl_getter_col(table, *col, output)?;
        if col.info().has_iter_range() {
            impl_iter_range(table, *col, output)?;
        }
    }

    let reverse_join = project.join_to_columns(table);
    for (join, col) in reverse_join {
        impl_reverse_join(table, col, &join.name, output)?;
    }

    Ok(())
}
// ================================================================================================
//  Project
// ================================================================================================
fn header_path(project: &table::Project) -> PathBuf {
    let mut path = PathBuf::from(&project.dst_path);
    path.set_extension("h");
    path
}

fn header_project(project: &table::Project) -> aperror::Result<()> {
    let mut outfile = fs::File::create(&header_path(project))?;
    let output = (&mut outfile) as &mut dyn io::Write;

    let include_guard = project.name().to_shouty_snake_case();
    let notice = language::file_notice();
    writeln!(
        output,
        "// {notice}

#ifndef INSRCDATA_{include_guard}_H
#define INSRCDATA_{include_guard}_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>"
    )?;
    for import in project.imports() {
        writeln!(output, "#include \"{import}\"")?;
    }
    writeln!(output)?;

    for table in &project.tables {
        header_table_types(project, table, output)?;
    }

    for table in &project.tables {
        header_table_methods(project, table, output)?;
    }
    writeln!(output, "\n#endif //  {include_guard}_H ")?;
    Ok(())
}

fn impl_project(project: &table::Project) -> aperror::Result<()> {
    let filename = project.name();

    let mut outfile =
        aperror::io_error_result(fs::File::create(&project.dst_path), &project.dst_path)?;
    let output = (&mut outfile) as &mut dyn io::Write;
    let notice = language::file_notice();
    writeln!(
        output,
        "// {notice}

#include \"{filename}.h\"
#include <string.h>
"
    )?;

    for table in &project.tables {
        if table.has_data() {
            impl_table_data(project, table, output)?;
        }
    }

    for table in &project.tables {
        impl_table_methods(project, table, output)?;
    }
    Ok(())
}

// ================================================================================================
// Entry point
// ================================================================================================
impl language::Language for LangC {
    fn emit(&self, project: &table::Project) -> aperror::Result<()> {
        header_project(project)?;
        impl_project(project)
    }

    fn interface(&self, _project: &table::Project) -> aperror::Result<()> {
        Ok(())
    }

    fn extension(&self) -> String {
        "c".to_string()
    }
}

const LANG_C_: LangC = LangC {};
pub const LANG_C: &'static dyn language::Language = &LANG_C_;
