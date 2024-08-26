# arknights-api

## postgres

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

## pgadmin (optional)

```bash
docker run -d \
-e PGADMIN_DEFAULT_EMAIL={} \
-e PGADMIN_DEFAULT_PASSWORD={} \
-p 5433:80 \
--restart=always \
--network=bridge \
--name pgadmin \
dpage/pgadmin4:8
```

## .env

```env
DATABASE_URL=postgres://{USERNAME}:{PASSWORD}@{HOST}:{PORT}/{DATABASE}
TOKEN={TOKEN}
RUST_LOG={LOG_LEVEL}
```

## arknights-api

```bash
docker build -t arknights-api .
```

```bash
docker run -d \
-e RUST_LOG=info \
-e DATABASE_URL={} \
-e TOKEN={} \
-p 3000:3000 \
--restart=always \
--network=bridge \
--name arknights-api \
arknights-api:latest
```
