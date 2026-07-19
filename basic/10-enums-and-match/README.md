# 10 — Enums y match: el pedido no está en tres lugares

## Qué vas a aprender

Enums con datos, patrones, destructuración y exhaustividad. **Tiempo:** 25
minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Un envío está preparando, en camino, entregado o cancelado; nunca todo a la vez.
Un enum es un tablero con una sola luz encendida, pero cada luz puede llevar datos
distintos, como minutos o receptor.

## Antes de ejecutar

¿Qué dato puede leerse en `OnTheWay`? ¿Qué pasa si agregas una variante sin
actualizar `customer_message`?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

Cada variante representa solamente datos válidos para ese estado. `match`
desestructura la variante activa y debe cubrir todas las posibilidades. Así no
existe un `delivered = true` acompañado de un absurdo `cancelled = true`.

## Experimenta y desafío

Añade `Delayed { minutes, reason }` y deja que el error de compilación te guíe.
Después implementa `is_finished(&self) -> bool`.

## Para pensar

- ¿Qué estados imposibles elimina el enum?
- ¿Qué ventaja aporta un `match` exhaustivo al evolucionar el modelo?

## Siguiente paso

[`Option` y `Result`](../11-option-and-result/) modelará ausencia y fallos
esperados.

