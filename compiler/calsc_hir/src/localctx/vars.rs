use calsc_typing::tree::Type;

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LocalContextVariable {
    pub introduced: usize,

    pub ty: Type,

    pub usage_count: usize,
    pub mutation_count: usize,
    pub reference_count: usize,

    pub has_default: bool,

    pub introduced_values: Vec<usize>,
}

impl LocalContextVariable {
    pub fn new(ty: Type, introduced: usize, has_default: bool) -> Self {
        Self {
            introduced,
            ty,
            has_default,
            usage_count: 0,
            mutation_count: 0,
            reference_count: 0,
            introduced_values: vec![],
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
