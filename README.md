# ZTimeDB

## Diesel

### Setup

```bash
 diesel setup
```

### Create migration

```sql
CREATE TABLE todos
(
    id          VARCHAR(255) PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP
)
```

```bash
diesel migration generate create_bandwidth
```

### Run migration

```bash
 diesel migration run
```

### Rollback (delete) migration

```sql
DROP TABLE todos
```

```bash
 diesel migration redo
```
