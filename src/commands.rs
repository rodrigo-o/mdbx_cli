use libmdbx::{Database, WriteFlags, WriteMap};

pub fn list(env: &Database<WriteMap>, table_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_ro_txn()?;
    let table = txn.open_table(Some(table_name))?;
    let mut cursor = txn.cursor(&table)?;
    let mut output = String::new();
    output.push_str(&format!("Values in table '{}':\n", table_name));
    for (i, item) in cursor.iter::<Vec<u8>, Vec<u8>>().enumerate() {
        if i >= 10 {
            output.push_str("... (truncated)\n");
            break;
        }
        match item {
            Ok((key, value)) => {
                let key_str = String::from_utf8_lossy(&key);
                let value_str = String::from_utf8_lossy(&value);
                output.push_str(&format!("Key: {}, Value: {}\n", key_str, value_str));
            }
            Err(e) => {
                output.push_str(&format!("Error iterating values: {}\n", e));
                break;
            }
        }
    }
    txn.commit()?;
    Ok(output)
}

pub fn get(env: &Database<WriteMap>, table_name: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_ro_txn()?;
    let table = txn.open_table(Some(table_name))?;
    let result = match txn.get::<Vec<u8>>(&table, key.as_bytes()) {
        Ok(Some(value_bytes)) => {
            match String::from_utf8(value_bytes.clone()) {
                Ok(value) => format!("Key '{}' has value '{}'.", key, value),
                Err(_) => format!("Key '{}' has a not valid UTF-8 value '{:?}'.", key, value_bytes),
            }
        },
        Ok(None) => format!("Key '{}' does not exist in table '{}'.", key, table_name),
        Err(e) => format!("Error: {}.", e),
    };
    txn.commit()?;
    Ok(result)
}

pub fn put(env: &Database<WriteMap>, table_name: &str, key: &str, value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_rw_txn()?;
    let table = txn.open_table(Some(table_name))?;
    txn.put(&table, key.as_bytes(), value.as_bytes(), WriteFlags::empty())?;
    txn.commit()?;
    Ok(format!("Key '{}' inserted with value '{}' in table '{}'.", key, value, table_name))
}

pub fn del(env: &Database<WriteMap>, table_name: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_rw_txn()?;
    let table = txn.open_table(Some(table_name))?;
    let result = txn.del(&table, key.as_bytes(), None)?;

    // Check if the key was deleted
    if result {
        txn.commit()?;
        return Ok(format!("Key '{}' deleted from table '{}'.", key, table_name));
    } else {
        return Ok(format!("Key '{}' does not exist in table '{}'.", key, table_name));
    }
}

pub fn list_tables(env: &Database<WriteMap>) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_ro_txn()?;
    let main_db = txn.open_table(None)?;
    let mut cursor = txn.cursor(&main_db)?;
    let mut output = String::new();
    output.push_str("Available tables:\n");
    for (i, item) in cursor.iter::<Vec<u8>, Vec<u8>>().enumerate() {
        if i >= 10 {
            break;
        }
        match item {
            Ok((key_bytes, _)) => {
                if let Ok(table_name) = String::from_utf8(key_bytes.clone()) {
                    output.push_str(&format!("- {}\n", table_name));
                } else {
                    output.push_str(&format!("- (non-utf8 key): {:?}\n", key_bytes));
                }
            }
            Err(e) => {
                output.push_str(&format!("Error iterating tables: {}\n", e));
                break;
            }
        }
    }
    txn.commit()?;
    Ok(output)
}

pub fn create_table(env: &Database<WriteMap>, table_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_rw_txn()?;
    txn.create_table(Some(table_name), Default::default())?;
    txn.commit()?;
    Ok(format!("Table '{}' created successfully.", table_name))
}

pub fn empty_table(env: &Database<WriteMap>, table_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txn = env.begin_rw_txn()?;
    let table = txn.open_table(Some(table_name))?;
    let mut cursor = txn.cursor(&table)?;
    let mut output = String::new();
    output.push_str(&format!("Emptying table '{:?}':\n", table));

    for item in cursor.iter::<Vec<u8>, Vec<u8>>() {
        match item {
            Ok((key, _)) => {
                txn.del(&table, &key, None)?;
                output.push_str(&format!("Deleted key: {}\n", String::from_utf8_lossy(&key)));
            }
            Err(e) => {
                output.push_str(&format!("Error iterating table: {}\n", e));
                break;
            }
        }
    }

    output.push_str(&format!("Table '{}' emptied successfully.\n", table_name));
    txn.commit()?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use libmdbx::{Database, DatabaseOptions, WriteMap};
    use tempfile::tempdir;

    #[test]
    fn test_commands() {
        // Create a temporary directory for the database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_commands_db.mdbx");

        // Define database options
        let db_options = DatabaseOptions {
            max_tables: Some(10),
            ..Default::default()
        };

        // Open the MDBX environment
        let env: Database<WriteMap> = Database::open_with_options(&db_path, db_options).unwrap();

        // Create a table
        let result = create_table(&env, "test_table").unwrap();
        assert_eq!(result, "Table 'test_table' created successfully.");

        // Test `put` command
        let result = put(&env, "test_table", "key1", "value1").unwrap();
        assert_eq!(result, "Key 'key1' inserted with value 'value1' in table 'test_table'.");

        // Test `get` command
        let result = get(&env, "test_table", "key1").unwrap();
        assert_eq!(result, "Key 'key1' has value 'value1'.");

        // Test `del` command
        let result = del(&env, "test_table", "key1").unwrap();
        assert_eq!(result, "Key 'key1' deleted from table 'test_table'.");
        let result = get(&env, "test_table", "key1").unwrap();
        assert_eq!(result, "Key 'key1' does not exist in table 'test_table'.");

        // Test deleting an already deleted key
        let result = del(&env, "test_table", "key1").unwrap();
        assert_eq!(result, "Key 'key1' does not exist in table 'test_table'.");

        // Create another table and insert some data
        let result = create_table(&env, "another_table").unwrap();
        assert_eq!(result, "Table 'another_table' created successfully.");

        // Test `list_tables` command
        put(&env, "another_table", "key2", "value2").unwrap();
        let result = list_tables(&env).unwrap();
        assert!(result.contains("test_table"));
        assert!(result.contains("another_table"));

        // Test `list_values` command
        put(&env, "test_table", "key3", "value3").unwrap();
        put(&env, "test_table", "key4", "value4").unwrap();
        let result = list(&env, "test_table").unwrap();
        assert!(result.contains("Key: key3, Value: value3"));
        assert!(result.contains("Key: key4, Value: value4"));

        // Test `empty_table` command
        let result = empty_table(&env, "test_table").unwrap();
        println!("{}", result);
        assert!(result.contains("Table 'test_table' emptied successfully."));
        let result = list(&env, "test_table").unwrap();
        assert!(result.contains("Values in table 'test_table':\n"));
    }
}

