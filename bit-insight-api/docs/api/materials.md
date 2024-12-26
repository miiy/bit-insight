## material api

create

```bash
curl -X POST -H "Content-Type: application/json" -H "Push-Username: test" -H "Push-Token: 123456" http://127.0.0.1:8080/v1/materials -d '
{
    "category_id": 1,
    "title": "title",
    "author": "admin",
    "source": "source",
    "source_url": "source_url",
    "thumbnail": "thumbnail",
    "summary": "summary",
    "content": "content"
}
'
```


detail

```bash
curl http://127.0.0.1:8080/v1/materials/1
```


lists

```bash
curl http://127.0.0.1:8080/v1/materials
```

delete

```bash
curl -X DELETE http://127.0.0.1:8080/v1/materials/1
```