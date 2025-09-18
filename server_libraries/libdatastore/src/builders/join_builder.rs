use crate::datastore::{AdvanceFilter, EntityFieldFrom, EntityFieldTo, FieldRelation, Join};

#[derive(Debug, Default)]
pub struct JoinBuilder {
    r#type: String,
    to_entity: String,
    to_field: String,
    to_alias: String,
    to_limit: i32,
    to_order_by: String,
    to_filters: Vec<AdvanceFilter>,
    from_entity: String,
    from_field: String,
}

impl JoinBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_entity(mut self, value: impl Into<String>) -> Self {
        self.to_entity = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_field(mut self, value: impl Into<String>) -> Self {
        self.to_field = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_alias(mut self, value: impl Into<String>) -> Self {
        self.to_alias = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_limit(mut self, value: i32) -> Self {
        self.to_limit = value;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_order_by(mut self, value: impl Into<String>) -> Self {
        self.to_order_by = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_filter(mut self, filter: AdvanceFilter) -> Self {
        self.to_filters.push(filter);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_filters(mut self, filters: impl IntoIterator<Item = AdvanceFilter>) -> Self {
        self.to_filters.extend(filters);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_entity(mut self, value: impl Into<String>) -> Self {
        self.from_entity = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_field(mut self, value: impl Into<String>) -> Self {
        self.from_field = value.into();
        self
    }

    pub fn build(self) -> Join {
        Join {
            r#type: self.r#type,
            field_relation: Some(FieldRelation {
                to: Some(EntityFieldTo {
                    entity: self.to_entity,
                    field: self.to_field,
                    alias: self.to_alias,
                    limit: self.to_limit,
                    order_by: self.to_order_by,
                    filters: self.to_filters,
                }),
                from: Some(EntityFieldFrom {
                    entity: self.from_entity,
                    field: self.from_field,
                }),
            }),
        }
    }
}
