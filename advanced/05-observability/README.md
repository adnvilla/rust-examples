# 05 — Observabilidad: migas de pan estructuradas

Aprenderás events, spans, campos y `#[instrument]`. **Tiempo:** 25 min.
**Infraestructura:** ninguna.

Un log dice qué pasó; un span conserva en qué viaje ocurrió. No reemplaza
métricas ni traces distribuidos, pero evita mensajes como “falló algo por ahí”.

```bash
cargo run
cargo test
```

Predice qué campos acompañan cada evento. Cambia el producto a vacío y observa
el warning. Como desafío, añade duración y un subscriber JSON. `tracing` modela
eventos; `tracing-subscriber` decide cómo mostrarlos.
