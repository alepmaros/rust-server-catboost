# rust-server-catboost
Basic rust server that runs a predict on a catboost model based on the Iris dataset

# curl

```
curl -X POST   http://localhost:8080/invocations -H 'Content-Type: application/json' -d '{
    "f1": 5.1,
    "f2": 3.5,
    "f3": 1.4,
    "f4": 0.2
}'
```

