# 08 — Borrowing: prestar sin desaparecer

## Qué vas a aprender

Referencias `&T`, referencias mutables `&mut T`, coerción a `&str` y exclusividad.
**Tiempo:** 25 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Queremos leer y luego marcar una etiqueta sin transferirla. Un préstamo compartido
es mirar un menú; uno mutable es llevártelo para corregir el precio. Mientras lo
editas, nadie debería leer una mitad vieja y otra nueva.

## Antes de ejecutar

¿Por qué `label_length` no devuelve la etiqueta? ¿Podrían existir dos `&mut`
activos al mismo valor?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`&label` presta lectura; `&mut label` presta modificación exclusiva. El dueño
continúa siendo `main`, y los préstamos terminan tras su último uso. El borrow
checker descarta accesos incompatibles antes de ejecutar.

## Experimenta y desafío

Intenta imprimir la etiqueta mientras una referencia mutable aún se usa después.
Luego crea `shorten_label(&mut String, max_chars)` sin romper UTF-8.

## Para pensar

- ¿Qué bug concurrente anticipa la regla de exclusividad?
- ¿Por qué aceptar `&str` suele ser más flexible que `&String`?

## Siguiente paso

[Structs y métodos](../09-structs-and-methods/) agrupará datos que pertenecen
juntos.

