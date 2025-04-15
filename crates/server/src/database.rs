use std::fmt::Display;

use postgres::{Client, NoTls};

use crate::configuration::{Configuration, DatabaseConfiguration, load};

pub struct Database {
    connection: Client,
}

#[derive(Debug)]
pub struct DatabaseError<'a> {
    text: &'a str,
}

impl Display for DatabaseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl std::error::Error for DatabaseError<'_> {
    fn description(&self) -> &str {
        self.text
    }
}

impl Database {
    pub fn new() -> Self {
        let config = match load() {
            Ok(cfg) => cfg,
            Err(_) => Configuration::default(),
        };
        let database_configuration = config.database;
        let client = Client::connect(
            &format!(
                "postgres://{}:{}@{}:{}/{}",
                &database_configuration.user,
                &database_configuration.password,
                &database_configuration.host,
                &database_configuration.port,
                &database_configuration.database
            ),
            NoTls,
        );

        match client {
            Ok(connection) => {
                println!(
                    "Connection with the database {} is good.",
                    database_configuration.database
                );
                Self { connection }
            }
            Err(e) => {
                println!("Error, could not connect to the database. {e}");
                std::process::exit(0);
            }
        }
    }

    pub fn setup_database(mut self) -> Result<(), DatabaseError<'static>> {
        let user_success = match self.connection.batch_execute(
            "
            create table if not exists users(
                uuid varchar(255) primary key,
                email varchar(50) not null,
                name varchar(50) not null,
                password varchar(50) not null
            );",
        ) {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let agenda_success = match self.connection.batch_execute(
            "
            create table if not exists agendas(
                id serial primary key,
                owner varchar(255) not null references users(uuid)
            );",
        ) {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let events_success = match self.connection.batch_execute(
            "
                create table if not exists events(
                    id serial primary key,
                    agenda_id int not null references agendas(id),
                    name varchar(255) not null,
                    date_start date not null,
                    date_end date not null
                );",
        ) {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let token_success = match self.connection.batch_execute(
            "
            create table if not exists token(
                id serial primary key,
                owner varchar(255) not null references users(uuid),
                expiration_date date not null
            );",
        ) {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        match self.connection.close() {
            Ok(_) => println!("Connection to database was closed successfully."),
            Err(e) => println!("Error while closing database: {e}"),
        }

        if user_success && agenda_success && events_success && token_success {
            Ok(())
        } else {
            Err(DatabaseError {
                text: "Error while creating tables",
            })
        }
    }

    pub fn execute_statement(mut self, sql: &str) {
        let result = self.connection.batch_execute(sql);
        match self.connection.close() {
            Ok(_) => println!("Connection to database was closed successfully."),
            Err(e) => println!("Error while closing database: {e}"),
        }
        match result {
            Ok(_) => println!("Statement was executed successfully."),
            Err(e) => {
                println!("Error while executing statement: {e}");
            }
        }
    }

    pub fn execute_multiple_statements(mut self, sql: Vec<&str>) {
        let mut i = 1;
        for statement in sql {
            match self.connection.batch_execute(statement) {
                Ok(_) => println!("Success for statement {i}"),
                Err(e) => println!("Error while executing statement: {e}"),
            }
            i += 1;
        }
        match self.connection.close() {
            Ok(_) => println!("Connection to the database was closed successfully."),
            Err(e) => println!("Error while closing database: {e}"),
        }
    }
}
