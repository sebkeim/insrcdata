// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// column data type
//

use std::ops::RangeInclusive;
use std::{cmp, fmt};

#[derive(PartialEq, Eq)]
pub enum BaseType {
    #[allow(dead_code)]
    Label {
        name: String,
    },
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    Object {
        objtype: String,
    },
    Join {
        strname: String,
    },

    Variant,
}

// Integer type needed to handle valuesin range
pub fn int_type_for_range(range: RangeInclusive<i64>) -> BaseType {
    let min = *range.start();
    let max = *range.end();
    if min < 0 {
        // signed integer
        match cmp::max(-min, max) {
            0..=0x7F => BaseType::I8,
            0x80..=0x7FFF => BaseType::I16,
            0x8000..=0x7FFF_FFFF => BaseType::I32,
            _ => BaseType::I64,
        }
    } else {
        // unsigned integer
        match max {
            0..=0xFF => BaseType::U8,
            0x100..=0xFFFF => BaseType::U16,
            0x1_0000..=0xFFFF_FFFF => BaseType::U32,
            _ => BaseType::U64,
        }
    }
}

impl BaseType {
    pub fn max(&self) -> usize {
        match self {
            BaseType::I8 => i8::MAX as usize,
            BaseType::I16 => i16::MAX as usize,
            BaseType::I32 => i32::MAX as usize,
            BaseType::I64 => i64::MAX as usize,
            BaseType::U8 => u8::MAX as usize,
            BaseType::U16 => u16::MAX as usize,
            BaseType::U32 => u32::MAX as usize,
            BaseType::U64 => u64::MAX as usize,
            BaseType::Bool
            | BaseType::F32
            | BaseType::F64
            | BaseType::Str
            | BaseType::Label { .. }
            | BaseType::Join { .. }
            | BaseType::Object { .. }
            | BaseType::Variant => 0,
        }
    }

    pub fn min(&self) -> isize {
        match self {
            BaseType::I8 => i8::MIN as isize,
            BaseType::I16 => i16::MIN as isize,
            BaseType::I32 => i32::MIN as isize,
            BaseType::I64 => i64::MIN as isize,
            BaseType::U8 => u8::MIN as isize,
            BaseType::U16 => u16::MIN as isize,
            BaseType::U32 => u32::MIN as isize,
            BaseType::U64 => u64::MIN as isize,
            BaseType::Bool
            | BaseType::F32
            | BaseType::F64
            | BaseType::Str
            | BaseType::Label { .. }
            | BaseType::Join { .. }
            | BaseType::Object { .. }
            | BaseType::Variant => 0,
        }
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Label { name } => write!(f, "label({})", name),
            BaseType::Bool => write!(f, "bool"),
            BaseType::I8 => write!(f, "i8"),
            BaseType::I16 => write!(f, "i16"),
            BaseType::I32 => write!(f, "i32"),
            BaseType::I64 => write!(f, "i64"),
            BaseType::U8 => write!(f, "u8"),
            BaseType::U16 => write!(f, "u16"),
            BaseType::U32 => write!(f, "u32"),
            BaseType::U64 => write!(f, "u64"),
            BaseType::Str => write!(f, "&'static str"),
            BaseType::F32 => write!(f, "f32"),
            BaseType::F64 => write!(f, "f64"),
            BaseType::Join { strname } => write!(f, "join({})", strname),
            BaseType::Object { objtype } => write!(f, "object({})", objtype),
            BaseType::Variant => write!(f, "variant"),
        }
    }
}
