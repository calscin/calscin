use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_typing::types::primitive::PrimitiveType;

use crate::ctx::{LoweredTypeContainer, TreeLowCtx, TreeLoweredEntry};

macro_rules! register_prelude_type {
    ($ctx: expr, $name: literal, $val: expr) => {
        $ctx.data.lowered_map.insert(
            ModulePath::new_prelude_path(vec![$name.into()]),
            TreeLoweredEntry::Type(LoweredTypeContainer($val, Visibility::Uncopiable)),
        )
    };
}

pub fn apply_lower_prelude(ctx: &mut TreeLowCtx) {
    register_prelude_type!(ctx, "s", PrimitiveType::Int(true));
    register_prelude_type!(ctx, "u", PrimitiveType::Int(false));
    register_prelude_type!(ctx, "f", PrimitiveType::Float);
    register_prelude_type!(ctx, "size", PrimitiveType::Size);
    register_prelude_type!(ctx, "str", PrimitiveType::Str);
    register_prelude_type!(ctx, "bool", PrimitiveType::Boolean);
}
