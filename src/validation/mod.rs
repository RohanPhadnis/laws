pub fn check_string_fields_exist(value: &serde_json::Value, fields: &[&str]) -> Result<(), crate::errors::DbError> {
    for field in fields {
        if value[field].is_null() {
            return Err(crate::errors::DbError::MissingFields(format!("{} field is missing or null", field)));
        } else if !value[field].is_string() {
            return Err(crate::errors::DbError::BadInput(format!("{} field must be a string", field)))
        }
    }
    Ok(())
}