# Nivel avanzado

Este nivel explora dónde Rust destaca cuando el programa espera redes, comparte
trabajo, conversa con infraestructura o necesita acercarse al metal sin regalarle
las llaves de la memoria.

| # | Ejemplo | Potencialidad | Infraestructura |
| --- | --- | --- | --- |
| 01 | [Async y tasks](01-async-tasks/) | concurrencia cooperativa y cancelación | Ninguna |
| 02 | [API HTTP](02-http-api/) | rutas y estado concurrente con Axum | Ninguna |
| 03 | [PostgreSQL](03-postgres/) | pool, queries y persistencia tipada | Docker Compose |
| 04 | [Redis](04-redis-cache/) | caché async y expiración | Docker Compose |
| 05 | [Observabilidad](05-observability/) | spans y contexto estructurado | Ninguna |
| 06 | [Zero-copy parsing](06-zero-copy-parsing/) | slices prestados sin asignaciones | Ninguna |
| 07 | [Typestate](07-typestate/) | flujos válidos codificados en tipos | Ninguna |
| 08 | [`unsafe` seguro](08-safe-unsafe/) | encapsular invariantes de bajo nivel | Ninguna |

Los ejemplos con servicios se levantan y destruyen dentro de su directorio. No
necesitan una base compartida que alguien bautizó `final_final_ahora_si`.
