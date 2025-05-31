use crate::*;

pub enum KeyDatatype {
    Null(()),
    Boolean(bool),
    SignedInt(i64),
    UnsignedInt(u64),
    // Float(f64),
    String(String),
}

impl KeyDatatype {
    pub fn to_str(&self) -> &str {
        match self {
            KeyDatatype::Null(_) => {"Null"}
            KeyDatatype::Boolean(_) => {"Boolean"}
            KeyDatatype::SignedInt(_) => {"SignedInt"}
            KeyDatatype::UnsignedInt(_) => {"UnsignedInt"}
            // KeyDatatype::Float(_) => {"Float"}
            KeyDatatype::String(_) => {"String"}
        }
    }

    pub fn to_int(&self) -> u8 {
        match self {
            KeyDatatype::Null(_) => {0}
            KeyDatatype::Boolean(_) => {1}
            KeyDatatype::SignedInt(_) => {2}
            KeyDatatype::UnsignedInt(_) => {3}
            // KeyDatatype::Float(_) => {4}
            KeyDatatype::String(_) => {5}
        }
    }

    pub fn from_int(n: u8) -> Self {
        match n {
            1 => {Self::Boolean(false)}
            2 => {Self::SignedInt(0)}
            3 => {Self::UnsignedInt(0)}
            // 4 => {Self::Float(0.)}
            5 => {Self::String(String::new())}
            _ => {Self::Null(())}
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Boolean" => {Self::Boolean(false)}
            "UnsignedInt" => {Self::UnsignedInt(0)}
            "SignedInt" => {Self::SignedInt(0)}
            // "Float" => {Self::Float(0.)}
            "String" => {Self::String(String::new())}
            &_ => {Self::Null(())}
        }
    }
}

impl Eq for KeyDatatype {}

impl PartialEq<Self> for KeyDatatype {
    fn eq(&self, other: &Self) -> bool {
        match self {
            KeyDatatype::Null(s) => {
                match other {
                    KeyDatatype::Null(o) => {s.eq(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::Boolean(s) => {
                match other {
                    KeyDatatype::Boolean(o) => {s.eq(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::SignedInt(s) => {
                match other {
                    KeyDatatype::SignedInt(o) => {s.eq(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::UnsignedInt(s) => {
                match other {
                    KeyDatatype::UnsignedInt(o) => {s.eq(o)}
                    _ => {panic!();}
                }
            }
            // KeyDatatype::Float(s) => {
            //     match other {
            //         KeyDatatype::Float(o) => {s.eq(o)}
            //         _ => {panic!();}
            //     }
            // }
            KeyDatatype::String(s) => {
                match other {
                    KeyDatatype::String(o) => {s.eq(o)}
                    _ => {panic!();}
                }
            }
        }
    }
}

impl PartialOrd<Self> for KeyDatatype {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            KeyDatatype::Null(s) => {
                match other {
                    KeyDatatype::Null(o) => {s.partial_cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::Boolean(s) => {
                match other {
                    KeyDatatype::Boolean(o) => {s.partial_cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::SignedInt(s) => {
                match other {
                    KeyDatatype::SignedInt(o) => {s.partial_cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::UnsignedInt(s) => {
                match other {
                    KeyDatatype::UnsignedInt(o) => {s.partial_cmp(o)}
                    _ => {panic!();}
                }
            }
            // KeyDatatype::Float(s) => {
            //     match other {
            //         KeyDatatype::Float(o) => {s.partial_cmp(o)}
            //         _ => {panic!();}
            //     }
            // }
            KeyDatatype::String(s) => {
                match other {
                    KeyDatatype::String(o) => {s.partial_cmp(o)}
                    _ => {panic!();}
                }
            }
        }
    }
}

impl Ord for KeyDatatype {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            KeyDatatype::Null(s) => {
                match other {
                    KeyDatatype::Null(o) => {s.cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::Boolean(s) => {
                match other {
                    KeyDatatype::Boolean(o) => {s.cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::SignedInt(s) => {
                match other {
                    KeyDatatype::SignedInt(o) => {s.cmp(o)}
                    _ => {panic!();}
                }
            }
            KeyDatatype::UnsignedInt(s) => {
                match other {
                    KeyDatatype::UnsignedInt(o) => {s.cmp(o)}
                    _ => {panic!();}
                }
            }
            // KeyDatatype::Float(s) => {
            //     match other {
            //         KeyDatatype::Float(o) => {s.cmp(o)}
            //         _ => {panic!();}
            //     }
            // }
            KeyDatatype::String(s) => {
                match other {
                    KeyDatatype::String(o) => {s.cmp(o)}
                    _ => {panic!();}
                }
            }
        }
    }
}

impl std::hash::Hash for KeyDatatype {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            KeyDatatype::Null(s) => {s.hash(state);}
            KeyDatatype::Boolean(s) => {s.hash(state);}
            KeyDatatype::SignedInt(s) => {s.hash(state);}
            KeyDatatype::UnsignedInt(s) => {s.hash(state);}
            // KeyDatatype::Float(s) => {s.hash(state);}
            KeyDatatype::String(s) => {s.hash(state);}
        }
    }
}

pub struct Key {
    pub name: String,
    pub datatype: KeyDatatype,
}


// pub trait KeyTrait: std::hash::Hash + std::cmp::Eq + std::cmp::Ord {}
// impl KeyTrait for () {}
// impl KeyTrait for bool {}
// impl KeyTrait for i64 {}
// impl KeyTrait for u64 {}
// impl KeyTrait for f64 {}
// impl KeyTrait for String {}


fn validate(key: &Key, info: &Value) -> Result<(), errors::DbError> {
    if !info.is_object() {
        return Err(errors::DbError::BadInput(String::from("Payload must be a valid JSON object!")));
    }
    match key.datatype {
        KeyDatatype::Null(_) => { Ok(()) }
        KeyDatatype::Boolean(_) => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_boolean() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::SignedInt(_) => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_i64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        KeyDatatype::UnsignedInt(_) => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_u64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
        // KeyDatatype::Float(_) => {
        //     if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_f64() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        // }
        KeyDatatype::String(_) => {
            if info.get(&key.name).is_some() && info.get(&key.name).unwrap().is_string() { Ok(()) } else { Err(errors::DbError::MissingFields(format!("Field {} must exist and be of type {}.", &key.name, key.datatype.to_str()))) }
        }
    }
}


pub fn extract_key(key: &Key, info: &Value) -> Result<KeyDatatype, errors::DbError> {
    if let Err(e) = validate(key, info) {
        return Err(e);
    }
    Ok(match key.datatype {
        KeyDatatype::Null(_) => {KeyDatatype::Null(())}
        KeyDatatype::Boolean(_) => { KeyDatatype::Boolean(info.get(&key.name).unwrap().as_bool().unwrap()) }
        KeyDatatype::SignedInt(_) => { KeyDatatype::SignedInt(info.get(&key.name).unwrap().as_i64().unwrap()) }
        KeyDatatype::UnsignedInt(_) => {KeyDatatype::UnsignedInt(info.get(&key.name).unwrap().as_u64().unwrap())}
        // KeyDatatype::Float(_) => {KeyDatatype::Float(info.get(&key.name).unwrap().as_f64().unwrap())}
        KeyDatatype::String(_) => {KeyDatatype::String(String::from(info.get(&key.name).unwrap().as_str().unwrap()))}
    })
}
