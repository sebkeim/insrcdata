// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// trace lint errror
//

use crate::aperror;
use std::cell::Cell;

pub struct Linter {
    emit: bool,
    contexts: Vec<String>,
    errors: Cell<usize>,
}

impl Linter {
    pub fn new() -> Linter {
        Linter {
            emit: true,
            contexts: vec![],
            errors: Cell::new(0),
        }
    }

    pub fn errors(&self) -> usize {
        self.errors.get()
    }

    pub fn str_context(&self) -> String {
        self.contexts.join(":")
    }

    pub fn err(&self, condition: bool, message: &str) {
        if !condition {
            self.errors.replace(self.errors.get() + 1);
            if self.emit {
                eprintln!("!{} {}\n", self.str_context(), message);
            }
        }
    }

    pub fn check_result<R>(&self, prologue: &str, result: aperror::Result<R>) {
        match result {
            Ok(_) => {}
            Err(e) => self.context(prologue, |lt| {
                lt.err(false, &e.to_string());
            }),
        }
    }

    pub fn context<F>(&self, prologue: &str, block: F)
    where
        F: Fn(&Linter),
    {
        let mut contexts = self.contexts.clone();
        contexts.push(prologue.to_string());

        let linter = Linter {
            emit: self.emit,
            contexts,
            errors: Cell::new(0),
        };

        block(&linter);
        self.errors.replace(self.errors.get() + linter.errors.get());
    }

    pub fn row<F>(&self, row: usize, block: F)
    where
        F: Fn(&Linter),
    {
        let num = format!("{}", row);
        self.context(&num, block);
    }
}

// check if label is conform
pub fn label(str: &str) -> bool {
    let mut first = true;
    for c in str.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
        if first && c.is_numeric() {
            // TODO only letters as first character
            return false;
        }
        first = false;
    }
    !str.is_empty()
}

#[cfg(test)]
pub fn test_linter() -> Linter {
    Linter {
        emit: false,
        contexts: vec![],
        errors: Cell::new(0),
    }
}
