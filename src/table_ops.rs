use rmps;
use rocksdb::{Error, DB};
use std::boxed::Box;
use std::collections::HashMap;
use std::option::Option;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColumnDataType {
    Int(String),
    Text(String),
}

impl ColumnDataType {
    pub fn get_name(&self) -> &String {
        match self {
            ColumnDataType::Int(name) => name,
            ColumnDataType::Text(name) => name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColumnType {
    PrimaryKey(ColumnDataType),
    Nullable(ColumnDataType),
    NonNullable(ColumnDataType),
}

impl ColumnType {
    pub fn get_name(&self) -> &String {
        match self {
            ColumnType::PrimaryKey(data) => data.get_name(),
            ColumnType::Nullable(data) => data.get_name(),
            ColumnType::NonNullable(data) => data.get_name(),
        }
    }

    pub fn get_inner(&self) -> ColumnDataType {
        match self {
            ColumnType::PrimaryKey(data) => data.clone(),
            ColumnType::Nullable(data) => data.clone(),
            ColumnType::NonNullable(data) => data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableBuilder {
    name: String,
    columns: Vec<ColumnType>,
}

impl TableBuilder {
    pub fn new(name: String) -> Self {
        TableBuilder {
            name,
            columns: Vec::new(),
        }
    }

    pub fn build_table(self, db: &DB) -> Table {
        let mut column_map: HashMap<String, i32> = HashMap::new();

        let mut i = 0i32;

        let columns = self.columns;

        for col in columns.as_slice() {
            column_map.insert(col.get_name().to_owned(), i);
            i += 1;
        }

        let table = Table {
            name: self.name,
            column_map,
            columns,
        };

        create_table(db, &table).unwrap();

        table
    }
    pub fn set_columns(mut self, columns: Vec<ColumnType>) -> Self {
        self.columns.extend_from_slice(columns.as_slice());
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    name: String,
    column_map: HashMap<String, i32>,
    columns: Vec<ColumnType>,
}

/// table name :: column name :: primary key value
pub const COLUMN_KEY_NUM_COMPONENTS: u32 = 3;

pub struct InsertStatement<'a> {
    table: &'a Table,
    i: i32,
    columns: Vec<Vec<u8>>,
}

pub trait VecSerializer {
    fn write_msgpack_in_vec(&self, buf: &mut Vec<u8>) -> Option<()>;
    fn check_declared_type(&self, declared_type: &ColumnType) -> bool;
}

impl VecSerializer for i64 {
    fn write_msgpack_in_vec(&self, buf: &mut Vec<u8>) -> Option<()> {
        match rmp::encode::write_sint(buf, *self) {
            Err(_) => None,
            _ => Some(()),
        }
    }

    fn check_declared_type(&self, declared_type: &ColumnType) -> bool {
        match declared_type.get_inner() {
            ColumnDataType::Int(_) => true,
            _ => false,
        }
    }
}

pub trait InsertError {}

impl InsertError for rocksdb::Error {}

#[derive(Debug)]
pub struct ConsistencyError {}

impl InsertError for ConsistencyError {}

impl InsertError for std::io::Error {}

impl std::fmt::Debug for std::boxed::Box<dyn InsertError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "write error")
    }
}

impl<'a> InsertStatement<'a> {
    pub fn set_kv<T: VecSerializer>(&mut self, key: &String, value: T) -> Option<&mut Self> {
        self.i = -1i32;
        let i: usize = *self.table.column_map.get(key)? as usize;
        if let Some(buf) = self.columns.get_mut(i) {
            value.write_msgpack_in_vec(buf)?;
            return Some(self);
        }

        None
    }

    pub fn set_next<T: VecSerializer>(&mut self, value: T) -> Option<&mut Self> {
        let i: usize = self.i as usize;
        self.i = self.i + 1;
        if let Some(buf) = self.columns.get_mut(i) {
            value.write_msgpack_in_vec(buf)?;
            return Some(self);
        }

        None
    }

    pub fn execute(&mut self, db: &DB) -> Result<(), Box<InsertError>> {
        let mut primary_keys: Vec<usize> = Vec::new();

        {
            // count primary key number
            // nullable columns will be filled with msgpack nil;
            // other columns must be filled by user
            let mut i = 0;
            for v in self.columns.iter_mut() {
                if v.len() == 0 {
                    match self.table.columns[i] {
                        ColumnType::Nullable(_) => match rmp::encode::write_nil(v) {
                            Ok(_) => {}
                            Err(error_from_rocks) => return Err(Box::new(error_from_rocks)),
                        },
                        _ => return Err(Box::new(ConsistencyError {})),
                    };
                }

                match self.table.columns[i] {
                    ColumnType::PrimaryKey(_) => primary_keys.push(i),
                    _ => {}
                };

                i += 1;
            }
        }

        if primary_keys.len() == 0 {
            return Err(Box::new(ConsistencyError {}));
        }

        let mut pkey_component = Vec::<u8>::new();

        rmp::encode::write_array_len(&mut pkey_component, primary_keys.len() as u32).unwrap();

        for pk in primary_keys.iter() {
            pkey_component.extend_from_slice(self.columns[*pk].as_slice());
        }

        {
            // fill in fields into storage engine
            let mut i = 0;
            for v in self.columns.iter_mut() {
                let column_name = self.table.columns[i].get_name();

                let mut key = Vec::<u8>::new();

                rmp::encode::write_array_len(&mut key, COLUMN_KEY_NUM_COMPONENTS).unwrap();

                rmp::encode::write_str(&mut key, self.table.name.as_str()).unwrap();
                rmp::encode::write_str(&mut key, column_name.as_str()).unwrap();
                key.extend_from_slice(pkey_component.as_slice());

                match db.put(key.as_slice(), v.as_slice()) {
                    Ok(_) => {}
                    Err(e) => return Err(Box::new(e)),
                }

                i += 1;
            }
        }

        return Ok(());
    }
}

impl Table {
    pub fn table_key(&self) -> Vec<u8> {
        rmps::to_vec(&(".table", self.name.as_str())).unwrap()
    }
    pub fn table_value(&self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }

    pub fn insert_into(&self) -> InsertStatement {
        let mut columns: Vec<Vec<u8>> = Vec::new();
        columns.resize_with(self.columns.len(), Vec::new);
        InsertStatement {
            table: &self,
            i: 0i32,
            columns,
        }
    }
}

pub fn create_table(db: &DB, table: &Table) -> Result<(), Error> {
    db.put(table.table_key().as_slice(), table.table_value().as_slice())
}

#[cfg(test)]
mod tests {
    use super::{Table, TableBuilder, COLUMN_KEY_NUM_COMPONENTS};
    use rmps;
    use rocksdb::DB;
    use std::borrow::Borrow;
    use tempdir::TempDir;

    use super::{ColumnDataType, ColumnType};
    use std::collections::HashMap;

    #[test]
    fn test_msgpack_nested_array() {
        let output = rmps::to_vec(&(1, 2, (3, 4), 5)).unwrap();
        // [0]: len 4
        // [1]: 1
        // [2]: 2
        // [3]: len 2
        assert_eq!(output[0], 0x94);
        assert_eq!(output[3], 0x92);
    }

    #[test]
    fn test_schema_key() {
        assert_eq!(
            Table {
                name: "User".to_owned(),
                column_map: HashMap::new(),
                columns: Vec::new(),
            }
            .table_key()[0],
            0x92
        );
    }

    #[test]
    fn test_number_serialization_consistency() {
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u8).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u16).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u32).unwrap());
        assert_eq!(rmps::to_vec(&42).unwrap(), rmps::to_vec(&42u64).unwrap());
    }

    #[test]
    fn test_create_table_empty() {
        if let Ok(tmp_dir) = TempDir::new("") {
            let db = DB::open_default(tmp_dir.path().to_str().unwrap()).unwrap();
            let table = TableBuilder::new("User".to_owned()).build_table(&db);

            let reply = db.get(table.table_key().as_slice()).unwrap().unwrap();

            assert_eq!(reply.to_vec(), table.table_value());
        }
    }

    #[test]
    fn test_create_table_ints() {
        if let Ok(tmp_dir) = TempDir::new("") {
            let db: DB = DB::open_default(tmp_dir.path().to_str().unwrap()).unwrap();
            let table = TableBuilder::new("User".to_owned())
                .set_columns(vec![ColumnType::PrimaryKey(ColumnDataType::Int(
                    "id".to_owned(),
                ))])
                .build_table(&db);

            table
                .insert_into()
                .set_kv(&"id".to_owned(), 1i64)
                .unwrap()
                .execute(&db)
                .unwrap();

            {
                let mut key_prefix = Vec::<u8>::new();
                rmp::encode::write_array_len(&mut key_prefix, COLUMN_KEY_NUM_COMPONENTS).unwrap();

                rmp::encode::write_str(&mut key_prefix, "User").unwrap();
                rmp::encode::write_str(&mut key_prefix, "id").unwrap();

                let iter = db.prefix_iterator(key_prefix.as_slice());

                for (key, value) in iter {
                    let decoded_key: (String, String, (i64,)) =
                        rmps::decode::from_slice(key.borrow()).unwrap();
                    let decoded_value: i64 = rmps::decode::from_slice(value.borrow()).unwrap();

                    println!("Saw {:?} {:?}", decoded_key, decoded_value);
                }
            }

            table
                .insert_into()
                .set_next(1i64)
                .unwrap()
                .execute(&db)
                .unwrap();

            table
                .insert_into()
                .execute(&db)
                .expect_err("the column is not filled");

            let reply = db.get(table.table_key().as_slice()).unwrap().unwrap();

            assert_eq!(reply.to_vec(), table.table_value());
        }
    }
}
