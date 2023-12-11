```bash
sudo apt-get update
sudo apt-get install sqlite3
sqlite3 local.db 
```

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

```bash
export DATABASE_URL="sqlite:./local.db"
echo $DATABASE_URL
```

```bash
cargo sqlx prepare --database-url "sqlite:./local.db"
```
