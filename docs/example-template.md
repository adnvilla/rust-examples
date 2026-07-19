# NN — Nombre del ejemplo

> Una frase corta que presente el problema, no la solución.

## Qué vas a aprender

- Concepto principal.
- Concepto secundario.
- Qué ventaja de Rust observarás.

**Prerrequisitos:** ejemplo anterior o conocimiento mínimo necesario.  
**Tiempo estimado:** 10–20 minutos.  
**Infraestructura:** ninguna, o Docker con Compose.

## La situación

Presenta un caso reconocible y pequeño. Añade humor únicamente si ayuda a
recordar el problema.

## La metáfora

Explica el modelo mental con una comparación breve. Indica también el límite de
la metáfora para evitar una idea incorrecta.

## Antes de ejecutar: haz una predicción

1. ¿Qué crees que imprimirá el programa?
2. ¿Qué cambiaría si modificas la línea señalada?

## Ejecutar

Desde este directorio:

```bash
cargo run
cargo test
```

Si requiere infraestructura:

```bash
docker compose up -d --wait
cargo run
cargo test
docker compose down --volumes
```

Incluye la salida esperada relevante.

## Cómo funciona

Explica las decisiones por bloques. Enlaza fragmentos cortos; no copies todo el
archivo ni describas mecánicamente cada línea.

## El momento Rust

Explica por qué el sistema de tipos, ownership, el modelo de errores, la
concurrencia u otra propiedad de Rust mejora este caso.

## Experimenta

Propón dos cambios pequeños: uno que funcione y otro que produzca un error útil.
Pide predecir el resultado antes de ejecutar.

## Desafío

Define una extensión pequeña con criterios observables de éxito.

<details>
<summary>Pista</summary>

Ofrece dirección sin entregar inmediatamente toda la solución.

</details>

## Para pensar

- Pregunta sobre una decisión o tradeoff.
- Pregunta que conecte el ejemplo con un sistema real.

## Siguiente paso

Enlaza la próxima lección y explica qué conocimiento reutilizará.

