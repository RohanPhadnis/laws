use crate::*;

pub struct Table {
    pub table_name: String,
    primary_key: db_keys::Key,
    sort_key: db_keys::Key,
    data: Arc<RwLock<HashMap<db_keys::KeyDatatype, Arc<RwLock<BTreeMap<db_keys::KeyDatatype, Value>>>>>>,
}

impl Table {

    pub async fn new(info: &Value) -> Result<Self, errors::DbError> {
        validation::check_string_fields_exist(info, &["table_name"])?;
        validation::check_key_fields_exist(info, &["primary_key", "sort_key"])?;
        let mut output = Self {
            table_name: info.get("table_name").unwrap().as_str().unwrap().to_string(),
            primary_key: db_keys::Key {
                name: info.get("primary_key").unwrap().get("name").unwrap().as_str().unwrap().to_string(),
                datatype: db_keys::KeyDatatype::from_str(info.get("primary_key").unwrap().get("name").unwrap().as_str().unwrap()),
            },
            sort_key: db_keys::Key {
                name: info.get("sort_key").unwrap().get("name").unwrap().as_str().unwrap().to_string(),
                datatype: db_keys::KeyDatatype::from_str(info.get("sort_key").unwrap().get("name").unwrap().as_str().unwrap()),
            },
            data: Arc::new(RwLock::new(HashMap::new())),
        };
        if let Ok(data) = output.load(info).await {
            output.data = Arc::new(RwLock::new(data));
        }
        Ok(output)
    }

    pub async fn load(&self, info: &Value) -> Result<HashMap<db_keys::KeyDatatype, Arc<RwLock<BTreeMap<db_keys::KeyDatatype, Value>>>>, errors::DbError> {
        if info.get("data").is_none() {
            return Err(errors::DbError::MissingFields(String::from("Must contain field data")));
        }
        let data = info.get("data").unwrap();
        if !data.is_array() {
            return Err(errors::DbError::BadInput(String::from("data must be valid JSON array")));
        }
        let data = data.as_array().unwrap();
        let mut output: HashMap<db_keys::KeyDatatype, Arc<RwLock<BTreeMap<db_keys::KeyDatatype, Value>>>> = HashMap::new();
        for document in data {
            let pk = db_keys::extract_key(&self.primary_key, document)?;
            let sk = db_keys::extract_key(&self.sort_key, document)?;
            let mut inner_guard = output.entry(pk).or_insert(Arc::new(RwLock::new(BTreeMap::new()))).write().await;
            inner_guard.deref_mut().insert(sk, document.clone());
        }
        Ok(output)
    }

    pub async fn read_table(&self) -> Result<Value, errors::DbError> {
        let mut output = Vec::new();
        let outer_guard = self.data.read().await;
        for (_, val) in outer_guard.deref() {
            let inner_guard = val.read().await;
            for (_, val) in inner_guard.deref() {
                output.push(val.clone());
            }
        }
        Ok(serde_json::json!({
            "table_name": self.table_name,
            "primary_key": serde_json::json!({
                "name": self.primary_key.name,
                "datatype": self.primary_key.datatype.to_str(),
            }),
            "sort_key": serde_json::json!({
                "name": self.sort_key.name,
                "datatype": self.sort_key.datatype.to_str(),
            }),
            "data": Value::Array(output),
        }))
    }

    pub async fn create_document(&self, info: Value) -> Result<Value, errors::DbError> {
        let pk; let sk;
        match db_keys::extract_key(&self.primary_key, &info) {
            Ok(x) => {pk = x;}
            Err(e) => {return Err(e);}
        }
        match db_keys::extract_key(&self.sort_key, &info) {
            Ok(x) => {sk = x;}
            Err(e) => {return Err(e);}
        }
        let mut outer_guard = self.data.write().await;
        let mut inner_guard = outer_guard.deref_mut().entry(pk).or_insert(Arc::new(RwLock::new(BTreeMap::new()))).write().await;
        inner_guard.deref_mut().insert(sk, info);
        Ok(NULL_VAL)
    }

    pub async fn read_document(&self, info: Value) -> Result<Value, errors::DbError> {
        let pk; let sk;
        match db_keys::extract_key(&self.primary_key, &info) {
            Ok(x) => {pk = x;}
            Err(e) => {return Err(e);}
        }
        match db_keys::extract_key(&self.sort_key, &info) {
            Ok(x) => {sk = x;}
            Err(e) => {return Err(e);}
        }
        let outer_guard = self.data.read().await;
        let inner_guard = outer_guard.deref().get(&pk);
        if inner_guard.is_none() {return Ok(NULL_VAL);}
        let inner_guard = inner_guard.unwrap().read().await;
        let output = inner_guard.get(&sk);
        if output.is_none() {
            Ok(NULL_VAL)
        } else {
            Ok(output.unwrap().clone())
        }
    }
    pub async fn update_document(&self, info: Value) -> Result<Value, errors::DbError> {
        let pk; let sk;
        match db_keys::extract_key(&self.primary_key, &info) {
            Ok(x) => {pk = x;}
            Err(e) => {return Err(e);}
        }
        match db_keys::extract_key(&self.sort_key, &info) {
            Ok(x) => {sk = x;}
            Err(e) => {return Err(e);}
        }
        let outer_guard = self.data.read().await;
        let inner_guard = outer_guard.deref().get(&pk);
        if inner_guard.is_none() {return Ok(NULL_VAL);}
        let mut inner_guard = inner_guard.unwrap().write().await;
        let output = inner_guard.get_mut(&sk).unwrap().as_object_mut().unwrap();
        for (key, val) in info.as_object().unwrap() {
            output.insert(key.clone(), val.clone());
        }
        Ok(NULL_VAL)
    }
    pub async fn delete_document(&self, info: Value) -> Result<Value, errors::DbError> {
        let pk; let sk;
        match db_keys::extract_key(&self.primary_key, &info) {
            Ok(x) => {pk = x;}
            Err(e) => {return Err(e);}
        }
        match db_keys::extract_key(&self.sort_key, &info) {
            Ok(x) => {sk = x;}
            Err(e) => {return Err(e);}
        }
        let outer_guard = self.data.read().await;
        let inner_guard = outer_guard.deref().get(&pk);
        if inner_guard.is_none() {return Ok(NULL_VAL);}
        let mut inner_guard = inner_guard.unwrap().write().await;
        inner_guard.deref_mut().remove(&sk);
        Ok(NULL_VAL)
    }
}
