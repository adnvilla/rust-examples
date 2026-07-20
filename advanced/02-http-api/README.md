# 02 — HTTP: una cafetería con rutas

Aprenderás Axum, handlers async, extractores, JSON y estado con `RwLock`.
**Tiempo:** 35 min. **Infraestructura:** ninguna.

El router es recepcionista: dirige cada método y ruta al handler correcto. El
lock no convierte la memoria en base de datos; al reiniciar, olvida hasta la
orden del cliente que juró ser habitual.

```bash
cargo run
curl http://127.0.0.1:3000/health
curl -X POST http://127.0.0.1:3000/orders -H 'content-type: application/json' -d '{"product":"café","quantity":2}'
```

Predice qué ocurre con JSON inválido. Como desafío, valida cantidad, devuelve
`201 Created` y prueba el router sin abrir un puerto.
