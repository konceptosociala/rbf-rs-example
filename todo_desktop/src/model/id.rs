use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Record {
    pub id: Id,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "tb")]
    table: String,
    #[serde(rename = "id")]
    inner: InnerId,
}

impl Id {
    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn inner(&self) -> &str {
        &self.inner.id
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct InnerId {
    #[serde(rename = "String")]
    id: String,
}