use std::collections::HashMap;
use std::fmt::{Debug};
use std::hash::Hash;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use serde_json::{Map, Value};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;
use crate::errors::DbError;

const NULL_VAL: serde_json::Value = serde_json::Value::Null;

pub struct Table {
    table_name: String,
    primary_key: String,
    data: Arc<tokio::sync::RwLock<HashMap<String, serde_json::Value>>>
}

impl Table {
    fn new(table_name: String, primary_key: String) -> Self {
        Self {
            table_name: table_name,
            primary_key: primary_key,
            data: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    fn new_load(table_name: String, primary_key: String, data: HashMap<String, serde_json::Value>) -> Self{
        Self {
            table_name: table_name,
            primary_key: primary_key,
            data: Arc::new(RwLock::new(data)),
        }
    }

    async fn create_document(&self, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        if let Err(e) = crate::validation::check_string_fields_exist(&info, &[self.primary_key.as_str()]) {
            return Err(e);
        }
        let mut guard = self.data.write().await;
        guard.deref_mut().insert(info[&(self.primary_key)].as_str().unwrap().to_string(), info);
        Ok(NULL_VAL)
    }
    async fn read_document(&self, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        if let Err(e) = crate::validation::check_string_fields_exist(&info, &[self.primary_key.as_str()]) {
            return Err(e);
        }
        let guard = self.data.read().await;
        match guard.deref().get(&(info[&(self.primary_key)].as_str().unwrap().to_string())) {
            None => {Ok(NULL_VAL)}
            Some(output) => {Ok(output.clone())}
        }

    }
    async fn update_document(&self, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {todo!()}
    async fn delete_document(&self, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        if let Err(e) = crate::validation::check_string_fields_exist(&info, &[self.primary_key.as_str()]) {
            return Err(e);
        }
        let mut guard = self.data.write().await;
        guard.deref_mut().remove(&(self.primary_key));
        Ok(NULL_VAL)
    }
}

pub struct Database {
    tables: Arc<tokio::sync::RwLock<HashMap<String, Table>>>,
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

    fn load(path: &str) -> Self {
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
        let map = serde_json::from_str(s.as_str());
        if map.is_err() {
            return Self::cold_start(path);
        }
        let map: HashMap<String, serde_json::Value> = map.unwrap();

        let mut output = HashMap::new();
        for (key, val) in map {
            let json_map = val["data"].as_object();
            match json_map {
                None => {output.insert(key, Table::new(val["table_name"].as_str().unwrap().to_string(), val["primary_key"].as_str().unwrap().to_string()));}
                Some(json_map) => {
                    let mut table_data = HashMap::new();
                    for (inner_key, inner_val) in json_map {
                        table_data.insert(inner_key.clone(), inner_val.clone());
                    }
                    output.insert(key, Table::new_load(val["table_name"].as_str().unwrap().to_string(), val["primary_key"].as_str().unwrap().to_string(), table_data));
                }
            }
        }

        Self {
            tables: Arc::new(tokio::sync::RwLock::new(output)),
            path: path.to_string(),
        }
    }

    fn cold_start(path: &str) -> Self {
        let p = std::path::Path::new(path);
        let _ = std::fs::remove_dir(p);
        let _ = std::fs::create_dir(p);
        let _ = std::fs::File::create(p.join("data.json"));
        Self {
            tables: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            path: path.to_string(),
        }
    }

    pub fn new(path: &str) -> Self {
        Self::load(path)
    }

    pub async fn read_db(&self) -> Result<serde_json::Value, crate::errors::DbError> {
        let guard = self.tables.read().await;
        let mut v = Vec::new();
        v.reserve(guard.keys().len());
        for key in guard.keys() {
            v.push(key);
        }
        Ok(serde_json::json!({
            "tables": v
        }))
    }


    pub async fn create_table(&self, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        if let Err(e) = crate::validation::check_string_fields_exist(&info, &["table_name", "primary_key"]) {
            return Err(e);
        }
        let table = Table::new(info["table_name"].as_str().unwrap().to_string(), info["primary_key"].as_str().unwrap().to_string());
        let mut guard = self.tables.write().await;
        guard.deref_mut().insert(info["table_name"].as_str().unwrap().to_string(), table);
        Ok(NULL_VAL)
    }

    pub async fn read_table(&self, table_name: &String) -> Result<serde_json::Value, crate::errors::DbError> {
        let outer_guard = self.tables.read().await;

        match outer_guard.deref().get(table_name) {
            None => {Err(crate::errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                let inner_guard = table.data.read().await;
                let mut v = Vec::new();
                v.reserve(inner_guard.values().len());
                for val in inner_guard.values() {
                    v.push(val.clone());
                }
                Ok(serde_json::json!(
                    {
                        "table_name": table.table_name,
                        "primary_key": table.primary_key,
                        "data": serde_json::Value::Array(v),
                    }
                ))
            }
        }
    }
    pub async fn update_table() {todo!();}
    pub async fn delete_table(&self, table_name: &String) ->  Result<serde_json::Value, crate::errors::DbError> {
        let mut guard = self.tables.write().await;
        guard.deref_mut().remove(table_name);
        Ok(NULL_VAL)
    }


    pub async fn create_document(&self, table_name: &String, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(crate::errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.create_document(info).await
            }
        }
    }
    pub async fn read_document(&self, table_name: &String, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(crate::errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.read_document(info).await
            }
        }
    }
    pub async fn update_document(&self, table_name: &String, info: serde_json::Value) {todo!()}
    pub async fn delete_document(&self, table_name: &String, info: serde_json::Value) -> Result<serde_json::Value, crate::errors::DbError> {
        let guard = self.tables.read().await;
        match guard.deref().get(table_name) {
            None => {Err(crate::errors::DbError::TableNotFound(String::from("Table does not exist")))}
            Some(table) => {
                table.delete_document(info).await
            }
        }
    }

    pub async fn save(&self) {
        let mut output = HashMap::new();
        let outer_guard = self.tables.read().await;
        for (table_name, table_obj) in outer_guard.deref() {
            let inner_guard = table_obj.data.read().await;
            output.insert(table_name, serde_json::json!({
                "table_name": table_name,
                "primary_key": table_obj.primary_key,
                "data": inner_guard.deref(),
            }));
        }

        let p = std::path::Path::new(self.path.as_str()).join("data.json");
        let _ = std::fs::write(p, serde_json::to_vec(&output).unwrap());
    }
}
