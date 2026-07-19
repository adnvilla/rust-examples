# 09 — Structs y métodos: un pedido con forma propia

## Qué vas a aprender

Structs, `impl`, constructores, métodos, campos privados y tipos derivados.
**Tiempo:** 20 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Una cafetería dejará de representar pedidos como tuplas misteriosas. Un struct es
un formulario con campos nombrados; a diferencia del papel, sus métodos pueden
proteger reglas y producir comportamiento junto a los datos.

## Antes de ejecutar

¿Por qué `summary` recibe `&self`? ¿Podría modificar `shots` con esa referencia?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`CoffeeOrder` hace imposible confundir el nombre con el número de shots. `new`
construye valores válidos y `summary` toma prestado el pedido. Los campos privados
obligan al consumidor a usar la API definida.

## Experimenta y desafío

Añade el tamaño del café. Luego impide construir pedidos con cero shots,
devolviendo un `Option<CoffeeOrder>`.

## Para pensar

- ¿Cuándo una tupla deja de comunicar suficiente intención?
- ¿Qué invariantes podría proteger el constructor?

## Siguiente paso

[Enums y match](../10-enums-and-match/) modelará estados mutuamente excluyentes.

