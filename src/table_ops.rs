use rmps;
use rocksdb::{Error, DB};
use std::cmp::Ordering;
use std::{string, vec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColumnDataType {
    Int,
    Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Nullity {
    PrimaryKey,
    Nullable,
    NonNullable,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnDefinition {
    name: String,
    data_type: ColumnDataType,
    nullity: Nullity,
}

impl ColumnDefinition {
    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.nullity {
            Nullity::PrimaryKey => match other.nullity {
                Nullity::PrimaryKey => self.name.cmp(&other.name),
                _ => Ordering::Greater,
            },
            _ => match other.nullity {
                Nullity::PrimaryKey => Ordering::Less,
                _ => self.name.cmp(&other.name),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    name: String,
    table_name: String,
    column_indexes: Vec<i32>,
}

pub enum IndexType {
    Asc,
    Desc,
    Unique
}

impl Index {
    pub fn index_key(&self, table: &Table) -> Vec<u8> {
        rmps::to_vec(&(".index", table.name.as_str(), self.name.as_str())).unwrap()
    }
    pub fn index_value(&self, table: Table) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    name: String,
    indexes: Vec<Index>,
    // columns are sorted by name and indexed by offset
    columns: Vec<ColumnDefinition>,
    // primary key can be one column or combination of several columns
    primary_keys: Vec<i32>,
    // no foreign keys for now
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            indexes: Vec::new(),
            columns: Vec::new(),
            primary_keys: Vec::new(),
        }
    }

    pub fn build_table(&mut self, db: &DB) -> &Self {
        self.execute(db);
        self
    }

    pub fn table_key(&self) -> Vec<u8> {
        rmps::to_vec(&(".table", self.name.as_str())).unwrap()
    }
    pub fn table_value(&self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }

    pub fn set_columns(&mut self, columns: &[ColumnDefinition]) -> &Self {
        self.columns.extend_from_slice(&columns);
        self
    }

    pub fn set_index(&mut self) -> &Self {

    }
}

impl Table {
    fn execute(&mut self, db: &DB) -> () {
        self.columns.sort_by(|a, b| a.name.cmp(&b.name));

        create_table(db, self).unwrap();
    }
}

pub fn create_table(db: &DB, table: &Table) -> Result<(), Error> {
    db.put(table.table_key().as_slice(), table.table_value().as_slice())
}

//pub fn insert_into(db: &DB, table: &Table, values: &[FieldValue]) -> Result<(), Error> {}

#[cfg(test)]
mod tests {
    use super::{create_table, Table};
    use rmps;
    use rocksdb::DB;

    #[test]
    fn test_schema_key() {
        assert_eq!(Table::new("User".to_owned()).table_key()[0], 0x92);
    }

    #[test]
    fn test_number_serialization_consistency() {
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u8).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u16).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u32).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u64).unwrap());
    }

    #[test]
    fn test_create_table() {
        let db = DB::open_default("./test_folder/").unwrap();
        Table::new("User".to_owned()).build_table(&db);
    }
}
