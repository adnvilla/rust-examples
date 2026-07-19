# 09 — Threads: dos cajeros, una cuenta

Aprenderás `thread::scope`, `spawn`, `join` y préstamos entre hilos. **Tiempo:**
30 min. **Infraestructura:** ninguna.

Los threads son cajeros trabajando en secciones distintas; el scope es el jefe
que no deja irse a nadie antes de entregar resultados.

Predice cómo se divide un slice con cantidad impar.

```bash
cargo run
cargo test
```

Los workers toman prestadas mitades disjuntas y `join` reúne sus sumas. Rust
demuestra que los préstamos no escaparán del scope. Como desafío, usa tantos
workers como núcleos disponibles y mide cuándo la coordinación cuesta más que el
trabajo. ¿Paralelo siempre significa más rápido?

Sigue con [channels](../10-channels/).

