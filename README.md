# MDBX CLI Tool

## Overview
Super naive interactive command-line interface for managing and interacting with an MDBX database.

## Usage
1. Build the project using Cargo:
   ```bash
   cargo build
   ```
2. Run the CLI tool:
   ```bash
   cargo run -- --db-path <path_to_mdbx_file>
   ```
3. Enter commands interactively in the shell.

## Notes
- Ensure the MDBX database file exists before running the tool.
- Use the `create_table` command to create tables before performing operations on them.

## How It Works
1. **Initialization**: The tool initializes an MDBX environment using the provided database file path.
2. **Interactive Shell**: Users interact with the database through an interactive shell, where they can input commands.
3. **Command Execution**: Commands are parsed and executed, with results displayed in the shell.

## Available Commands
Below is a list of commands supported by the MDBX CLI Tool:

### 1. `create_table <table>`
Creates a new table in the database.
- **Example**: `create_table my_table`
- **Output**: `Table 'my_table' created successfully.`

### 2. `put <table> <key> <value>`
Inserts a key-value pair into the specified table.
- **Example**: `put my_table my_key my_value`
- **Output**: `Key 'my_key' inserted with value 'my_value' in table 'my_table'.`

### 3. `get <table> <key>`
Retrieves the value associated with the specified key in the table.
- **Example**: `get my_table my_key`
- **Output**: `Key 'my_key' has value 'my_value'.` or `Key 'my_key' does not exist in table 'my_table'.`

### 4. `del <table> <key>`
Deletes the specified key from the table.
- **Example**: `del my_table my_key`
- **Output**: `Key 'my_key' deleted from table 'my_table'.` or `Key 'my_key' does not exist in table 'my_table'.`

### 5. `list <table>`
Lists all key-value pairs in the specified table.
- **Example**: `list my_table`
- **Output**:
  ```
  Values in table 'my_table':
  Key: key1, Value: value1
  Key: key2, Value: value2
  ```

### 6. `list_tables`
Lists all tables in the database.
- **Example**: `list_tables`
- **Output**:
  ```
  Available tables:
  - my_table
  - another_table
  ```

### 7. `empty_table <table>`
Clears all key-value pairs from the specified table.
- **Example**: `empty_table my_table`
- **Output**: `Table 'my_table' emptied successfully.`

## License
This project is licensed under the MIT License.
