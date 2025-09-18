use crate::datastore::MultipleSort;

#[derive(Debug, Default)]
pub struct MultipleSortBuilder {
    by_field: String,
    by_direction: String,
    is_case_sensitive_sorting: bool,
}

impl MultipleSortBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn by_field(mut self, value: impl Into<String>) -> Self {
        self.by_field = value.into();
        self
    }

    pub fn by_direction(mut self, value: impl Into<String>) -> Self {
        self.by_direction = value.into();
        self
    }

    pub fn case_sensitive_sorting(mut self, value: bool) -> Self {
        self.is_case_sensitive_sorting = value;
        self
    }

    pub fn build(self) -> MultipleSort {
        MultipleSort {
            by_field: self.by_field,
            by_direction: self.by_direction,
            is_case_sensitive_sorting: self.is_case_sensitive_sorting,
        }
    }
}
