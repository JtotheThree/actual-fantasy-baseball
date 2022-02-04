use async_graphql::*;


#[derive(Default, SimpleObject)]
pub struct MetaSelect {
    pub values: Vec::<String>,
    pub labels: Vec::<String>,
}
