use crate::lexer::Lexer;
use crate::parser::{nodes::NodeVariant, Parser};

#[test]
fn create_database_success() {
    let input = "create database my_database;";
    let mut parser = Parser {
        lexer: Lexer::new(input),
    };

    let result = parser.parse_statement();

    let db = match result {
        Ok(NodeVariant::DatabaseDefinition(db)) => db,
        Err(err) => panic!("{}", err),
        _ => panic!("Expected database definition"),
    };

    assert_eq!(db.name, "my_database");
}

#[test]
fn create_table_success() {
    let input = "create table my_table (col_1 bool, col_2 int, col_3 char(5));";
    let mut parser = Parser {
        lexer: Lexer::new(input),
    };

    let result = parser.parse_statement();

    let table = match result {
        Ok(NodeVariant::TableDefinition(table)) => table,
        Err(err) => panic!("{}", err),
        _ => panic!("Expected table definition"),
    };

    assert_eq!(table.name, "my_table");
}
