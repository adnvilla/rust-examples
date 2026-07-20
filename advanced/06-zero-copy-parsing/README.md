# 06 — Zero-copy: cortar el mapa, no fotocopiarlo

Aprenderás slices prestados, parsing, lifetimes y asignaciones evitadas.
**Tiempo:** 25 min. **Infraestructura:** ninguna.

El parser devuelve ventanas del texto original; no crea un nuevo letrero para
cada palabra. Por eso el resultado nunca puede sobrevivir a la entrada.

```bash
cargo run
cargo test
```

Predice si `level.as_ptr()` cae dentro del rango de `input`. Como desafío,
interpreta varias líneas conservando referencias y mide asignaciones frente a
una versión con `String`. ¿Cuándo copiar sería más simple y suficientemente barato?
