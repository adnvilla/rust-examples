# 03 — PostgreSQL: memoria que sobrevive al proceso

Aprenderás pools async, parámetros SQL y persistencia. **Tiempo:** 35 min.
**Infraestructura:** PostgreSQL efímero con Compose.

El pool es una flotilla de taxis: reutiliza conexiones costosas sin prometer que
cada viaje será gratis. Los parámetros separan datos de SQL.

```bash
docker compose up -d --wait
cargo run
cargo test
docker compose down --volumes
```

Predice el contador tras ejecutar dos veces. Como desafío, usa una transacción
para insertar pedido y líneas atómicamente. `sqlx` aporta driver async y pool;
Tokio conduce el runtime. Los datos viven en `tmpfs` y desaparecen al limpiar.
