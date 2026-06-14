//! The error declarations

use std::fmt::{Debug, Display};

use calsc_utils::pos::FilePosition;

use crate::{
    Diagnostic, DiagnosticCode, DiagnosticSource, Level, PosDiagnosticSource, declare_diagnostic,
    fmt::fmt_list,
    span::{Span, SpanKind},
};

// Is triggered whenever the Lexer cannot parse something (eg: cannot parse literals).

// Parsing errors
declare_diagnostic!(CANNOT_PARSE, 1);
declare_diagnostic!(UNEXPECTED_TOKEN, 2);
declare_diagnostic!(EXPECTED_TOKEN, 3);

declare_diagnostic!(ALREADY_IN_SCOPE, 4);
declare_diagnostic!(CANNOT_FIND, 5);
declare_diagnostic!(FIELD_MISSING, 6);
declare_diagnostic!(EXPECTED_SIZE_SPECS, 7);
declare_diagnostic!(EXPECTED_TYPE_PARAMETERS, 8);
declare_diagnostic!(VARIABLE_UNALIVE, 9);
declare_diagnostic!(REMIR_ERROR, 10);
declare_diagnostic!(EXPECTED_RETURN, 11);
declare_diagnostic!(RESTRICTED_ARGUMENT_TYPES, 12);
declare_diagnostic!(RESTRICTED_RETURN_TYPE, 13);
declare_diagnostic!(COMPILE_TIME_SIZE, 14);
declare_diagnostic!(NOT_ITERABLE, 15);
declare_diagnostic!(NOT_INITIALIZED, 16);

/// Builds a `CANNOT_PARSE` error (E1) based on the given source and given element.
pub fn build_cannot_parse_error<P: Display, S: DiagnosticSource>(p: &P, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, CANNOT_PARSE),
        format!("cannot parse {}", p),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_unexpected_error<E: Display, S: DiagnosticSource>(elem: &E, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, UNEXPECTED_TOKEN),
        format!("unexpected {}", elem),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, EXPECTED_TOKEN),
        format!("expected {} but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_already_in_scope<E: Display, S: DiagnosticSource>(
    element: &E,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ALREADY_IN_SCOPE),
        format!("{} already in scope", element),
        Some(format!("re-introduction of {} done here", element)),
        vec![],
        vec!["this name is already taken in the current scope".to_string()],
        vec!["try changing the name of the re-introduction".to_string()],
    )
}

pub fn build_cannot_find_element<E: Display, C: Display, S: DiagnosticSource>(
    element: &E,
    closest: &C,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, CANNOT_FIND),
        format!("cannot find {} did you mean {}?", element, closest),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_cannot_find_element_no_closest<E: Display, S: DiagnosticSource>(
    element: &E,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, CANNOT_FIND),
        format!("cannot find {}", element),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_missing_field<E: Display, S: DiagnosticSource>(element: &E, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, FIELD_MISSING),
        format!("missing field {}", element),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_size_specifiers_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, EXPECTED_SIZE_SPECS),
        format!("expected {} size specifiers but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_type_parameters_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, EXPECTED_TYPE_PARAMETERS),
        format!("expected {} type parameters but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_variable_unalive<S: DiagnosticSource, V: Display, I: Display, E: Display>(
    variable: &V,
    introduced: &I,
    expired: &E,
    source: &S,
    start: FilePosition,
    end: FilePosition,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, VARIABLE_UNALIVE),
        format!("variable {} is not alive anymore", variable),
        None,
        vec![Span::new(
            SpanKind::Secondary,
            start,
            end,
            Some(format!("variable {} dropped here", variable)),
        )],
        vec![format!(
            "variable {} introduced in branch {} expired in branch {}",
            variable, introduced, expired
        )],
        vec!["the variable is not available anymore".to_string()],
    )
}

pub fn build_remir_error<E: Debug>(
    error: &E,
    start: FilePosition,
    end: FilePosition,
) -> Diagnostic {
    let source = PosDiagnosticSource::new(start, end);

    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, REMIR_ERROR),
        format!("ReMIR error happened here: {:#?}", error),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_return_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, UNEXPECTED_TOKEN),
        format!("expected return type {} but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_restricted_arument_type<R: Display, S: DiagnosticSource>(
    restricted: &Vec<R>,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, RESTRICTED_ARGUMENT_TYPES),
        format!(
            "argument types are restricted to {} for this function",
            fmt_list(restricted)
        ),
        None,
        vec![],
        vec!["invalid argument types".to_string()],
        vec![format!("change argument types to {}", fmt_list(restricted))],
    )
}

pub fn build_restricted_return_type<R: Display, S: DiagnosticSource>(
    restricted: &R,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, RESTRICTED_RETURN_TYPE),
        format!(
            "return type is restricted to {} for this function",
            restricted
        ),
        None,
        vec![],
        vec!["invalid return type".to_string()],
        vec![format!("change return type to {}", restricted)],
    )
}

pub fn build_compile_time_size<T: Display, S: DiagnosticSource>(ty: &T, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, COMPILE_TIME_SIZE),
        format!("the type {} requires a size at compile time", ty),
        None,
        vec![],
        vec![format!(
            "the type {} does not have a constant comp time size",
            ty
        )],
        vec!["change the type to a type that has a constant size".to_string()],
    )
}

pub fn build_not_iterable<T: Display, S: DiagnosticSource>(
    it_ty: Option<&T>,
    ty: &T,
    source: &S,
) -> Diagnostic {
    if it_ty.is_some() {
        source.make_diagnostic_simple(
            DiagnosticCode::new(Level::Error, NOT_ITERABLE),
            format!("the type {} is not iterable on type {}", it_ty.unwrap(), ty),
            None,
            vec![],
            vec![],
            vec![],
        )
    } else {
        source.make_diagnostic_simple(
            DiagnosticCode::new(Level::Error, NOT_ITERABLE),
            format!("the type {} is not iterable", ty),
            None,
            vec![],
            vec![],
            vec![],
        )
    }
}

pub fn build_not_initialized<V: Display, S: DiagnosticSource>(
    variable: &V,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, NOT_INITIALIZED),
        format!("the variable {} was not initialized", variable),
        None,
        vec![],
        vec![],
        vec![],
    )
}
