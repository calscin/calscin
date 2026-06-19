use calsc_ast::path::ElementPath;
use calsc_modules::path::ModulePath;
use calsc_utils::hash::HashedString;

pub fn lower_stage_0_key(key: ElementPath) -> (ModulePath, HashedString) {
    assert!(!key.members.is_empty());

    let path = if key.members.len() == 1 {
        ModulePath::new("prelude".into(), vec![])
    } else {
        ModulePath::new(
            key.members[0].clone(),
            key.members[1..key.members.len() - 1].to_vec(),
        )
    };

    (path, key.members[key.members.len() - 1].clone())
}
