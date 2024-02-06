## Install SQLite3 Client
```bash
sudo apt-get update
sudo apt-get install sqlite3
sqlite3 local.db 
```

## Create add and delete queries and migrate data
```bash
cargo install sqlx-cli --no-default-features --features sqlite
sqlx migrate add -r create_table

# Add query to migrations files

sqlx migrate run --database-url sqlite:./local.db
```

## Check the migrated data
```bash
vscode ➜ /workspaces/rust-api-samples-rocket/rocket (main) $ sqlite3 local.db 
SQLite version 3.34.1 2021-01-20 14:10:07
Enter ".help" for usage hints.
sqlite> .tables
_sqlx_migrations  users           
sqlite> select * from users;
1|maria@example.com|0|0|
999|admin@example.com|0|0|
sqlite> .exit
vscode ➜ /workspaces/rust-api-samples-rocket/rocket (main) 
```

## Set environment variables
```bash
export DATABASE_URL="sqlite:./local.db"
echo $DATABASE_URL
```

## Type Checking
```bash
cargo sqlx prepare --database-url "sqlite:./local.db"
```

## Run
```bash
cargo run
```

## Test
```bash
curl -i localhost:8080/users/1
```

## Port Settings
[Rocket.toml](./Rocket.toml)

## Reference
- [Default Provider](https://rocket.rs/v0.5/guide/configuration/#default-provider)
