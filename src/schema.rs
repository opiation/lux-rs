use async_graphql::{Object, Scalar, ScalarType};

///
/// A [universally unique identifier][rfc] v4
///
/// [rfc]: https://www.rfc-editor.org/rfc/rfc4122
///
pub struct UUID(String);

impl UUID {
    fn generate() -> UUID {
        UUID::from(uuid::Uuid::new_v4())
    }
}

impl TryFrom<&String> for UUID {
    type Error = uuid::Error;

    fn try_from(a_string: &String) -> Result<Self, uuid::Error> {
        uuid::Uuid::try_parse(a_string.as_str()).map(|valid_uuid| UUID(valid_uuid.to_string()))
    }
}

impl From<uuid::Uuid> for UUID {
    fn from(a_uuid: uuid::Uuid) -> Self {
        UUID(a_uuid.to_string())
    }
}

///
/// A [universally unique identifier][rfc] v4  
///
///   e.g.: `76600240-5522-44c0-8c4c-18fbb0b6058d`
///
/// [rfc]: https://www.rfc-editor.org/rfc/rfc4122
///
#[Scalar]
impl ScalarType for UUID {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            Ok(value.parse().map(UUID)?)
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_string())
    }
}

pub struct Account {
    id: uuid::Uuid,
    transactions: Vec<f64>,
}

impl TryInto<UUID> for String {
    type Error = uuid::Error;

    fn try_into(self) -> Result<UUID, uuid::Error> {
        uuid::Uuid::try_parse(self.as_str()).map(UUID::from)
    }
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

pub struct Query;

#[Object]
impl Query {
    async fn account(&self) -> Account {
        Account::with_transactions(vec![12.7, 0.13, 0.80, 9.12])
    }

    async fn generate_uuid(&self) -> UUID {
        UUID::generate()
    }

    async fn hello(&self) -> &'static str {
        "Hello, world!"
    }
}
