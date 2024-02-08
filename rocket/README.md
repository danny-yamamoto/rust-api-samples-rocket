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

## Deploy
```bash
yamamoto_daisuke@cloudshell:~/rust-api-samples-rocket/rocket (sanbox-334000)$ gcloud run deploy
Deploying from source. To deploy a container use [--image]. See https://cloud.google.com/run/docs/deploying-source-code for more details.
Source code location (/home/yamamoto_daisuke/rust-api-samples-rocket/rocket):
Next time, use `gcloud run deploy --source .` to deploy the current directory.

Service name (rocket):
Please specify a region:
 [1] africa-south1
 [2] asia-east1
 [3] asia-east2
 [4] asia-northeast1
 [5] asia-northeast2
 [6] asia-northeast3
 [7] asia-south1
 [8] asia-south2
 [9] asia-southeast1
 [10] asia-southeast2
 [11] australia-southeast1
 [12] australia-southeast2
-  Building and deploying... Building Container.

OK Building and deploying... Done.

  OK Building Container... Logs are available at [https://console.cloud.google.com/cloud-build/builds/a-b-c-d-ed?project=xxxxxxxxxxxx].
  OK Creating Revision...
  OK Routing traffic...
Done.
Service [rocket] revision [rocket-00005-v4l] has been deployed and is serving 100 percent of traffic.
Service URL: https://rocket-hoge-an.a.run.app
yamamoto_daisuke@cloudshell:~/rust-api-samples-rocket/rocket (sanbox-334000)$ 
```

## Port Settings
[Rocket.toml](./Rocket.toml)

## Reference
- [Default Provider](https://rocket.rs/v0.5/guide/configuration/#default-provider)
