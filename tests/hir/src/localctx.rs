#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_hir::globalctx::key::GlobalContextKey;

#[cfg(test)]
use calsc_hir::localctx::LocalContext;

#[cfg(test)]
use calsc_typing::{
    base::{BaseType, instance::BaseTypeInstance, kind::BaseTypeKind},
    tree::Type,
};

#[test]
fn test_alive_branch_simple() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let branch = ctx.start_branch();

    assert!(ctx.is_branch_alive(branch));

    ctx.end_branch(branch, &origin);
    assert!(!ctx.is_branch_alive(branch));
}

#[test]
fn test_alive_variable() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let sample_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Boolean),
        vec![],
        vec![],
    ));

    let branch = ctx.start_branch();

    let var = ctx
        .introduce_variable("test".into(), sample_type.clone(), false, &origin)
        .unwrap_cleanly();

    assert!(ctx.is_variable_alive(var));

    ctx.end_branch(branch, &origin);

    assert!(!ctx.is_variable_alive(var));
}

#[test]
fn test_alive_variable_next_branch() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let sample_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Boolean),
        vec![],
        vec![],
    ));

    let branch = ctx.start_branch();

    let var = ctx
        .introduce_variable_next_branch("test".into(), sample_type.clone(), false, &origin)
        .unwrap_cleanly();

    assert!(!ctx.is_variable_alive(var));

    let branch2 = ctx.start_branch();

    assert!(ctx.is_variable_alive(var));

    ctx.end_branch(branch2, &origin);
    ctx.end_branch(branch, &origin);

    assert!(!ctx.is_variable_alive(var));
}

#[test]
fn test_variable_gather() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let sample_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Boolean),
        vec![],
        vec![],
    ));

    let _ = ctx.start_branch();

    let var = ctx
        .introduce_variable("test".into(), sample_type.clone(), false, &origin)
        .unwrap_cleanly();

    let var2 = ctx.obtain("test".into(), &origin).unwrap_cleanly();

    assert_eq!(var, var2);
}

#[test]
pub fn test_ending_point() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let branch = ctx.start_branch();

    assert!(ctx.is_code_in_branch_alive(branch));

    ctx.introduce_ending_point();

    assert!(!ctx.is_code_in_branch_alive(branch));

    ctx.end_branch(branch, &origin);
    let _ = ctx.start_branch();

    assert!(ctx.meets_ending_point_requirement());
}

#[test]
fn test_ending_point_unreal_branches() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    ctx.contain_unreal_branches = true;

    let branch1 = ctx.start_branch();

    assert!(ctx.is_code_in_branch_alive(branch1));

    let branch2 = ctx.start_branch();

    ctx.introduce_ending_point();

    ctx.end_branch(branch2, &origin);
    ctx.move_branch(branch1);

    assert!(ctx.is_code_in_branch_alive(branch1));
    assert!(!ctx.is_code_in_branch_alive(branch2));

    ctx.end_branch(branch1, &origin);

    assert!(!ctx.meets_ending_point_requirement());
}

#[test]
fn test_ending_point_real_branches() {
    let origin = PosDiagnosticSource::new(Default::default(), Default::default());
    let key = GlobalContextKey::new("test".into());

    let mut ctx = LocalContext::new("test".into(), key, None, false);

    let branch1 = ctx.start_branch();

    assert!(ctx.is_code_in_branch_alive(branch1));

    let branch2 = ctx.start_branch();

    ctx.introduce_ending_point();

    ctx.end_branch(branch2, &origin);
    ctx.move_branch(branch1);

    assert!(ctx.is_code_in_branch_alive(branch1));
    assert!(!ctx.is_code_in_branch_alive(branch2));

    ctx.end_branch(branch1, &origin);

    assert!(!ctx.meets_ending_point_requirement());
}
