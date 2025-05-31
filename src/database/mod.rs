use crate::{*};

pub struct Database {
    tables: Arc<RwLock<HashMap<&String, table::Table >>>,
    path: String,
}

impl Clone for Database{
    fn clone(&self) -> Self {
        Self {
            tables: self.tables.clone(),
            path: self.path.clone(),
        }
    }
}

impl Database {

    async fn load(path: &str) -> Self {
        let p = std::path::Path::new(path);

        // directory existence check
        match std::fs::exists(p) {
            Ok(b) => {if !b {return Self::cold_start(path);}}
            Err(_) => {return Self::cold_start(path);}
        }

        // file existence check
        match std::fs::exists(p.join("data.json")) {
            Ok(b) => {if !b {return Self::cold_start(path);}}
            Err(_) => {return Self::cold_start(path);}
        }

        // file opening check
        let file = std::fs::File::open(p.join("data.json"));
        if file.is_err() {
            return Self::cold_start(path);
        }
        let mut file = file.unwrap();

        // file read
        let mut s = String::new();
        if file.read_to_string(&mut s).is_err() {
            return Self::cold_start(path);
        }

        // JSON-ify check
        let tables = serde_json::from_str(s.as_str());
        if tables.is_err() {
            return Self::cold_start(path);
        }
        let Value::Array(tables) = tables.unwrap();

        let mut output = HashMap::new();
        for table in tables {
            let table = table::Table::new(&table).await;
            if table.is_err() {
                return Self::cold_start(path);
            }
            let table = table.unwrap();
            output.insert(&table.table_name, table);
        }

        Self {
            tables: Arc::new(RwLock::new(output)),
            path: path.to_string(),
        }
    }

    fn cold_start(path: &str) -> Self {
        let p = std::path::Path::new(path);
        let _ = std::fs::remove_dir(p);
        let _ = std::fs::create_dir(p);
        let _ = std::fs::File::create(p.join("data.json"));
        Self {
            tables: Arc::new(RwLock::new(HashMap::new())),
            path: path.to_string(),
        }
    }

    pub async fn new(path: &str) -> Self {
        Self::load(path).await
    }

    pub async fn read_db(&self) -> Result<Value, errors::DbError> {
        let guard = self.tables.read().await;
        let mut v = Vec::new();
        v.reserve(guard.keys().len());
        for db_keys in guard.keys() {
            v.push(db_keys);
        }
        Ok(serde_json::json!({
            "tables": v
        }))
    }


    pub async fn create_table(&self, info: Value) -> Result<Value, errors::DbError> {
        let table = table::Table::new(&info).await?;
        let mut guard = self.tables.write().await;
        guard.deref_mut().insert(&table.table_name, table);
        Ok(NULL_VAL)
    }

    pub async fn read_table(&self, table_name: &String) -> Result<Value, errors::DbError> {
        let outer_guard = self.tables.read().await;

        match outer_guard.deref().get(table_name) {
            None => {Err(errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.read_table().await
            }
        }
    }

    pub async fn update_table() {todo!();}

    pub async fn delete_table(&self, table_name: &String) ->  Result<Value, errors::DbError> {
        let mut guard = self.tables.write().await;
        guard.deref_mut().remove(table_name);
        Ok(NULL_VAL)
    }


    pub async fn create_document(&self, table_name: &String, info: Value) -> Result<Value, errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.create_document(info).await
            }
        }
    }
    pub async fn read_document(&self, table_name: &String, info: Value) -> Result<Value, errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.read_document(info).await
            }
        }
    }
    pub async fn update_document(&self, table_name: &String, info: Value) -> Result<Value, errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.update_document(info).await
            }
        }
    }
    pub async fn delete_document(&self, table_name: &String, info: Value) -> Result<Value, errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.delete_document(info).await
            }
        }
    }

    pub async fn save(&self) {
        let mut output = Vec::new();
        let outer_guard = self.tables.read().await;
        for (_, table_obj) in outer_guard.deref() {
            let table_data = table_obj.read_table().await.unwrap();
            output.push(table_data);
        }
        let p = std::path::Path::new(self.path.as_str()).join("data.json");
        let _ = std::fs::write(p, serde_json::to_vec(&output).unwrap());
    }
}
