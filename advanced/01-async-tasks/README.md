# 01 — Async: muchos meseros, pocos hilos

Aprenderás futures, tasks, `await`, `JoinSet` y concurrencia cooperativa.
**Tiempo:** 30 min. **Infraestructura:** ninguna.

Una task es un mesero que cede el paso mientras espera la cocina; no es
necesariamente un hilo nuevo. Predice el orden de finalización y por qué el
ejemplo ordena la salida antes de devolverla.

```bash
cargo run
cargo test
```

`JoinSet` inicia tareas y las cancela si se abandona. Quita el `sort` y observa
el orden temporal. Como desafío, añade timeout y representa expiración sin
`unwrap`. ¿Qué trabajo bloquearía injustamente al runtime?

Tokio aporta runtime, reloj y tasks; sus features están limitadas a lo usado.
