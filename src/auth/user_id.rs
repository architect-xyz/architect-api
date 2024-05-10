use crate::uuid_val;
use uuid::{uuid, Uuid};

static USER_ID_NS: Uuid = uuid!("5b7ef607-1ecb-4231-9630-a0b005a4393b");
uuid_val!(UserId, USER_ID_NS);

impl UserId {
    pub fn anonymous() -> UserId {
        Self(Uuid::nil())
    }

    pub fn is_anonymous(&self) -> bool {
        self.0.is_nil()
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for UserId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}
