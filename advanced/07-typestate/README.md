# 07 — Typestate: no envíes antes de cobrar

Aprenderás estados fantasma, consumo de `self` y APIs que excluyen transiciones
inválidas. **Tiempo:** 25 min. **Infraestructura:** ninguna.

Cada estado es una puerta cuya llave es el tipo actual. `Order<Draft>` no tiene
el método `ship`; el error aparece al compilar, antes de despertar a logística.

```bash
cargo run
cargo test
```

Intenta enviar el borrador. Como desafío, añade cancelación solo a `Draft` y
`Paid`, conservando una referencia de reembolso. ¿Cuándo el número de estados
haría esta técnica demasiado ceremonial?
