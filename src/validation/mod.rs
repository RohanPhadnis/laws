use crate::*;

pub fn check_string_fields_exist(value: &Value, fields: &[&str]) -> Result<(), errors::DbError> {
    for field in fields {
        if value.get(field).is_none() {
            return Err(errors::DbError::MissingFields(format!("{} field is missing or null", field)));
        } else if !value[field].is_string() {
            return Err(errors::DbError::BadInput(format!("{} field must be a string", field)))
        }
    }
    Ok(())
}

pub fn check_key_fields_exist(value: &Value, fields: &[&str]) -> Result<(), errors::DbError> {
    for field in fields {
        if value.get(field).is_none() {
            return Err(errors::DbError::MissingFields(format!("{} field is missing or null", field)));
        }
        let Some(key_obj) = value.get(field);
        if !key_obj.is_object() {
            return Err(errors::DbError::BadInput(format!("{} must be a valid JSON object", field)))
        }
        check_key_fields_exist(key_obj, &["name", "datatype"])?
    }
    Ok(())
}