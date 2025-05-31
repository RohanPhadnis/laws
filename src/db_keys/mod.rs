use crate::*;

pub enum KeyDatatype {
    Null,
    Boolean,
    SignedInt,
    UnsignedInt,
    Float,
    String,
}

impl KeyDatatype {
    pub fn to_str(&self) -> &str {
        match self {
            KeyDatatype::Null => {"Null"}
            KeyDatatype::Boolean => {"Boolean"}
            KeyDatatype::SignedInt => {"SignedInt"}
            KeyDatatype::UnsignedInt => {"UnsignedInt"}
            KeyDatatype::Float => {"Float"}
            KeyDatatype::String => {"String"}
        }
    }

    pub fn to_int(&self) -> u8 {
        match self {
            KeyDatatype::Null => {0}
            KeyDatatype::Boolean => {1}
            KeyDatatype::SignedInt => {2}
            KeyDatatype::UnsignedInt => {3}
            KeyDatatype::Float => {4}
            KeyDatatype::String => {5}
        }
    }

    pub fn from_int(n: u8) -> Self {
        match n {
            1 => {Self::Boolean}
            2 => {Self::SignedInt}
            3 => {Self::UnsignedInt}
            4 => {Self::Float}
            5 => {Self::String}
            _ => {Self::Null}
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Boolean" => {Self::Boolean}
            "UnsignedInt" => {Self::UnsignedInt}
            "SignedInt" => {Self::SignedInt}
            "Float" => {Self::Float}
            "String" => {Self::String}
            &_ => {Self::Null}
        }
    }
}

pub struct Key {
    pub name: String,
    pub datatype: KeyDatatype,
}


pub trait KeyTrait {}
impl KeyTrait for () {}
impl KeyTrait for bool {}
impl KeyTrait for i64 {}
impl KeyTrait for u64 {}
impl KeyTrait for f64 {}
impl KeyTrait for String {}


fn validate(key: &Key, info: &Value) -> Result<(), errors::DbError> {
    if !info.is_object() {
        return Err(errors::DbError::BadInput(String::from("Payload must be a valid JSON object!")));
    }
    match key.datatype {
        KeyDatatype::Null => { Ok(()) }
        KeyDatatype::Boolean => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_boolean() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::SignedInt => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_i64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::UnsignedInt => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_u64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::Float => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_f64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::String => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_string() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
    }
}


pub fn extract_key(key: &Key, info: &Value) -> Result<Box<dyn KeyTrait>, errors::DbError> {
    if let Err(e) = validate(key, info) {
        return Err(e);
    }
    Ok(match key.datatype {
        KeyDatatype::Null => {Box::new(())}
        KeyDatatype::Boolean => { Box::new(info.get(&key.name).unwrap().as_bool().unwrap()) }
        KeyDatatype::SignedInt => { Box::new(info.get(&key.name).unwrap().as_i64().unwrap()) }
        KeyDatatype::UnsignedInt => {Box::new(info.get(&key.name).unwrap().as_u64().unwrap())}
        KeyDatatype::Float => {Box::new(info.get(&key.name).unwrap().as_f64().unwrap())}
        KeyDatatype::String => {Box::new(String::from(info.get(&key.name).unwrap().as_str().unwrap()))}
    })
}

