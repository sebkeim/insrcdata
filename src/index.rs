// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Index generator
//

pub fn index<T: Ord>(values: &Vec<T>) -> Vec<usize> {
    let mut indexes = Vec::from_iter(0..values.len());
    indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
    indexes
}

pub fn filter_index<T: Ord>(values: &Vec<T>, exclude: T) -> Vec<usize> {
    let mut indexes = Vec::from_iter((0..values.len()).filter(|a| values[*a] != exclude));
    indexes.sort_by(|a, b| values[*a].cmp(&values[*b]));
    indexes
}
