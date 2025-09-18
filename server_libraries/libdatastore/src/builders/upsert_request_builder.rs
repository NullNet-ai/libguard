use crate::{Params, Query, UpsertBody, UpsertRequest};

#[derive(Debug, Default)]
pub struct UpsertRequestBuilder {
    id: Option<String>,
    table: Option<String>,
    pluck: Option<String>,
    durability: Option<String>,
    data: Option<String>,
    conflict_columns: Vec<String>,
    is_root: bool,
}

impl UpsertRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn table(mut self, table: impl Into<String>) -> Self {
        self.table = Some(table.into());
        self
    }

    pub fn query(mut self, pluck: impl Into<String>, durability: impl Into<String>) -> Self {
        self.pluck = Some(pluck.into());
        self.durability = Some(durability.into());
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn conflict_column(mut self, column: impl Into<String>) -> Self {
        self.conflict_columns.push(column.into());
        self
    }

    pub fn conflict_columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.conflict_columns
            .extend(columns.into_iter().map(|s| s.into()));
        self
    }

    pub fn performed_by_root(mut self, value: bool) -> Self {
        self.is_root = value;
        self
    }

    pub fn build(self) -> UpsertRequest {
        UpsertRequest {
            params: Some(Params {
                id: self.id.unwrap_or_default(),
                table: self.table.unwrap_or_default(),
                r#type: if self.is_root {
                    String::from("root")
                } else {
                    String::new()
                },
            }),
            query: Some(Query {
                pluck: self.pluck.unwrap_or_default(),
                durability: self.durability.unwrap_or_default(),
            }),
            body: Some(UpsertBody {
                data: self.data.unwrap_or_default(),
                conflict_columns: self.conflict_columns,
            }),
        }
    }
}
