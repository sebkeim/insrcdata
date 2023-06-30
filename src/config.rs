// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Parse the .toml configuration file
// using the [crate toml](https://docs.rs/toml/)
//

use crate::table::Project;
use crate::{
    aperror, basetype, colint, coljoin, collabel, colstr, langrust, language, lint, log, table,
};
use csv::StringRecord;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::time::SystemTime;

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

pub struct Runtime<'a> {
    pub projectpath: &'a Path,
    pub indir: String,
    pub dest: String,
    pub linter: &'a lint::Linter,
}

impl Runtime<'_> {
    pub fn projectdir(&self) -> &Path {
        let path = self.projectpath.parent().unwrap_or(Path::new("."));
        path
    }

    pub fn indir_path(&self) -> &Path {
        if self.indir.is_empty() {
            self.projectdir()
        } else {
            Path::new(&self.indir)
        }
    }
}

// ================================================================================================
//  [[table.col]]
// ================================================================================================

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
}

impl Col {
    /// generate column for integer type value
    fn create_int(
        &self,
        strvals: &[String],
        typ: basetype::BaseType,
    ) -> aperror::Result<Box<dyn table::Column>> {
        let mut vals: Vec<i64> = vec![];

        for (i, s) in strvals.iter().enumerate() {
            let Ok(v) =  s.parse::<i64>() else {
                return Err(aperror::Error::new(&format!("{} not a number at row {}", s, i)));
            };
            vals.push(v);
        }

        Ok(Box::new(colint::ColInt::new(
            typ,
            &self.name,
            vals,
            self.range.unwrap_or(false),
        )))
    }

    /// generate column object from configuration
    fn create(
        &self,
        table: &Table,
        tablecols: &HashMap<String, Vec<String>>,
    ) -> aperror::Result<Box<dyn table::Column>> {
        // retrieve src values
        let src = self.src_name();
        let key = table.key(src);
        let Some(strvals) = tablecols.get(&key) else {
            return Err(aperror::Error::new(&format!("column not found {}", key)));
        };

        // generate column from flied type
        let default_format = "str".to_string();
        let format = self.format.as_ref().unwrap_or(&default_format);
        match format.as_str() {
            "str" => Ok(Box::new(colstr::ColStr::new(
                &self.name,
                strvals.clone(),
                self.range.unwrap_or(false),
            ))),
            "i8" => self.create_int(strvals, basetype::BaseType::I8),
            "i16" => self.create_int(strvals, basetype::BaseType::I16),
            "i32" => self.create_int(strvals, basetype::BaseType::I32),
            "i64" => self.create_int(strvals, basetype::BaseType::I64),
            "u8" => self.create_int(strvals, basetype::BaseType::U8),
            "u16" => self.create_int(strvals, basetype::BaseType::U16),
            "u32" => self.create_int(strvals, basetype::BaseType::U32),
            "u64" => self.create_int(strvals, basetype::BaseType::U64),
            "label" => Ok(Box::new(collabel::ColLabel::new(
                &self.name,
                strvals.clone(),
            ))),
            _ => Err(aperror::Error::new("unknown column type")),
        }
    }

    fn src_name(&self) -> &String {
        self.src.as_ref().unwrap_or(&self.name)
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
    allow_null: Option<bool>,
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
            self.allow_null.unwrap_or_default(),
            self.reverse.as_ref().unwrap_or(&String::new()),
        )))
    }

    fn src_name(&self) -> &String {
        self.src.as_ref().unwrap_or(&self.name)
    }
}

// ================================================================================================
// [[table]]
// ================================================================================================

#[derive(Deserialize)]
struct Table {
    name: String,
    src: Option<String>,
    r#trait: Option<String>,
    array: Option<bool>,
    sorted: Option<bool>,
    col: Option<Vec<Col>>,
    join: Option<Vec<Join>>,
}
static EMPTY_COLS: Vec<Col> = vec![];
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
        let cols = self.col.as_ref().unwrap_or(&EMPTY_COLS);
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
        log::log(&format!(
            "open file {}",
            path.to_str().unwrap_or("<undefined>")
        ));
        let file = File::open(&path)?;
        let mut reader = csv::Reader::from_reader(file);

        let headers: csv::StringRecord = match reader.headers() {
            Ok(v) => v.clone(),
            Err(_) => return Err(aperror::Error::new("empty file")),
        };

        // read column names
        let mut keys = vec![];
        for i in 0..headers.len() {
            let key = self.key(&headers[i]);
            keys.push(key.to_string());
            if cols.contains_key(&key) {
                return Err(aperror::Error::new(&format!("duplicate column : {}", key)));
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
    fn create(&self, tablecols: &HashMap<String, Vec<String>>, runtime: &Runtime) -> table::Table {
        let _sorted = self.sorted; // silent dead code warning until the option is actually used

        let mut columns = vec![];

        let cols = self.col.as_ref().unwrap_or(&EMPTY_COLS);
        for col in cols {
            let res = col.create(self, tablecols);
            match res {
                Ok(c) => columns.push(c),
                Err(_) => runtime.linter.check_result(&self.name, res),
            }
        }

        let emptyjoins: Vec<Join> = vec![];
        let joins = match &self.join {
            Some(v) => v,
            None => &emptyjoins,
        };
        for join in joins {
            let res = join.create(self, tablecols);
            match res {
                Ok(c) => columns.push(c),
                Err(_) => runtime.linter.check_result(&self.name, res),
            }
        }

        let itrait = match &self.r#trait {
            Some(itrait) => itrait,
            None => "",
        };

        let get_array = self.array.unwrap_or(false);

        table::Table::new(&self.name, columns, itrait, get_array)
    }

    /// modification time of table source.csv
    fn last_modified(&self, indir: &Path) -> SystemTime {
        let path = self.src_path(indir);
        match fs::metadata(path) {
            Ok(v) => v.modified().unwrap_or(SystemTime::now()),
            Err(_) => SystemTime::now(), // force rebuild if source.csv unavailable
        }
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

impl Config {
    /// read columns values for all tables of project
    fn read_values(&self, indir: &Path, runtime: &Runtime) -> HashMap<String, Vec<String>> {
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
            match &self.dest {
                Some(dst) => dst.to_string(),
                None => {
                    let defaultname = match runtime.projectpath.file_stem() {
                        Some(name) => name.to_os_string().into_string().unwrap(),
                        None => "unnamed".to_string(),
                    };
                    let lang = langrust::RUST;
                    format!("{}.{}", defaultname, lang.extension())
                }
            }
        } else {
            runtime.dest.to_string()
        };
        let confpath = Path::new(&dst);
        if confpath.is_absolute() {
            confpath.to_path_buf()
        } else {
            runtime.projectdir().join(confpath)
        }
    }

    /// create project object
    fn project(&self, runtime: &Runtime, project_modified: SystemTime) -> aperror::Result<Project> {
        let values = self.read_values(runtime.indir_path(), runtime);
        let mut tables = vec![];
        for table in &self.table {
            let t = table.create(&values, runtime);
            tables.push(t);
        }

        let dst_path = self.dst_path(runtime);

        let lang = language::language_for_dest(dst_path.clone());

        let table_modified = self
            .table
            .iter()
            .map(|x| x.last_modified(runtime.indir_path()))
            .max()
            .unwrap_or(project_modified);
        let src_modified = std::cmp::max(project_modified, table_modified);

        let project = Project {
            dst_path,
            lang,
            tables,
            src_modified,
        };

        log::log("configuration file processed");
        Ok(project)
    }
}

// ================================================================================================
// entry point
// ================================================================================================

/// create project object from config file
pub fn read(runtime: &Runtime) -> aperror::Result<Project> {
    let metadata =
        aperror::io_error_result(fs::metadata(runtime.projectpath), runtime.projectpath)?;
    let project_modified = metadata.modified()?;

    let contents =
        aperror::io_error_result(fs::read_to_string(runtime.projectpath), runtime.projectpath)?;

    let config: Config = match toml::from_str(&contents) {
        Ok(file) => Ok(file),
        Err(error) => Err(aperror::Error::new(error.message())),
    }?;

    config.project(runtime, project_modified)
}
