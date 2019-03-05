use rmps;
use rocksdb::{Error, DB};
use std::{string, vec};

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Int(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColumnBaseType {
    Int(String),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColumnType {
    PrimaryKey(ColumnBaseType),
    Nullable(ColumnBaseType),
    NonNullable(ColumnBaseType),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    name: String,
    table_name: String,
    column_indexes: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    name: String,
    indexes: Vec<Index>,
    // columns are sorted by name and indexed by offset
    columns: Vec<ColumnType>,
    // primary key can be one column or combination of several columns
    primary_keys: Vec<i32>,
    // no foreign keys for now
}

impl Table {
    pub fn new() -> self
    {

    }

    pub fn schema_key(&self) -> Vec<u8> {
        rmps::to_vec(&("schema", self.name.as_str())).unwrap()
    }
    pub fn schema_value(&self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}

pub fn create_table(db: &DB, table: &Table) -> Result<(), Error> {
    db.put(
        table.schema_key().as_slice(),
        table.schema_value().as_slice(),
    )
}

//pub fn insert_into(db: &DB, table: &Table, values: &[FieldValue]) -> Result<(), Error> {}

#[cfg(test)]
mod tests {
    use super::{create_table, Table};
    use rmps;
    use rocksdb::DB;

    #[test]
    fn test_schema_key() {
        assert_eq!(Table::new().schema_key()[0], 0x92);
    }

    fn test_number_serialization_consistency() {
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u8).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u16).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u32).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u64).unwrap());
    }

    fn test_create_table() {
        //let db = DB::open_default("./test_folder/").unwrap();
        //create_table(db, User::new()).unwrap();
    }
}
