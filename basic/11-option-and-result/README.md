# 11 — Option y Result: fallar con información

## Qué vas a aprender

Ausencia con `Option`, éxito o error con `Result`, errores propios, `?` y
conversiones con `map_err`. **Tiempo:** 30 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

La caja recibe pedidos como `2: café`, aunque algún cliente escribe “muchos
croissants” y confía en la telepatía. `Option` es una caja que puede estar vacía;
`Result` añade una nota explicando por qué no obtuvimos el producto esperado.

## Antes de ejecutar

¿En qué operación aparece `Option`? ¿Cuál de los tres errores devuelve `0: té`?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`split_once` puede no encontrar `:` y por eso devuelve `Option`. `ok_or` lo
convierte a `Result`; `?` detiene la ruta feliz y propaga el error. El enum
`OrderError` conserva causas que el llamador debe decidir cómo mostrar.

## Experimenta y desafío

Prueba entradas vacías, negativas y con espacios. Después acepta un precio
opcional en el formato `cantidad:producto:precio` sin usar `unwrap`.

## Para pensar

- ¿Qué pierde un programa que representa todos los fallos con `false`?
- ¿Cuándo debe registrarse un error y cuándo mostrarse al usuario?

## Siguiente paso

[Colecciones e iteradores](../12-collections-and-iterators/) resumirá muchas
ventas sin perder de vista sus tipos.

