# SharedAgenda REST API
This is the Rust crate of the REST API of SharedAgenda. 

## Endpoints
[A file is there with all the endpoints](endpoints.md)

But one example is `/event/list` which lists the events after a specified date

POST with body
```json
{
  "token": "{token}",
  "start_date": "{date}"
}
```
Send the list of event after the specified date 
```json
[
  {
    "name": "{name}",
    "date_start": "{date}",
    "date_end": "{date}",
    "event_id": "{event_id}",
  },
  ...
  {
    "name": "{name}",
    "date_start": "{date}",
    "date_end": "{date}",
    "event_id": "{event_id}",
  }
]
```

## Return Codes
[Go to the codes.md file for the return codes](codes.md)

## Install
A Makefile is provided.

### Dev 

```sh 
$ make
```

The binary will be located at `$PROJECT/target/debug/server`

### Release

```sh 
$ make release
```

The binary will be located at `$PROJECT/target/release/server`

## Configuration
The file where the config is for the database is at
`$HOME/.config/sharedagenda/config.toml`

It looks like this:
```toml
listen_address = "localhost"
listen_port = 8000

[database]
host = "localhost"
port = "5432"
user = "postgres"
password = "postgres"
database = "agenda"
```

You need to have a `user` who has the rights to create tables, insert, delete, update
and select on all created tables on the database in the configuration. 

You can create a role and a database like this for example
```sql
$ CREATE ROLE agenda PASSWORD 'agenda'
$ CREATE DATABASE agenda OWNER agenda
```

> NB: Using a role who is the OWNER on the database might not be the best idea
> for production. But it does work.
