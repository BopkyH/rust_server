# rust_server


## Features

- **User CRUD API:** Create, read, update, and delete users
- **Layered architecture:** Handlers, services, repositories, models
- **PostgreSQL support:** Connection pooling with r2d2
- **Input validation:** All field errors returned in JSON
- **Custom error handling:** Structured responses for all error types
- **Bearer authentication:** Simple middleware (static token)
- **Modern best practices:** .env config, logging, separation of concerns

## Endpoints

| Method | Route         | Description                | Auth required |
|--------|--------------|----------------------------|--------------|
| GET    | /users       | Get all users              | Yes          |
| GET    | /users/{id}  | Get user by ID             | Yes          |
| POST   | /users       | Create a new user          | Yes          |
| PUT    | /users/{id}  | Update user                | Yes          |
| DELETE | /users/{id}  | Delete user                | Yes          |

All endpoints require Bearer token:  
`Authorization: Bearer my_secret_token_123`

## Quick Start

1. **Install Rust and Diesel CLI**
   ```bash
   cargo install diesel_cli --no-default-features --features postgres


  ```bash
Authorization: Bearer my_secret_token_123
```

## Create .env in core project
  ```bash
DATABASE_URL=postgres://postgres:PASSWORD@localhost/DB_NAME
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
LOG_LEVEL=debug
```


## RUN MIGRATIONS
  ```bash
diesel migration run
```

## RUN SERVER 
  ```bash
cargo run
```

## API usage examples

## Get all users
  ```bash
curl -i -H "Authorization: Bearer my_secret_token_123" http://localhost:8080/users
```

## Create user
  ```bash
curl -i -X PUT http://localhost:8080/users/1 ^
  -H "Authorization: Bearer my_secret_token_123" ^
  -H "Content-Type: application/json" ^
  -d "{\"username\": \"JaneDoe\", \"email\": \"jane@example.com\"}"
 ```

## Delete user
  ```bash
curl -i -X DELETE http://localhost:8080/users/1 ^
  -H "Authorization: Bearer my_secret_token_123"
```
