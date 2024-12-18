# bit-insight-api


# Required

- MySQL
- Redis

## Getting Started

create database

```sql
CREATE DATABASE bit_insight DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

migrate

```bash
cargo install sqlx-cli
export DATABASE_URL=mysql://root:123456@localhost/bit_insight
sqlx migrate run
```

edit config

config/*.yaml

## Running

```bash
cargo run
```
