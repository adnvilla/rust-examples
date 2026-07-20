# 08 — Unsafe seguro: material radiactivo en una caja

Aprenderás invariantes, aritmética de punteros y encapsulación segura. **Tiempo:**
30 min. **Infraestructura:** ninguna.

`unsafe` permite operaciones que el compilador no puede demostrar; no apaga las
reglas. La función pública verifica límites antes de construir el slice.

```bash
cargo run
cargo test
```

Predice los casos de overflow y fuera de rango. Lee el comentario `SAFETY` y
relaciona cada afirmación con una comprobación. Como desafío, demuestra con
pruebas de frontera que la API equivale a `slice.get(range)`. En producción se
usaría la alternativa segura; aquí el rodeo existe para estudiar el contrato.
