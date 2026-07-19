# 02 — Dependencias: Ferris contrata un megáfono

> Reutilizar una biblioteca evita tallar cada rueda desde una roca nueva.

## Qué vas a aprender

- Qué es un crate externo y cómo se declara en `Cargo.toml`.
- Cómo `use` trae nombres al alcance actual.
- Para qué sirve `Cargo.lock`.
- Cómo devolver un `Result` desde `main`.

**Prerrequisitos:** completar [Hola, Rust](../01-hello-rust/).  
**Tiempo estimado:** 15 minutos.  
**Infraestructura:** ninguna; la primera compilación puede necesitar internet.

## La situación

Ferris quiere saludar dentro de un bocadillo ASCII. Podríamos dibujar cada raya
a mano, pero ya existe el crate `ferris-says`. Delegaremos la tipografía para
concentrarnos en integrar una dependencia sin convertir el saludo en un proyecto
de infraestructura nacional.

## La metáfora

Una dependencia es una herramienta alquilada: declaramos qué modelo aceptamos,
Cargo la consigue y el lockfile registra exactamente cuál recibimos. A diferencia
de una herramienta física, el crate pasa también por el sistema de tipos durante
la compilación; la metáfora no cubre esa integración profunda.

## Antes de ejecutar: haz una predicción

1. ¿Cargo descargará `ferris-says` en cada ejecución?
2. ¿Qué ocurriría si `say` no pudiera escribir en la salida estándar?
3. ¿Por qué contamos caracteres en lugar de bytes?

## Ejecutar

Desde este directorio:

```bash
cargo run
```

Verás el mensaje dentro de un bocadillo y a Ferris debajo. El dibujo exacto es
responsabilidad de `ferris-says`; nuestro programa prepara el texto y el destino
de escritura.

## Cómo funciona

La sección `[dependencies]` declara `ferris-says`. La versión permite a Cargo
seleccionar una release compatible, mientras el `Cargo.lock` de la raíz conserva
la resolución exacta para repetir la compilación.

`use ferris_says::say` permite llamar `say` sin escribir la ruta completa.
También usamos `BufWriter` de la biblioteca estándar para agrupar escrituras a
la terminal.

`main` devuelve `io::Result<()>`. Así propagamos el posible error de escritura
en vez de asegurar con `unwrap()` que el universo jamás cerrará stdout en el peor
momento.

## El momento Rust

El tipo de retorno obliga a reconocer que escribir puede fallar. La ruta feliz
sigue siendo breve, pero el contrato conserva el error y permite que el runtime
lo reporte correctamente.

`message.chars().count()` cuenta caracteres Unicode. `message.len()` contaría
bytes; con signos y acentos los números pueden diferir.

## Experimenta

1. Sustituye `chars().count()` por `len()` y observa el ancho del bocadillo.
2. Elimina el tipo de retorno de `main`. Antes de compilar, predice por qué la
   última expresión deja de ser válida.

## Desafío

Lee un nombre desde los argumentos de línea de comandos y haz que Ferris lo
salude. Si no se proporciona, utiliza `Rustacean misterioso`.

<details>
<summary>Pista</summary>

Investiga `std::env::args().nth(1)` y usa `unwrap_or_else` para generar el valor
predeterminado.

</details>

## Para pensar

- ¿Qué costo de mantenimiento introduce cada dependencia?
- ¿Cuándo preferirías una pequeña implementación propia sobre un crate externo?
- ¿Qué información aporta `Result` que se perdería al ignorar el error?

## Siguiente paso

El siguiente ejemplo estudiará variables, mutabilidad y tipos mediante un
inventario de tacos que, según el equipo nocturno, “ya venía incompleto”.

