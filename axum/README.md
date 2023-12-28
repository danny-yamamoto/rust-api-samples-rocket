```bash
export DATABASE_URL="sqlite:./local.db"
export SERVICE_ACCOUNT="/workspaces/rust-api-samples-rocket/axum/key.json"
echo $DATABASE_URL
echo $SERVICE_ACCOUNT
```

## Test
```bash
curl -i localhost:8080/users/1
curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.html" -i
```

```bash
nullvscode ➜ /workspaces/rust-api-samples-rocket (main) $ curl -i localhost:8000/users/1
HTTP/1.1 200 OK
content-type: application/json
content-length: 90
date: Thu, 14 Dec 2023 11:10:07 GMT

{"user_id":1,"email_address":"maria@example.com","created_at":0,"deleted":0,"settings":""}
vscode ➜ /workspaces/rust-api-samples-rocket (main) $ 
```

## Reference
- [Using closure captures](https://docs.rs/axum/latest/axum/#using-closure-captures)
