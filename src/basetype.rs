// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// column data type
//

use std::{cmp, fmt};

#[derive(PartialEq, Eq)]
pub enum BaseType {
    #[allow(dead_code)]
    Label,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Str,
    Join {
        strname: String,
        mincard: usize,
        maxcard: usize,
    },
}

// Integer type
fn signed_type(v: i64) -> BaseType {
    match v {
        0..=0x7F => BaseType::I8,
        0x80..=0x7FFF => BaseType::I16,
        0x8000..=0x7FFF_FFFF => BaseType::I32,
        _ => BaseType::I64,
    }
}

pub fn int_type_for_range(min: i64, max: i64) -> BaseType {
    if min < 0 {
        let limit = cmp::max(-min, max);
        signed_type(limit)
    } else {
        match max {
            0..=0xFF => BaseType::U8,
            0x100..=0xFFFF => BaseType::U16,
            0x1_0000..=0xFFFF_FFFF => BaseType::U32,
            _ => BaseType::U64,
        }
    }
}

pub enum TypeImpl {
    Label,
    Join01,
    Join11,
    Scalar,
}

impl BaseType {
    pub fn max(&self) -> usize {
        match self {
            BaseType::Label => 0,
            BaseType::I8 => i8::MAX as usize,
            BaseType::I16 => i16::MAX as usize,
            BaseType::I32 => i32::MAX as usize,
            BaseType::I64 => i64::MAX as usize,
            BaseType::U8 => u8::MAX as usize,
            BaseType::U16 => u16::MAX as usize,
            BaseType::U32 => u32::MAX as usize,
            BaseType::U64 => u64::MAX as usize,
            BaseType::Str => 0,
            BaseType::Join { .. } => 0,
        }
    }

    pub fn min(&self) -> isize {
        match self {
            BaseType::Label => 0,
            BaseType::I8 => i8::MIN as isize,
            BaseType::I16 => i16::MIN as isize,
            BaseType::I32 => i32::MIN as isize,
            BaseType::I64 => i64::MIN as isize,
            BaseType::U8 => u8::MIN as isize,
            BaseType::U16 => u16::MIN as isize,
            BaseType::U32 => u32::MIN as isize,
            BaseType::U64 => u64::MIN as isize,
            BaseType::Str => 0,
            BaseType::Join { .. } => 0,
        }
    }

    pub fn type_impl(&self) -> TypeImpl {
        match self {
            BaseType::Label => TypeImpl::Label,
            BaseType::I8
            | BaseType::I16
            | BaseType::I32
            | BaseType::I64
            | BaseType::U8
            | BaseType::U16
            | BaseType::U32
            | BaseType::U64
            | BaseType::Str => TypeImpl::Scalar,
            BaseType::Join {
                strname: _,
                mincard,
                maxcard,
            } => {
                if *maxcard == 1 {
                    if *mincard == 0 {
                        return TypeImpl::Join01;
                    }

                    if *mincard == 1 {
                        return TypeImpl::Join11;
                    }
                }
                panic!("join n m cardinality  still to do");
            }
        }
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Label => write!(f, ""),
            BaseType::I8 => write!(f, "i8"),
            BaseType::I16 => write!(f, "i16"),
            BaseType::I32 => write!(f, "i32"),
            BaseType::I64 => write!(f, "i64"),
            BaseType::U8 => write!(f, "u8"),
            BaseType::U16 => write!(f, "u16"),
            BaseType::U32 => write!(f, "u32"),
            BaseType::U64 => write!(f, "u64"),
            BaseType::Str => write!(f, "&'static str"),
            BaseType::Join {
                strname,
                mincard,
                maxcard,
            } => write!(f, "join({},{},{})", strname, mincard, maxcard),
        }
    }
}
