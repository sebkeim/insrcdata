use crate::basetype::BaseType;
use crate::{aperror, langc, language, log, table};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use language::write_help;

use crate::table::JoinTo;
use std::{fs, io};

struct LangSwift {}
fn struct_name(table_name: &str) -> String {
    table_name.to_upper_camel_case()
}

fn field_name(col_name: &str) -> String {
    col_name.to_snake_case()
}
fn method_name(col_name: &str) -> String {
    col_name.to_lower_camel_case()
}

fn strtype(typ: &BaseType) -> String {
    String::from(match typ {
        BaseType::Label { name } => name,
        BaseType::Bool => "Bool",
        BaseType::I8 => "Int8",
        BaseType::I16 => "Int16",
        BaseType::I32 => "Int32",
        BaseType::I64 => "Int64",
        BaseType::U8 => "UInt8",
        BaseType::U16 => "UInt16",
        BaseType::U32 => "UInt32",
        BaseType::U64 => "UInt64",
        BaseType::Str => "String",
        BaseType::F32 => "Float32",
        BaseType::F64 => "Float64",
        BaseType::Object { objtype } => objtype,
        BaseType::Join { .. } => "TODO",
        BaseType::Variant => "TODO",
    })
}

fn cast_to(typ: &BaseType, value: &str) -> String {
    match typ {
        BaseType::Str => format!("String(cString: {value})"),
        _ => value.to_string(),
    }
}

fn cast_from(typ: &BaseType, value: &str) -> String {
    match typ {
        BaseType::Str => format!("{value}.cString(using:.utf8)"),
        _ => value.to_string(),
    }
}

fn getter_col(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();

    let field = field_name(col.name());
    let c_field = col.name();
    let c_strname = langc::struct_name(&table.name);

    write_help(output, "    /// ", &info.config.help)?;
    match &info.type_impl() {
        table::TypeImpl::Label => {
            log::verbose("unexpected getter_col for Label type");
        }
        table::TypeImpl::Scalar => {
            let outtype = strtype(&info.interface_type);
            // const {strname}_t* s) {{ return s->{field}_; }}",
            let getter = cast_to(
                &info.interface_type,
                &format!(" {c_strname}_{c_field}(cstruct)"),
            );
            writeln!(output, "    var {field}: {outtype} {{ {getter} }}")?;
        }
        table::TypeImpl::Join => {
            let outtype = struct_name(&info.join_table());

            writeln!(
                output,
                "    var {field} : {outtype} {{ {outtype}(cstruct:{c_strname}_{c_field}(cstruct)) }}",
            )?;
        }
        table::TypeImpl::JoinOptional => {
            let outtype = struct_name(&info.join_table());

            let c_outtype = langc::struct_name(&info.join_table());
            writeln!(
                output,
                "    var {field} : {outtype}? {{
        var p = UnsafePointer<{c_outtype}_t>?(nil)
        if {c_strname}_{c_field}(cstruct, &p) {{ return {outtype}(cstruct:p!) }} else {{ return nil }}
    }}",
            )?;
        }

        table::TypeImpl::Variant => {
            getter_variant(table, col, output)?;
        }
    }
    Ok(())
}

// ================================================================================================
// Range iterator
// ================================================================================================
fn iter_range(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let info = col.info();
    let argtype = strtype(&info.interface_type);
    let field = col.name();
    write_help(output, "    /// ", &info.config.iter_help)?;
    let c_strname = langc::struct_name(&table.name);
    let c_colname = langc::struct_name(col.name());
    let start = cast_from(&info.interface_type, "start");
    let stop = cast_from(&info.interface_type, "stop");
    writeln!(
        output,
        "    static func {field}Range(start:{argtype}, stop:{argtype}) -> {c_strname}_iter_t {{
        {c_strname}_{c_colname}_range({start}, {stop}) 
    }}"
    )
}

// ================================================================================================
// Reverse join
// ================================================================================================
fn reverse_join(table: &table::Table, rj: &JoinTo, output: &mut dyn io::Write) -> io::Result<()> {
    let info = rj.col.info();
    let reverse = &rj.reverse_name;
    let c_strsrc = langc::struct_name(&rj.table.name);
    let c_strname = langc::struct_name(&table.name);
    write_help(output, "    /// ", &info.config.iter_help)?;
    writeln!(
        output,
        "    var {reverse}:{c_strsrc}_iter_t {{ {c_strname}_{reverse}(cstruct) }}"
    )
}

// ================================================================================================
// // Labels
// ================================================================================================

fn col_labels(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    if table.has_data() {
        let c_enumname = langc::enum_type_name(col);
        let c_strname = langc::struct_name(&table.name);
        let methname = method_name(&c_enumname);
        writeln!(
            output,
            "    init(ref:{c_enumname}_t){{
        cstruct = {c_strname}_from_{c_enumname}(ref)
    }}
    var {methname}: {c_enumname}_t {{ {c_strname}_{c_enumname}(cstruct) }}"
        )?;
    }
    Ok(())
}

// ================================================================================================
// Variants
// ================================================================================================
fn variant_type_name(table: &table::Table, col: &dyn table::Column) -> String {
    let strname = struct_name(&table.name);
    let varname = struct_name(col.name());
    format!("{strname}{varname}")
}
fn write_variant(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let Some(variants) = col.variants() else {
        return Ok(());
    };
    let vartypname = variant_type_name(table, col);

    writeln!(output, "enum  {vartypname} : Hashable {{")?;
    for vrn in variants {
        let joinstruct = struct_name(&vrn.name);
        if vrn.is_none {
            writeln!(output, "   case {joinstruct}")?;
        } else {
            writeln!(output, "   case {joinstruct}({joinstruct})")?;
        }
    }

    writeln!(output, "}}\n")
}
/*
var  Wikidata_object:wd  {
            let w = wikidata_object(cstruct)
            switch w.type {
            case WIKIDATA_PERSON: return .PERSON(Person(cstruct:w.person))

            case WIKIDATA_LETTERCASE: return .LETTERCASE(Lettercase(cstruct:w.lettercase))

            default:
                  assert(false)
            }
      }
 */
fn getter_variant(
    table: &table::Table,
    col: &dyn table::Column,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let variants = col.variants().expect("variant must have variant");
    let vartypname = variant_type_name(table, col);
    let field = field_name(col.name());

    let c_field = col.name();
    let c_strname = langc::struct_name(&table.name);

    writeln!(
        output,
        "    var {field} : {vartypname} {{ 
       let w = {c_strname}_{c_field}(cstruct)
       switch w.type {{"
    )?;

    for vrn in variants {
        if vrn.count == 0 {
            continue;
        }

        let c_enuname = format!(
            "{}_{}",
            langc::enum_name(&table.name),
            langc::enum_name(&vrn.name)
        );
        let strct = struct_name(&vrn.name);
        let c_field = langc::struct_name(&vrn.name);
        if vrn.is_none {
            writeln!(output, "             case {c_enuname}: return .{strct}")?;
        } else {
            writeln!(
                output,
                "             case {c_enuname}: return .{strct}({strct}(cstruct:w.{c_field}))"
            )?;
        }
    }

    writeln!(
        output,
        "             default : fatalError( \"insrcdata variant index overflow\")\n        }}\n    }}"
    )
}

// ================================================================================================
//
// ================================================================================================
fn table_data(
    project: &table::Project,
    table: &table::Table,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let strname = struct_name(&table.name);
    let c_struct = langc::struct_name(&table.name);

    // define structure
    write_help(output, "/// ", &table.help)?;
    writeln!(
        output,
        "public struct {strname} : Hashable {{
        var cstruct : UnsafePointer<{c_struct}_t>
    "
    )?;

    // default constructor
    let labelcols: Vec<&dyn table::Column> = table.label_columns();
    if table.has_data() && (!labelcols.is_empty() || table.get_array) {
        writeln!(
            output,
            "    init(cstruct: UnsafePointer<{c_struct}_t>){{
        self.cstruct = cstruct
    }}"
        )?;
    }
    // init from labels
    for col in labelcols {
        col_labels(table, col, output)?;
    }
    // init from index
    if table.get_array {
        let c_tablename = langc::table_name(&table.name);
        //   let strname = langc::struct_name(&table.name);
        writeln!(
            output,
            "    init(index:Int){{
        cstruct = {c_tablename}_TABLE_PTR()+index
    }}"
        )?;
    }

    //methods for data column
    let datacols: Vec<&dyn table::Column> = table.data_columns();
    for col in &datacols {
        getter_col(table, *col, output)?;
        if col.info().has_iter_range() {
            iter_range(table, *col, output)?;
        }
    }

    let reverse_joins = project.join_to_columns(table);
    for rj in reverse_joins {
        reverse_join(table, &rj, output)?;
    }

    writeln!(output, "\n}} // struct {}\n", strname)?;

    if project.table_need_iter(table) {
        header_index(table, output)?;
    }

    for col in datacols {
        write_variant(table, col, output)?;
    }

    writeln!(output, "\n")
}
fn header_index(table: &table::Table, output: &mut dyn io::Write) -> io::Result<()> {
    let strucname = struct_name(&table.name);
    let c_strname = langc::struct_name(&table.name);

    writeln!(
        output,
        "extension {c_strname}_iter_t : Sequence, IteratorProtocol {{
        public typealias Element = {strucname}
        public mutating func next() -> {strucname}? {{
            if let ptr = {c_strname}_next(&self) {{
                return {strucname}(cstruct:ptr)
            }}
            return nil
        }}
}}"
    )
}

// ================================================================================================
// Entry point
// ================================================================================================
impl language::Language for LangSwift {
    fn emit(&self, project: &table::Project) -> aperror::Result<()> {
        // c binding
        langc::header_project(self, project)?;
        langc::impl_project(self, project)?;

        // swift source
        let mut outfile =
            aperror::io_error_result(fs::File::create(&project.dst_path), &project.dst_path)?;
        let output = (&mut outfile) as &mut dyn io::Write;
        write_help(output, "/// ", &project.help)?;

        let notice = language::file_notice();
        writeln!(output, "// {notice}\nimport Foundation\n")?;

        for table in &project.tables {
            table_data(project, table, output)?;
        }
        Ok(())
    }

    fn extension(&self) -> String {
        "swift".to_string()
    }

    // for language that use C binding
    fn c_binding_header(
        &self,
        project: &table::Project,
        output: &mut dyn io::Write,
    ) -> io::Result<()> {
        writeln!(output, "\n// swift bindings")?;

        for table in &project.tables {
            if table.get_array {
                let tablename = langc::table_name(&table.name);
                let strname = langc::struct_name(&table.name);

                writeln!(
                            output,
                            "static inline const {strname}_t* {tablename}_TABLE_PTR() {{ return {tablename}_TABLE; }}"
                        )?;
            }
        }
        writeln!(output)?;

        Ok(())
    }
}

const LANG_SWIFT_: LangSwift = LangSwift {};
pub const LANG_SWIFT: &'static dyn language::Language = &LANG_SWIFT_;
