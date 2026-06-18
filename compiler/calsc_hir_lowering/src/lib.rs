#![deny(unsafe_code)]

use calsc_modules::path::ModulePath;
use calsc_modules::visibility::Visibility;

pub mod modules;
pub mod modules_lower;
pub mod stage0;
pub mod stage1;
pub mod stage2;

pub fn convert_visibility(
    visibility: Option<calsc_ast::visibility::Visibility>,
    module_path: ModulePath,
) -> Visibility {
    let visibility = visibility.unwrap_or(calsc_ast::visibility::Visibility::Protected);

    match visibility {
        calsc_ast::visibility::Visibility::Public => Visibility::Public,
        calsc_ast::visibility::Visibility::Protected => Visibility::Protected(module_path),
        calsc_ast::visibility::Visibility::Private => Visibility::Private(module_path),
    }
}
