use crate::schema::UUID;
use async_graphql::Object;

#[derive(Clone, Debug)]
pub struct Account {
    pub id: uuid::Uuid,
    transactions: Vec<f64>,
}

#[Object]
impl Account {
    #[graphql(skip)]
    pub fn with_transactions(ts: Vec<f64>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            transactions: ts,
        }
    }

    pub async fn id(&self) -> UUID {
        UUID::from(self.id)
    }

    pub async fn balance(&self) -> f64 {
        self.transactions
            .iter()
            .fold(0.0, |balance, txn| balance + txn)
    }
}
