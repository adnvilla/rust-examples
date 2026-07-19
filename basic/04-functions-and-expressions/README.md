# 04 — Funciones y expresiones: la propina pensativa

## Qué vas a aprender

Parámetros, retornos, expresiones, bloques y `if` como valor. **Tiempo:** 15
minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Calcularemos una propina sin esconder la regla en `main`. Una función es una
máquina con entradas y salida; la metáfora falla porque una función también
puede tener efectos, aunque esta se mantiene pura y predecible.

## Antes de ejecutar

¿Qué cambia si agregas `;` después de la expresión final de la función?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

El bloque `if` produce el porcentaje. La última expresión, sin punto y coma, es
el valor devuelto. La firma obliga a que ambos caminos produzcan un `u32`.

## Experimenta y desafío

Añade un tercer nivel de servicio. Luego crea `split_bill(total, people)` y
decide explícitamente qué hacer cuando `people` sea cero.

## Para pensar

- ¿Qué aporta una función pura al momento de probar?
- ¿Cuándo una expresión resulta más clara que varias mutaciones?

## Siguiente paso

[Control de flujo](../05-control-flow/) hará que un robot busque estacionamiento.

