# List of Endpoints
## Users
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
  "status": "200 OK"
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

### User Modification
POST at /user/modify

WITH body
```json
{
  "token": "{token}",
  "password": "{password hash}",
  "new_password": "{password hash} -- CAN BE NULL",
  "email": "{email} -- CAN BE NULL",
  "name": "{name} -- CAN BE NULL",
}
```
Checks the token and password, and modifies the password, email, name if filled.
