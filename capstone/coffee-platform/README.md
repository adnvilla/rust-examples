# Café sin bugs — Proyecto integrador

Una API de pedidos donde PostgreSQL guarda la verdad, Redis recuerda por un rato
y tracing deja migas de pan. Si Redis se cae, la cafetería sigue sirviendo: una
caché dramática no debería convertirse en directora general.

## Qué integra

- Axum y Tokio para HTTP async;
- SQLx y PostgreSQL como fuente de verdad;
- Redis con TTL e invalidación best-effort;
- errores HTTP, validación previa y constraints de base de datos;
- spans estructurados y apagado con `Ctrl+C`;
- Compose efímero con healthchecks y `tmpfs`.

## Ejecutar

```bash
docker compose up -d --wait
cargo run
```

En otra terminal:

```bash
curl http://127.0.0.1:3000/health
curl -X POST http://127.0.0.1:3000/orders \
  -H 'content-type: application/json' \
  -d '{"product":"café sin bugs","quantity":2}'
curl http://127.0.0.1:3000/orders
```

Al terminar:

```bash
docker compose down --volumes
```

## Antes de ejecutar

1. ¿Qué responde el listado si Redis no está disponible?
2. ¿Por qué validamos cantidad tanto en Rust como en PostgreSQL?
3. ¿Qué petición produce cache hit y cuál invalida la caché?

## El momento Rust

`AppState` puede clonarse porque pool y cliente son handles concurrentes. Los
handlers expresan errores y respuestas en sus tipos; SQL usa parámetros; la
caché se trata como optimización falible. El sistema de tipos no garantiza que
la arquitectura sea buena, pero hace visibles muchas de sus promesas.

## Desafío

Añade `GET /orders/{id}`, paginación y una prueba HTTP contra infraestructura
efímera. Después detén Redis y demuestra que PostgreSQL mantiene el servicio.

## Para pensar

- ¿Dónde pondrías reintentos sin duplicar pedidos?
- ¿Qué métricas acompañarían los spans?
- ¿Cuándo migrarías la creación de tabla a migraciones versionadas?
