mod account;
mod server_metadata;
mod uuid;

use std::collections::HashMap;

pub use account::Account;
use async_graphql::{Context, Object};
pub use server_metadata::ServerMetadata;
use std::sync::RwLock;
pub use uuid::UUID;

pub struct Query;

#[Object]
impl Query {
    async fn account(&self) -> Account {
        Account::with_transactions(vec![12.7, 0.13, 0.80, 9.12])
    }

    async fn accounts(&self, ctx: &Context<'_>) -> Vec<Account> {
        ctx.data::<RwLock<HashMap<UUID, Account>>>()
            .unwrap()
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    async fn generate_uuid(&self) -> UUID {
        UUID::generate()
    }

    async fn hello(&self) -> &'static str {
        "Hello, world!"
    }

    async fn server(&self) -> ServerMetadata {
        ServerMetadata::new()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_an_account(&self, ctx: &Context<'_>) -> Account {
        let new_account = Account::with_transactions(vec![]);
        let mut accounting_repository = ctx
            .data::<RwLock<HashMap<UUID, Account>>>()
            .unwrap()
            .write()
            .unwrap();
        accounting_repository.insert(new_account.id.into(), new_account.clone());
        new_account
    }

    /// Delete accounts matcing the given `ids` and return a list of the deleted
    /// accounts.
    ///
    /// Note that the returned list of deleted accounts may not match the list
    /// of `ids` provided as only accounts that were actually deleted by this
    /// mutation will be included in the result. If the data source for accounts
    /// does not have an account for a given ID, it is ignored and therefore not
    /// included in the results.
    async fn delete_accounts_by_id(&self, ctx: &Context<'_>, ids: Vec<UUID>) -> Vec<Account> {
        let mut accounting_repository = ctx
            .data::<RwLock<HashMap<UUID, Account>>>()
            .unwrap()
            .write()
            .unwrap();
        ids.iter()
            .filter_map(|id| accounting_repository.remove(&id))
            .collect()
    }
}
