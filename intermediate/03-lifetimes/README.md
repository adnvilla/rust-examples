# 03 — Lifetimes: reseñas que no sobreviven al restaurante

Aprenderás anotaciones de lifetime, structs con referencias y relaciones de
validez. **Tiempo:** 25 min. **Infraestructura:** ninguna.

Un lifetime es la fecha de devolución escrita en dos préstamos relacionados; no
alarga la vida de nada, solo permite demostrar qué referencia puede devolverse.

Predice qué ocurriría si `second` se creara en un bloque interior y la reseña se
usara fuera.

```bash
cargo run
cargo test
```

`most_detailed` promete que el resultado vive como máximo lo que viven sus
entradas. Provoca el caso inválido y lee el diagnóstico. Como desafío, crea
`Announcement<'a>` que combine título propio con una cita prestada.

¿Cuándo preferirías poseer un `String`? Sigue con
[closures e iteradores](../04-closures-and-iterators/).

