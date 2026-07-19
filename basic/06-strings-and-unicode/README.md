# 06 — Strings y Unicode: emojis en la lista de invitados

## Qué vas a aprender

`&str`, `String`, UTF-8, bytes, caracteres y `format!`. **Tiempo:** 20 minutos.
**Infraestructura:** ninguna.

## La situación y la metáfora

Una cafetería internacional imprime saludos con acentos y emojis. Un `&str` es
como leer una nota prestada y `String` como poseer una libreta extensible. La
comparación no describe la codificación UTF-8: ahí necesitamos contar de verdad.

## Antes de ejecutar

¿`greeting.len()` coincide con `greeting.chars().count()`? Explica tu apuesta.

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`len` cuenta bytes; `chars` recorre valores escalares Unicode. `format!` crea un
`String` propietario sin modificar el nombre prestado. Rust no permite indexar
texto con un número porque un carácter no ocupa siempre un byte.

## Experimenta y desafío

Prueba nombres con `ñ`, chino y emojis. Después cuenta palabras con
`split_whitespace` y compara las tres medidas.

## Para pensar

- ¿Qué medida necesita una base de datos y cuál una interfaz?
- ¿Un “carácter visible” siempre equivale a un `char`?

## Siguiente paso

[Ownership](../07-ownership/) explicará quién conserva la libreta.

