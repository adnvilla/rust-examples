# 02 — Traits y genéricos: pagos con el mismo uniforme

Aprenderás contratos, implementaciones, bounds y monomorfización. **Tiempo:** 25
min. **Infraestructura:** ninguna.

Tarjeta y efectivo visten distinto, pero caja les exige el mismo protocolo. Un
trait es el reglamento; no es herencia y no guarda estado por sí mismo.

Predice cuántas versiones concretas de `receipt` genera el compilador.

```bash
cargo run
cargo test
```

`M: PaymentMethod` acepta cualquier tipo que cumpla el contrato con despacho
estático. Añade `GiftCard` y haz que una compra falle. Como desafío, crea una
función que acepte una lista heterogénea usando `&dyn PaymentMethod` y compara el
tradeoff. ¿Cuándo importa el despacho dinámico?

Sigue con [lifetimes](../03-lifetimes/).

