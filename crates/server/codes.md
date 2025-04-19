# Codes
Here are the possible return codes of the API, heavily inspired by HTTP codes

## Code 20X
### Code 200: OK
Everything is fine

## Code 30X
There are no 30X codes

## Code 40X
### Code 400: Wrong
Something has gone wrong

### Code 401: Token is expired
Triggered when an expired token is used

### Code 402: Token does not exist
Triggered when a token which does not exist has been used.

### Code 403: User with the same email exists
Triggered when a user creation is tried with an email which already exists

### Code 404: Password does not match
Triggered when the password supplied does not match the password in the database

### Code 405: User does not exist
Triggered when a login is tried with a user which does not exist
