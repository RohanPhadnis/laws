use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::RwLock;

const NULL_VAL: serde_json::Value = serde_json::Value::Null;

pub mod database;
pub mod errors;
mod validation;
mod table;
mod db_keys;
