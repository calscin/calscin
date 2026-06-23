use std::collections::HashSet;

use calsc_typing_v2::types::TypeKind;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub struct LocalContextVariable {
    pub introduced: usize,
    pub mutable: bool,

    pub ty: TypeKind,

    pub usage_count: usize,
    pub mutation_count: usize,
    pub reference_count: usize,

    pub has_default: bool,

    pub introduced_values: HashSet<usize>,
}

impl LocalContextVariable {
    pub fn new(ty: TypeKind, introduced: usize, mutable: bool, has_default: bool) -> Self {
        Self {
            mutable,
            introduced,
            ty,
            has_default,
            usage_count: 0,
            mutation_count: 0,
            reference_count: 0,
            introduced_values: HashSet::new(),
        }
    }

    #[inline(always)]
    pub fn introduce_usage(&mut self) {
        self.usage_count += 1;
    }

    #[inline(always)]
    pub fn introduce_mutation(&mut self) {
        self.mutation_count += 1;
    }

    #[inline(always)]
    pub fn introduce_reference(&mut self) {
        self.reference_count += 1;
    }
}
