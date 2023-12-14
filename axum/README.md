```bash
curl -i localhost:8000/users/1
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

- [Using closure captures](https://docs.rs/axum/latest/axum/#using-closure-captures)
