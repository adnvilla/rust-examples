# 03 — Variables y tipos: el inventario sospechoso

## Qué vas a aprender

Variables inmutables, `mut`, anotaciones de tipo, tuplas y enteros con tamaño
explícito. **Tiempo:** 15 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Una taquería debe vender sin prometer tacos inexistentes. Un tipo es como la
etiqueta de una caja: dice qué puede guardar; no garantiza que el cocinero haya
contado bien. Rust infiere muchas etiquetas, pero podemos hacerlas explícitas.

## Antes de ejecutar

¿Qué devuelve `sell_tacos(3, 5)`? ¿Por qué `stock` no necesita ser mutable?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`u16` expresa que el inventario no es negativo. `min` limita la venta y la tupla
devuelve dos valores relacionados. Las variables son inmutables por defecto:
cuando algo cambia, `mut` lo vuelve una decisión visible.

## Experimenta y desafío

Cambia `initial_stock` a `-1` y lee el error. Después añade el precio unitario y
calcula el total sin usar números de punto flotante.

## Para pensar

- ¿Por qué almacenar dinero en centavos evita algunas sorpresas?
- ¿Un tipo numérico más grande siempre es mejor?

## Siguiente paso

[Funciones y expresiones](../04-functions-and-expressions/) convertirá decisiones
de negocio en valores.

