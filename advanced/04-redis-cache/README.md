# 04 — Redis: memoria con fecha de caducidad

Aprenderás conexiones multiplexadas, comandos async, TTL y caché. **Tiempo:** 30
min. **Infraestructura:** Redis efímero con Compose.

Una caché es una libreta rápida y olvidadiza, no el acta de nacimiento del dato.
El ejemplo guarda el producto destacado durante 30 segundos.

```bash
docker compose up -d --wait
cargo run
cargo test
docker compose down --volumes
```

Predice el resultado después del TTL. Como desafío, implementa cache-aside con
una función que reciba el productor del valor. `redis` aporta comandos async y
Tokio el runtime; la persistencia del contenedor está desactivada.
