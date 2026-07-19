# 12 — Colecciones e iteradores: las ventas eligen campeón

## Qué vas a aprender

Arrays, `HashMap`, `entry`, iteradores, closures y `Option` en combinación.
**Tiempo:** 30 minutos. **Infraestructura:** ninguna.

## La situación y la metáfora

Agruparemos ventas y encontraremos el producto ganador. Un `HashMap` es un
casillero donde cada llave conduce a un valor; la metáfora falla porque su orden
no está garantizado y las colisiones se resuelven internamente.

## Antes de ejecutar

¿Cuánto acumula `café`? ¿Qué devuelve `best_seller` para un mapa vacío?

```bash
cargo run
cargo test
```

## Cómo funciona y el momento Rust

`entry(...).or_insert(0)` obtiene o crea el contador. El iterador busca el máximo
sin exponer índices ni permitir invalidarlos. El lifetime conecta el nombre
devuelto con los textos prestados que viven dentro del mapa.

El desempate por nombre hace determinista el resultado aunque `HashMap` no tenga
orden estable; ni siquiera los campeonatos de café deberían decidirse al azar.

## Experimenta y desafío

Vacía las ventas y maneja `None`. Después devuelve un ranking ordenado de mayor a
menor, sin modificar el mapa original.

## Para pensar

- ¿Cuándo elegirías `Vec` o `BTreeMap` en lugar de `HashMap`?
- ¿Qué trabajo evita expresar la transformación como iteradores?

## Siguiente paso

Has completado el núcleo básico. El nivel intermedio comenzará separando módulos
y diseñando APIs reutilizables.
