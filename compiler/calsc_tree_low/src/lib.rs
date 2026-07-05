use calsc_modules::{path::ModulePath, visibility::Visibility};

pub mod ctx;
pub mod lower;
pub mod types;

pub fn convert_visibility(
    visibility: Option<calsc_ast::visibility::Visibility>,
    module_path: &ModulePath,
) -> Visibility {
    let visibility = visibility.unwrap_or(calsc_ast::visibility::Visibility::Protected);
    let parent = module_path.everything_but_last();

    match visibility {
        calsc_ast::visibility::Visibility::Public => Visibility::Public,
        calsc_ast::visibility::Visibility::Protected => Visibility::Protected(parent),
        calsc_ast::visibility::Visibility::Private => Visibility::Private(parent),
    }
}
