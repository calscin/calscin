//! The definitions for the diagnostic container. It's main purpose is to contain diagnostics statically

use std::cell::RefCell;

use crate::{Diagnostic, Level};

thread_local! {
    static DIAGNOSTIC_CONTAINER: RefCell<Vec<Diagnostic>> = RefCell::new(vec![]);
}

pub(crate) fn push_diagnostic(diagnostic: Diagnostic) {
    DIAGNOSTIC_CONTAINER.with_borrow_mut(|f| f.push(diagnostic))
}

pub fn has_diagnostics() -> bool {
    DIAGNOSTIC_CONTAINER.with_borrow(|f| !f.is_empty())
}

pub fn has_errors() -> bool {
    let mut has_errors = false;

    DIAGNOSTIC_CONTAINER.with_borrow(|f| {
        for d in f {
            if d.code.level == Level::Error {
                has_errors = true;
                break;
            }
        }
    });

    has_errors
}

pub fn dump_diagnostics() {
    DIAGNOSTIC_CONTAINER.with_borrow(|f| {
        for d in f {
            println!("{}", d);
        }
    })
}

pub fn clear_diagnostics() {
    DIAGNOSTIC_CONTAINER.with_borrow_mut(|f| f.clear());
}

pub fn dump_and_stop_if_errors() {
    dump_diagnostics();

    if has_errors() {
        std::process::exit(1);
    }
}
