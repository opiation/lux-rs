use crate::schema::UUID;
use async_graphql::Object;

pub struct ServerMetadata {
    /// A unique identifier for the server
    pub id: UUID,
}

#[Object]
impl ServerMetadata {
    #[graphql(skip)]
    pub fn new() -> Self {
        ServerMetadata {
            id: UUID::generate(),
        }
    }

    /// A unique identifier for the server
    pub async fn id(&self) -> &UUID {
        &self.id
    }
}
