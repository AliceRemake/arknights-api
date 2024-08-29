# arknights-api

## Quick Start

1. postgres

```bash
docker run -d \
-e POSTGRES_USER={} \
-e POSTGRES_PASSWORD={} \
-e POSTGRES_DB={} \
-v postgres-volume:/var/lib/postgresql/data \
-p 5432:5432 \
--restart=always \
--network=bridge \
--name postgres \
postgres:16
```

2. arknights-api

```bash
docker run -itd \
-e RUST_LOG={} \
-e DATABASE_URL={} \
-e TOKEN={} \
-v arknights-api-volume:/root/.arknights-api \
-p 3000:3000 \
--restart=always \
--network=bridge \
--name arknights-api \
ghcr.io/aliceremake/arknights-api:latest
```

## Develop

1 `.env`

```.env
DATABASE_URL=postgres://{USERNAME}:{PASSWORD}@{HOST}:{PORT}/{DATABASE}
TOKEN={TOKEN}
RUST_LOG={LOG_LEVEL}
```
