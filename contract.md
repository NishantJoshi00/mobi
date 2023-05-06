
## Init Messages

```json
{"src": "c1", "dest": "n1", "body": {"type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"]}}
```

```json
{"src": "n1", "dest": "c1", "body": {"type": "init_ok", "in_reply_to": 1}}
```

## Generate Contract

```json
{"src": "c1", "dest": "n1", "body": {"type": "generate", "msg_id": 1}}
```

```json
{"src": "n1", "dest": "c1", "body": {"type": "generate_ok", "in_reply_to": 1, "id": 123}}
```
