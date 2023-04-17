# rust-server-catboost
Basic rust server that runs a predict on a catboost model

# TODO

- [ ] Create simple server that accepts json
- [ ] Dockerize application
- [ ] Train simple catboost model
- [ ] Load it and run predict on server

# curl

```
curl -X POST -H "Content-Type: application/json" -d '{"key1":"value1", "key2":"value2"}' http://localhost:8080
```