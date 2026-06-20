//! The error declarations

use std::fmt::{Debug, Display};

use calsc_utils::pos::FilePosition;

use crate::{
    Diagnostic, DiagnosticCode, DiagnosticSource, Level, PosDiagnosticSource,
    fmt::fmt_list,
    span::{Span, SpanKind},
};

// Is triggered whenever the Lexer cannot parse something (eg: cannot parse literals).

enum ErrorCode {
    CannotParse,
    UnexpectedToken,
    ExpectedToken,

    // Typing
    UnexpectedType,
    ExpectedType,
    ExpectedSimpleType,
    ExpectedReferencable,
    FieldMissing,
    _FunctionMissing,
    ExpectedSizeSpecifiers,
    ExpectedTypeParameters,
    _ExpectedRealType,
    ExpectedCompileTimeType,
    NotIterable,
    InfiniteSize,
    TypeNotStatic,

    // HIR local context
    AlreadyInScope,
    ElementNotAlive,
    NotInitialized,

    // HIR Global context
    CannotFind,
    ExpectedEntryType,
    AdditionalTypeAliasParameters,
    ElementUnreadable,

    // HIR misc
    RestrictedArgumentTypes,
    RestrictedReturnType,
    _UnexpectedReturn,
    ExpectedReturn,
    ExpectedMutableLike,

    // MIR
    RemirError,

    // Internal
    InternalHIRNode,
    InternalSingleton,
}

pub enum InternalErrors {
    CannotFindReturnType,
    StrongerTypeLiterals,
}

/// Builds a `CANNOT_PARSE` error (E1) based on the given source and given element.
pub fn build_cannot_parse_error<P: Display, S: DiagnosticSource>(p: &P, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::CannotParse as usize),
        format!("cannot parse {}", p),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_unexpected_token_error<E: Display, S: DiagnosticSource>(
    elem: &E,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::UnexpectedToken as usize),
        format!("unexpected token {}", elem),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_token_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedToken as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::AlreadyInScope as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::CannotFind as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::CannotFind as usize),
        format!("cannot find {}", element),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_missing_field<E: Display, S: DiagnosticSource>(element: &E, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::FieldMissing as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedSizeSpecifiers as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedTypeParameters as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::ElementNotAlive as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::RemirError as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedReturn as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::RestrictedArgumentTypes as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::RestrictedReturnType as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedCompileTimeType as usize),
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
            DiagnosticCode::new(Level::Error, ErrorCode::NotIterable as usize),
            format!("the type {} is not iterable on type {}", it_ty.unwrap(), ty),
            None,
            vec![],
            vec![],
            vec![],
        )
    } else {
        source.make_diagnostic_simple(
            DiagnosticCode::new(Level::Error, ErrorCode::NotIterable as usize),
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
        DiagnosticCode::new(Level::Error, ErrorCode::NotInitialized as usize),
        format!("the variable {} was not initialized", variable),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_entry_type<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedEntryType as usize),
        format!("expected element of type {} but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_unexpected_type_alias_additional_parameters<S: DiagnosticSource>(
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(
            Level::Error,
            ErrorCode::AdditionalTypeAliasParameters as usize,
        ),
        "unexpected additional parameters on type alias".to_string(),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_unexpected_type_error<T: Display, S: DiagnosticSource>(
    ty: &T,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::UnexpectedType as usize),
        format!("unexpected type {}", ty),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_type_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedType as usize),
        format!("expected type {} but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_simple_type<S: DiagnosticSource>(source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedSimpleType as usize),
        "expected simple type".to_string(),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_mutable<S: DiagnosticSource>(source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedMutableLike as usize),
        "expected a mutable-like value".to_string(),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_referencable<S: DiagnosticSource>(source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ExpectedReferencable as usize),
        "expected a referencable-like value".to_string(),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_internal_hir_node_leaked<N: Debug, S: DiagnosticSource>(
    node: &N,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::InternalHIRNode as usize),
        format!("Internal HIR node {:#?} leaked!", node),
        None,
        vec![],
        vec!["please report this issue immediately".to_string()],
        vec!["https://github.com/calscin/calscin".to_string()],
    )
}

pub fn build_internal_singleton_error<S: DiagnosticSource>(
    internal: InternalErrors,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::InternalSingleton as usize),
        format!("Internal singleton error: #{}", internal as usize),
        None,
        vec![],
        vec!["please report this issue immediately".to_string()],
        vec!["https://github.com/calscin/calscin".to_string()],
    )
}

pub fn build_unreadable_element_visibility<S: DiagnosticSource, E: Display, P: Display>(
    element: &E,
    path_trying: &P,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::ElementUnreadable as usize),
        format!(
            "element {} unreadable in module {} due to visibility rules",
            element, path_trying
        ),
        None,
        vec![],
        vec![format!(
            "change the visibility specifier of {} to pub or prot",
            element
        )],
        vec!["".to_string()],
    )
}

pub fn build_type_infinite_size<S: DiagnosticSource, T: Display>(ty: &T, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::InfiniteSize as usize),
        format!("type {} includes itself and thus has an infinite size", ty),
        Some(format!("infinite size of {} stated here", ty)),
        vec![],
        vec!["".to_string()],
        vec![format!("remove the inner {} field / type declaration", ty)],
    )
}

pub fn build_type_not_static<S: DiagnosticSource, T: Display>(ty: &T, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, ErrorCode::TypeNotStatic as usize),
        format!(
            "type  {} is not static and the value might expire in the future",
            ty
        ),
        Some(format!("static type mention of type {} here", ty)),
        vec![],
        vec!["this type is not static and potentially cannot hold a value for the entire duration of storage".to_string()],
        vec![format!("replace {} by a static type (eg: integers, floats, ...)", ty)],
    )
}
