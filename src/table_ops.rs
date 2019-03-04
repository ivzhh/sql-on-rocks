use rmps;
use rocksdb::{Error, DB};
use std::{string, vec};

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Int(String),
}

pub trait Table {
    fn name(&self) -> &str;
    fn fields(&self) -> &Vec<Field>;

    fn schema_key(&self) -> Vec<u8> {
        rmps::to_vec(&("schema", self.name())).unwrap()
    }
    fn schema_value(&self) -> Vec<u8> {
        rmps::to_vec(&(self.fields())).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    fields: Vec<Field>,
}

impl User {
    pub fn new() -> Self {
        Self {
            name: "User".to_owned(),
            fields: vec![Field::Int("id".to_owned()), Field::Int("score".to_owned())],
        }
    }
}

impl Table for User {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

pub enum FieldValue {
    Int(i32, String),
}

pub fn create_table(db: &DB, table: &Table) -> Result<(), Error> {
    db.put(
        table.schema_key().as_slice(),
        table.schema_value().as_slice(),
    )
}

pub fn insert_into(db: &DB, table: &Table, values: &[FieldValue]) -> Result<(), Error> {}

#[cfg(test)]
mod tests {
    use super::{create_table, Table, User};
    use rmps;
    #[test]
    fn test_schema_key() {
        assert_eq!(User::new().schema_key()[0], 0x92);
    }

    fn test_number_serialization_consistency() {
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u8).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u16).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u32).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u64).unwrap());
    }

    fn test_create_table() {
        let db = DB::open_default("./test_folder/").unwrap();
        create_table(db, User::new()).unwrap();
    }
}
