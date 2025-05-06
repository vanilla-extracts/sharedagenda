# List of Endpoints
## Users
### User list
GET at /user/list

Return the list of users
```json
{
  "code": "{code}",
  "users": [
    {
      "uuid": "{uuid}"
      "email": "{email}",
      "name": "{name}",
      "password": "{password}",
    },
    [...]
    {
      "uuid": "{uuid}",
      "email": "{email}",
      "name": "{name}",
      "password": "{password}"
    }
  ]
}
```
### User creation:
POST at /user/create 

WITH body
```json
{
  "name": "{name}",
  "email": "{email}",
  "password": "{password hash}"
}
```
The server then creates the database entry (after having checked if it didn't
already exist). Creates the `uuid`, creates the associated `agenda`

RETURN HTTP CODE.

### User login
POST at /user/login

WITH body
```json
{
  "email": "{email}",
  "password": "{password hash}"
}
```
The server checks and if successful return the following object
```json
{
  "code": "200 OK"
  "token": "{token}"
}
```
The server stores the tokens for a day then mark them for deletion

### User logout
POST at /user/logout

WITH body
```json
{
  "token": "{token}"
}
```
Deletes the token in the database, effectively loging out the user

It returns
```json
{
  "code": "{code}",
  "body": "{body}"
}
```

### User whoami
POST at /user/whoami

WITH body
```json
{
  "token": "{token}"
}
```
Returns the whole user informations:
```json
{
  "code": "{code}",
  "user": {
    "uuid": "{uuid}",
    "name": "{name}",
    "email": "{email}",
    "password": "{password}"
  }
}
```

### User Modification
POST at /user/modify

WITH body
```json
{
  "token": "{token}",
  "password": "{password hash} -- CAN BE NULL",
  "email": "{email} -- CAN BE NULL",
  "name": "{name} -- CAN BE NULL",
}
```
Checks the token, and modifies the password, email, name if filled.

It returns
```json
{
  "code": "{code}",
  "body": "{body}"
}
```

### User Deletion
POST at /user/delete

WITH body
```json
{
  "token": "{token}",
}
```

It returns
```json
{
  "code": "{code}",
  "body": "{body}"
}
```

## Events
### List 
POST at /event/list 

WITH body
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

### Create an event
POST at /event/create

WITH body
```json
{
  "token": "{token}",
  "date_start": "{date_start}",
  "date_end": "{date_end}",
  "name": "{name}",
  "invitees": [
    "invitee01",
    "invitee02",
  ]
}
```
Creates an event with the specified invitees. 

It returns
```json
{
  "code": "{code}",
  "answer": "{answer}"
}
```

### Modify an event
POST at /event/modify

WITH body
```json
{
  "token": "{token}",
  "event_id": "{event_id}",
  "date_start": "{date_start} -- CAN BE NULL",
  "date_end": "{date_end} -- CAN BE NULL",
  "name": "{name} -- CAN BE NULL",
}
```
Modifies an event with the specified fields.

It returns
```json
{
  "code": "{code}",
  "body": "{body}"
}
```

### Delete an event
POST at /event/delete

WITH body 
```json
{
  "token": "{token}",
  "event_id": "{event_id}",
}
```
Deletes the specified event

It returns
```json
{
  "code": "{code}",
  "body": "{body}"
}
```
