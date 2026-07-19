# 07 — Ownership: la única llave de la cafetería

## Qué vas a aprender

Movimiento, alcance, liberación automática y devolución de propiedad.
**Tiempo:** 25 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Un pedido `String` viaja a una función para recibir una nota. Ownership se parece
a la única llave del local: transferirla cambia quién puede usarla. La memoria no
es literalmente una llave; `Copy`, préstamos y concurrencia amplían las reglas.

## Antes de ejecutar

¿Puedes imprimir `order` después de pasarlo a `add_delivery_note` pero antes de
recibir el retorno? ¿Quién es dueño del texto dentro de la función?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

Pasar el `String` mueve su propiedad. La función lo modifica y lo devuelve. Al
salir del último alcance, Rust libera la memoria una sola vez, sin recolector de
basura y sin pedirnos un `free` que olvidar un viernes por la tarde.

## Experimenta y desafío

Intenta usar el valor movido y estudia la sugerencia del compilador. Después
crea una función que consuma dos pedidos y devuelva uno combinado.

## Para pensar

- ¿Por qué evitar dos propietarios reduce dobles liberaciones?
- ¿Copiar todo solucionaría el problema sin costos?

## Siguiente paso

[Borrowing](../08-borrowing/) prestará la llave sin regalar el local.

