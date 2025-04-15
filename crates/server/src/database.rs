use std::{fmt::Display, process::exit};

use rocket::tokio;
use tokio_postgres::{Client, Connection, NoTls, Row, Socket, tls::NoTlsStream};

use crate::configuration::load;

pub trait QueriedData {
    fn create_from_row(row: &Row) -> Self;
    fn len() -> usize;
}

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
    async fn database_connect() -> Option<(Client, Connection<Socket, NoTlsStream>)> {
        let config = load().unwrap_or_default();
        let database_configuration = config.database;
        match tokio_postgres::connect(
            &format!(
                "postgres://{}:{}@{}:{}/{}",
                &database_configuration.user,
                &database_configuration.password,
                &database_configuration.host,
                &database_configuration.port,
                &database_configuration.database
            ),
            NoTls,
        )
        .await
        {
            Ok((client, connection)) => Some((client, connection)),
            Err(e) => {
                println!("Error while connecting to database {e}");
                None
            }
        }
    }
    pub async fn new() -> Self {
        let (connection, stream) = match Self::database_connect().await {
            Some((client, stream)) => (client, stream),
            None => {
                eprintln!("Error while connecting to database");
                exit(1);
            }
        };
        tokio::spawn(async move {
            if let Err(e) = stream.await {
                eprintln!("connection error: {}", e);
            }
        });
        Self { connection }
    }

    pub async fn query<T: QueriedData>(self, sql: &str) -> Vec<T> {
        let mut res: Vec<T> = vec![];
        match self.connection.query(sql, &[]).await {
            Ok(rows) => {
                for row in rows {
                    if row.len() < T::len() {
                        continue;
                    }
                    res.push(T::create_from_row(&row))
                }
            }
            Err(e) => {
                println!("Error while reading database: {e}");
            }
        }
        res
    }

    pub async fn setup_database(self) -> Result<(), DatabaseError<'static>> {
        let user_success = match self
            .connection
            .batch_execute(
                "
            create table if not exists users(
                uuid varchar(255) primary key,
                email varchar(50) not null,
                name varchar(50) not null,
                password varchar(50) not null
            );",
            )
            .await
        {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let agenda_success = match self
            .connection
            .batch_execute(
                "
            create table if not exists agendas(
                id serial primary key,
                owner varchar(255) not null references users(uuid)
            );",
            )
            .await
        {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let events_success = match self
            .connection
            .batch_execute(
                "
                create table if not exists events(
                    id serial primary key,
                    agenda_id int not null references agendas(id),
                    name varchar(255) not null,
                    date_start date not null,
                    date_end date not null
                );",
            )
            .await
        {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        let token_success = match self
            .connection
            .batch_execute(
                "
            create table if not exists token(
                id serial primary key,
                owner varchar(255) not null references users(uuid),
                expiration_date date not null
            );",
            )
            .await
        {
            Ok(_) => true,
            Err(e) => {
                println!("{e}");
                false
            }
        };

        if user_success && agenda_success && events_success && token_success {
            Ok(())
        } else {
            Err(DatabaseError {
                text: "Error while creating tables",
            })
        }
    }

    pub async fn execute_statement(self, sql: &str) {
        let result = self.connection.batch_execute(sql).await;
        match result {
            Ok(_) => println!("Statement was executed successfully."),
            Err(e) => {
                println!("Error while executing statement: {e}");
            }
        }
    }

    pub async fn execute_multiple_statements(self, sql: Vec<&str>) {
        let mut i = 1;
        for statement in sql {
            match self.connection.batch_execute(statement).await {
                Ok(_) => println!("Success for statement {i}"),
                Err(e) => println!("Error while executing statement: {e}"),
            }
            i += 1;
        }
    }
}
