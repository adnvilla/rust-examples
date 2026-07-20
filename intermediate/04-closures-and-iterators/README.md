# 04 — Closures e iteradores: la cinta transportadora

Aprenderás captura de contexto, evaluación perezosa y `filter`/`map`/`sum`.
**Tiempo:** 25 min. **Infraestructura:** ninguna.

El iterador es una cinta que procesa un elemento cuando alguien lo pide; la
metáfora falla en adaptadores que consumen o producen estados más complejos.

Predice qué ocurre si quitas `sum`: ¿el pipeline hace trabajo?

```bash
cargo run
cargo test
```

La closure de `filter` captura `minimum_cents`; `map` transforma y `sum` consume.
Añade una venta barata y comprueba que desaparece. Como desafío, agrupa totales
por producto sin crear índices manuales.

¿Qué gana y qué pierde un pipeline frente a un `for`? Continúa con
[errores propios](../05-custom-errors/).

