# 05 — Control de flujo: el robot que busca estacionamiento

## Qué vas a aprender

`for`, `if`, retorno temprano, `match` y la primera aparición de `Option`.
**Tiempo:** 20 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Un robot recorre lugares hasta hallar uno libre. El flujo de control es una ruta
con desvíos; a diferencia de un mapa, algunos caminos pueden producir valores.
El robot, por fortuna, no cobra por cada vuelta.

## Antes de ejecutar

¿Qué lugar elige? ¿Qué devuelve un estacionamiento vacío?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`enumerate` combina cada estado con su posición. `Option<usize>` representa dos
resultados honestos: existe un lugar o no existe. `match` obliga a atender ambos.

## Experimenta y desafío

Marca todos los lugares como ocupados. Después devuelve también cuántos lugares
inspeccionó el robot.

## Para pensar

- ¿Por qué `None` comunica mejor que devolver el lugar `0`?
- ¿Cuándo conviene un retorno temprano?

## Siguiente paso

[Strings y Unicode](../06-strings-and-unicode/) mostrará por qué texto no es
sinónimo de bytes.

