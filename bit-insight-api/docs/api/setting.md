## setting api

set token

```bash
export TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0IiwiY29tcGFueSI6InJ1c3RfYXBpIiwiZXhwIjoxNzM0NTM3MTE2fQ.mMNmOUFyRdo-R9EwkFX3NWjskusGx8pgyLPIGKw9ZkE
```

get

```bash

curl http://127.0.0.1:8080/v1/settings/push
curl http://127.0.0.1:8080/v1/settings/push -H "Authorization: Bearer $TOKEN"
```

update

```bash
curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/v1/settings/push -d '
{
    "push_token": "123456"
}
'
```
