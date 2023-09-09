// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Parse the .toml configuration file
// using the [crate toml](https://docs.rs/toml/)
//

use crate::colvariant::ColVariant;
use crate::language::Language;
use crate::table::Project;
use crate::{
    aperror, basetype, colbool, colfloat, colint, coljoin, collabel, colobject, colstr, colvariant,
    language, lint, log, table,
};
use csv::StringRecord;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::ToString;

//  index for specific table column
fn colkey(tablename: &str, colname: &str) -> String {
    const TABLE_JOIN_SEPARATOR: &str = ".";
    format!("{}{}{}", tablename, TABLE_JOIN_SEPARATOR, colname)
}

// convert toml parsing error to applicative error
impl From<toml::de::Error> for aperror::Error {
    fn from(err: toml::de::Error) -> aperror::Error {
        aperror::Error::new(&err.to_string())
    }
}

pub struct Runtime {
    pub projectpath: PathBuf,
    pub indir: String,
    pub dest: String,
    pub linter: lint::Linter,
}

impl Runtime {
    pub fn projectdir(&self) -> &Path {
        self.projectpath.parent().unwrap_or(Path::new("."))
    }

    pub fn indir_path(&self) -> &Path {
        if self.indir.is_empty() {
            self.projectdir()
        } else {
            Path::new(&self.indir)
        }
    }

    // builder
    pub fn new(path: &str) -> Runtime {
        Runtime {
            projectpath: PathBuf::from(path),
            indir: "".to_string(),
            dest: "".to_string(),
            linter: lint::Linter::new(),
        }
    }

    pub fn indir(mut self, indir: String) -> Runtime {
        self.indir = indir;
        self
    }
    pub fn dest(mut self, dest: String) -> Runtime {
        self.dest = dest;
        self
    }

    //
    fn projectname(&self) -> String {
        match self.projectpath.file_stem() {
            Some(name) => name.to_str().unwrap_or("invalid"),
            None => "unnamed",
        }
        .to_string()
    }

    /// create project object from config file
    pub fn into_project(self) -> aperror::Result<Project> {
        let path = self.projectpath.as_path();
        let contents = aperror::io_error_result(fs::read_to_string(path), path)?;
        let config: Config = match toml::from_str(&contents) {
            Ok(file) => Ok(file),
            Err(error) => Err(aperror::Error::new(error.message())),
        }?;

        let context = ConfigContext { runtime: self };
        config.project(&context)
    }
}

// ================================================================================================
//  [[table.col.target]]
// ================================================================================================
#[derive(Deserialize)]
struct Target {
    /// generated language
    lang: String,
    /// data type
    r#type: String,
    /// transformer
    template: Option<String>,
    /// global directive
    import: Option<String>,
}

// ================================================================================================
//  [[table.col]]
// ================================================================================================
struct ColContext<'a> {
    table_context: &'a TableContext<'a>,
}

/// data column definition
#[derive(Deserialize)]
struct Col {
    /// name of the field in structure
    name: String,
    /// name of column header in source .csv
    src: Option<String>,
    /// exported data type of field
    format: Option<String>,
    /// generate accessor for range indexing
    range: Option<bool>,
    /// deduplicate similar rows
    single: Option<bool>,
    /// target (for object format)
    target: Option<Vec<Target>>,
    /// convert from row to label
    as_label: Option<String>,
}
static EMPTY_TARGET: Vec<Target> = vec![];
impl Col {
    /// generate column for integer type value

    fn target(&self, lang: &str) -> Option<&Target> {
        self.target
            .as_ref()
            .unwrap_or(&EMPTY_TARGET)
            .iter()
            .find(|&target| target.lang == lang)
    }

    fn create_object(
        &self,
        strvals: &[String],
        ctx: &ColContext,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let lang = ctx.table_context.lang.extension();
        let Some(target) = self.target(&lang) else {
            return Err(aperror::Error::new(&format!("target language {} not defined for column {}", lang, self.name)));
        };

        Ok(Box::new(colobject::ColObject::new(
            &self.name,
            strvals.to_owned(),
            &target.r#type,
            target.template.as_deref().unwrap_or("{}"),
            target.import.as_deref().unwrap_or(""),
        )))
    }

    /// generate column object from configuration
    fn create(
        &self,
        table: &Table,
        tablecols: &HashMap<String, Vec<String>>,
        ctx: &ColContext,
    ) -> aperror::Result<Box<dyn table::Column>> {
        log::log(&format!("create col {}", self.name));
        // retrieve src values
        let src = self.src_name();
        let key = table.key(src);
        let Some(strvals) = tablecols.get(&key) else {
            return Err(aperror::Error::new(&format!("column not found {}", key)));
        };

        let name = &self.name;
        let iterable = self.range.unwrap_or(false);

        // generate column from field type
        let format = self.format.as_deref().unwrap_or("str");
        match format {
            "bool" => colbool::ColBool::parse(name, strvals, iterable),
            "f32" => colfloat::ColF32::parse(name, strvals, iterable),
            "f64" => colfloat::ColF64::parse(name, strvals, iterable),
            "i8" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::I8),
            "i16" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::I16),
            "i32" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::I32),
            "i64" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::I64),
            "u8" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::U8),
            "u16" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::U16),
            "u32" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::U32),
            "u64" => colint::ColInt::parse(name, strvals, iterable, basetype::BaseType::U64),
            "str" => colstr::ColStr::parse(name, strvals, iterable),
            "label" => collabel::ColLabel::parse(name, strvals, self.tolabel(ctx)),
            "object" => self.create_object(strvals, ctx),
            _ => Err(aperror::Error::new(&format!(
                "unknown column type '{}'",
                format
            ))),
        }
    }

    fn src_name(&self) -> &String {
        self.src.as_ref().unwrap_or(&self.name)
    }

    // we use empty value to prevent label column generation
    fn tolabel(&self, ctx: &ColContext) -> &str {
        if ctx.table_context.lang.to_label() {
            self.as_label.as_deref().unwrap_or("")
        } else {
            ""
        }
    }
}

// ================================================================================================
// [[table.join]]
// ================================================================================================

/// join column definition
#[derive(Deserialize)]
struct Join {
    /// name of the field in structure
    name: String,
    /// name of column header in source .csv for join source
    src: Option<String>,
    /// name of column header for join target
    to: String,
    /// name of of table for join target
    external: Option<String>,
    /// allow getter to return an Option
    optional: Option<bool>,
    /// generate accessor for reverse join
    reverse: Option<String>,
}

impl Join {
    /// generate column object from configuration
    fn create(
        &self,
        table: &Table,
        tablecols: &HashMap<String, Vec<String>>,
    ) -> aperror::Result<Box<dyn table::Column>> {
        log::log(&format!("create join {}", self.name));

        // retrieve src values
        let src = self.src_name();
        let key = table.key(src);
        let Some(values) = tablecols.get(&key) else {
            return Err(aperror::Error::new(&format!("column not found {}", key)));
        };

        // target column
        let dest_table = self.external.as_ref().unwrap_or(&table.name);
        let dest_col = colkey(dest_table, &self.to);
        let Some(dest_keys) = tablecols.get(&dest_col) else {
            return Err(aperror::Error::new(&format!("column not found {}", dest_col)));
        };

        Ok(Box::new(coljoin::ColJoin::new(
            &self.name,
            values,
            dest_table,
            dest_keys,
            self.optional.unwrap_or_default(),
            self.reverse.as_ref().unwrap_or(&String::new()),
        )))
    }

    fn src_name(&self) -> &String {
        self.src.as_ref().unwrap_or(&self.name)
    }
}

// ================================================================================================
//  [[table.variont.either]]
// ================================================================================================
#[derive(Deserialize)]
struct Either {
    /// name of column header for join target
    to: String,
    /// name of of table for join target
    external: Option<String>,
    /// generate accessor for reverse join
    reverse: Option<String>,
}

impl Either {
    fn to_dest<'a>(&self, values: &'a [String], dest_table: String) -> colvariant::Dest<'a> {
        colvariant::Dest::new(
            values,
            dest_table,
            self.reverse.as_deref().unwrap_or_default().to_string(),
        )
    }
}

// ================================================================================================
// [[table.variant]]
// ================================================================================================

/// variant join column definition
#[derive(Deserialize)]
struct Variant {
    /// name of the field in structure
    name: String,

    /// name of column header in source .csv for join source
    src: Option<String>,

    /// dest
    either: Vec<Either>,

    /// allow getter to return an Option
    optional: Option<bool>,
}

impl Variant {
    /// generate column object from configuration
    fn create(
        &self,
        table: &Table,
        tablecols: &HashMap<String, Vec<String>>,
    ) -> aperror::Result<Box<dyn table::Column>> {
        log::log(&format!("create variant {}", self.name));

        // retrieve src values
        let src = self.src_name();
        let key = table.key(src);
        let Some(values) = tablecols.get(&key) else {
           return Err(aperror::Error::new(&format!("column not found {}", key)));
        };

        // variants
        let mut dests = Vec::<colvariant::Dest>::new();
        for n in &self.either {
            let dest_table = n.external.as_ref().unwrap_or(&table.name);
            let dest_col = colkey(dest_table, &n.to);
            let Some(dest_keys) = tablecols.get(&dest_col) else {
                return Err(aperror::Error::new(&format!("variant column  {} not found for table {}", n.to, dest_table)));
            };

            dests.push(n.to_dest(dest_keys, dest_table.to_string()));
        }

        ColVariant::parse(
            &self.name,
            values,
            self.optional.unwrap_or_default(),
            &mut dests,
        )
    }

    fn src_name(&self) -> &String {
        self.src.as_ref().unwrap_or(&self.name)
    }
}

// ================================================================================================
// [[table]]
// ================================================================================================
struct TableContext<'a> {
    config_context: &'a ConfigContext,
    lang: &'static dyn Language,
}

#[derive(Deserialize)]
struct Table {
    name: String,
    src: Option<String>,
    array: Option<bool>,
    sorted: Option<bool>,
    col: Option<Vec<Col>>,
    join: Option<Vec<Join>>,
    variant: Option<Vec<Variant>>,
}

static UNIC_SEPARATOR: &str = "\n";

impl Table {
    /// path of .csv data source
    fn src_path(&self, indir: &Path) -> PathBuf {
        let src = match &self.src {
            Some(v) => v.to_string(),
            None => format!("{}.csv", self.name.to_lowercase()),
        };

        let confpath = Path::new(&src);
        if confpath.is_absolute() {
            confpath.to_path_buf()
        } else {
            indir.join(confpath)
        }
    }

    //
    fn unic_indexes(&self, headers: &csv::StringRecord) -> Vec<usize> {
        let mut indexes: Vec<usize> = vec![];
        if let Some(cols) = &self.col {
            for col in cols {
                if col.single.unwrap_or_default() {
                    let src_name = col.src_name();
                    match headers.iter().position(|r| r == src_name) {
                        None => {}
                        Some(value) => {
                            indexes.push(value);
                        }
                    }
                }
            }
        }
        indexes
    }
    fn unic_key(indexes: &Vec<usize>, row: &StringRecord) -> String {
        let mut key = "".to_string();
        for i in indexes {
            let v = row.get(*i).unwrap_or_default();
            key = key + v + UNIC_SEPARATOR;
        }
        key
    }

    /// index for specific table column
    fn key(&self, colname: &str) -> String {
        colkey(&self.name, colname)
    }

    /// read all values by columns
    fn read_values(
        &self,
        indir: &Path,
        cols: &mut HashMap<String, Vec<String>>,
    ) -> aperror::Result<()> {
        let path = self.src_path(indir);
        let path_str = path.to_str().unwrap_or("<undefined>");
        log::log(&format!("open file {path_str}",));
        let file = File::open(&path)?;
        let mut reader = csv::Reader::from_reader(file);

        let headers: csv::StringRecord = match reader.headers() {
            Ok(v) => v.clone(),
            Err(_) => return Err(aperror::Error::new("empty file {path_str}")),
        };

        // read column names
        let mut keys = vec![];
        for i in 0..headers.len() {
            let key = self.key(&headers[i]);
            keys.push(key.to_string());
            if cols.contains_key(&key) {
                return Err(aperror::Error::new(&format!(
                    "duplicate column {key} in {path_str}"
                )));
            }
            cols.insert(key, vec![]);
        }

        // for deduplication
        let unic_indexes = self.unic_indexes(&headers);
        let mut unic_keys: HashSet<String> = HashSet::new();
        unic_keys.insert(UNIC_SEPARATOR.repeat(unic_indexes.len())); // skip empty rows

        // column values
        for result in reader.records() {
            let row = match result {
                Ok(v) => v,
                Err(e) => {
                    return Err(aperror::Error::new(&e.to_string()));
                }
            };

            // deduplicate
            if !unic_indexes.is_empty() {
                let key = Table::unic_key(&unic_indexes, &row);
                if unic_keys.contains(&key) {
                    continue;
                }
                unic_keys.insert(key);
            }

            for (i, _) in headers.iter().enumerate() {
                let Some(val) =  row.get(i) else {
                    return Err(aperror::Error::new("short row"));
                };
                let key = &keys[i];
                let col = cols.get_mut(key).expect("col must exist");
                col.push(val.to_string());
            }
        }
        Ok(())
    }

    /// create table object
    fn create(&self, tablecols: &HashMap<String, Vec<String>>, ctx: &TableContext) -> table::Table {
        log::log(&format!("create table {}", self.name));
        let _sorted = self.sorted; // silent dead code warning until the option is actually used

        let mut columns = vec![];
        let runtime = &ctx.config_context.runtime;

        let col_context = ColContext { table_context: ctx };
        if let Some(cols) = &self.col {
            for col in cols {
                let res = col.create(self, tablecols, &col_context);
                match res {
                    Ok(c) => columns.push(c),
                    Err(_) => runtime.linter.check_result(&self.name, res),
                }
            }
        }

        if let Some(joins) = &self.join {
            for join in joins {
                let res = join.create(self, tablecols);
                match res {
                    Ok(c) => columns.push(c),
                    Err(_) => runtime.linter.check_result(&self.name, res),
                }
            }
        }

        if let Some(variants) = &self.variant {
            for variant in variants {
                let res = variant.create(self, tablecols);
                match res {
                    Ok(c) => columns.push(c),
                    Err(_) => runtime.linter.check_result(&self.name, res),
                }
            }
        }

        let get_array = self.array.unwrap_or(false);

        table::Table::new(&self.name, columns, get_array)
    }
}

// ================================================================================================
// root of configuration file
// ================================================================================================

#[derive(Deserialize)]
struct Config {
    dest: Option<String>,
    table: Vec<Table>,
}
struct ConfigContext {
    runtime: Runtime,
}
impl Config {
    /// read columns values for all tables of project
    fn read_values(&self, runtime: &Runtime) -> HashMap<String, Vec<String>> {
        let indir = runtime.indir_path();
        let mut tablecols: HashMap<String, Vec<String>> = HashMap::new();
        for table in &self.table {
            let result = table.read_values(indir, &mut tablecols);
            runtime.linter.check_result(&table.name, result);
        }
        tablecols
    }

    /// path of generated file
    fn dst_path(&self, runtime: &Runtime) -> PathBuf {
        let dst = if runtime.dest.is_empty() {
            self.dest.as_deref().unwrap_or("./")
        } else {
            &runtime.dest
        };
        let dst = if dst.ends_with('/') {
            format!("{}/{}.rs", dst, runtime.projectname())
        } else {
            dst.to_string()
        };

        let confpath = Path::new(&dst);
        if confpath.is_absolute() {
            confpath.to_path_buf()
        } else {
            runtime.projectdir().join(confpath)
        }
    }

    /// create project object
    fn project(&self, ctx: &ConfigContext) -> aperror::Result<Project> {
        let dst_path = self.dst_path(&ctx.runtime);

        let lang = language::language_for_dest(dst_path.clone());

        let values = self.read_values(&ctx.runtime);
        let mut tables = vec![];
        let table_context = TableContext {
            config_context: ctx,
            lang,
        };
        let mut src_paths: Vec<PathBuf> = vec![ctx.runtime.projectpath.clone()];
        for table in &self.table {
            let t = table.create(&values, &table_context);
            tables.push(t);
            src_paths.push(table.src_path(ctx.runtime.indir_path()))
        }

        let project = Project {
            dst_path,
            lang,
            tables,
            src_paths,
        };
        log::log("configuration file processed");

        let linter = &ctx.runtime.linter;
        project.lint(linter);
        if linter.errors() > 0 {
            return Err(aperror::Error::new(&format!(
                "{} lint failures",
                linter.errors(),
            )));
        }

        Ok(project)
    }
}
