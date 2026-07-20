# Rust Examples

Una ruta práctica para aprender Rust desde el primer `println!` hasta servicios
concurrentes, observables y conectados a infraestructura real.

Aquí el compilador no es el villano: es ese compañero meticuloso que revisa el
paracaídas mientras tú preguntas si de verdad hacen falta tantas correas.

## Requisitos

- [Rustup](https://rustup.rs/). El repositorio selecciona automáticamente el
  toolchain indicado en `rust-toolchain.toml`.
- Docker con Compose, únicamente para los ejemplos que lo indiquen.

Comprueba el entorno:

```bash
rustc --version
cargo --version
```

## Cómo recorrer el repositorio

Los ejemplos están agrupados por nivel y numerados en el orden sugerido. Cada
directorio es un paquete ejecutable de manera independiente y tiene su propio
README.

| Nivel | Objetivo | Estado |
| --- | --- | --- |
| [`basic`](basic/) | Sintaxis, tipos, ownership y manejo de errores | Completo |
| [`intermediate`](intermediate/) | Traits, pruebas, CLI y concurrencia | Completo |
| [`advanced`](advanced/) | Async, redes, rendimiento e infraestructura | Completo |
| [`capstone`](capstone/) | Proyecto integrador de servicio completo | Completo |

Empieza aquí:

```bash
cd basic/01-hello-rust
cargo run
```

Después abre el README del ejemplo, modifica el programa y haz el desafío. La
meta no es mirar código como quien observa una licuadora apagada: hay que pulsar
botones y descubrir qué tapa olvidaste poner.

## Ejemplos disponibles

| # | Ejemplo | Conceptos | Infraestructura |
| --- | --- | --- | --- |
| 01 | [Hola, Rust](basic/01-hello-rust/) | Cargo, `main`, macros | Ninguna |
| 02 | [Dependencias](basic/02-hello-rust-dependencies/) | crates, SemVer, `use`, errores | Ninguna |
| 03 | [Variables y tipos](basic/03-variables-and-types/) | inmutabilidad, tipos, tuplas | Ninguna |
| 04 | [Funciones y expresiones](basic/04-functions-and-expressions/) | retornos, bloques, `if` | Ninguna |
| 05 | [Control de flujo](basic/05-control-flow/) | bucles, `match`, retorno temprano | Ninguna |
| 06 | [Strings y Unicode](basic/06-strings-and-unicode/) | `String`, `&str`, UTF-8 | Ninguna |
| 07 | [Ownership](basic/07-ownership/) | movimiento, alcance, liberación | Ninguna |
| 08 | [Borrowing](basic/08-borrowing/) | referencias compartidas y mutables | Ninguna |
| 09 | [Structs y métodos](basic/09-structs-and-methods/) | structs, `impl`, API | Ninguna |
| 10 | [Enums y match](basic/10-enums-and-match/) | variantes, patrones, exhaustividad | Ninguna |
| 11 | [`Option` y `Result`](basic/11-option-and-result/) | ausencia, errores propios, `?` | Ninguna |
| 12 | [Colecciones e iteradores](basic/12-collections-and-iterators/) | `HashMap`, closures, iteradores | Ninguna |
| I01 | [Módulos y visibilidad](intermediate/01-modules-and-visibility/) | módulos, rutas, APIs | Ninguna |
| I02 | [Traits y genéricos](intermediate/02-traits-and-generics/) | contratos, bounds, despacho | Ninguna |
| I03 | [Lifetimes](intermediate/03-lifetimes/) | referencias y validez | Ninguna |
| I04 | [Closures e iteradores](intermediate/04-closures-and-iterators/) | captura, pipelines perezosos | Ninguna |
| I05 | [Errores propios](intermediate/05-custom-errors/) | contexto, `Display`, fuentes | Ninguna |
| I06 | [Estrategias de pruebas](intermediate/06-testing-strategies/) | unitarias, integración, doc tests | Ninguna |
| I07 | [CLI](intermediate/07-command-line-app/) | argumentos, stderr, códigos de salida | Ninguna |
| I08 | [Serde](intermediate/08-serde-json/) | JSON tipado, derive, defaults | Ninguna |
| I09 | [Threads](intermediate/09-threads/) | scope, spawn, join | Ninguna |
| I10 | [Channels](intermediate/10-channels/) | mensajes, movimiento, cierre | Ninguna |
| I11 | [Smart pointers](intermediate/11-smart-pointers/) | `Rc`, `RefCell`, propiedad compartida | Ninguna |
| I12 | [Workspace](intermediate/12-workspace-architecture/) | paquetes y dependencias de ruta | Ninguna |
| A01 | [Async y tasks](advanced/01-async-tasks/) | futures, tasks, cancelación | Ninguna |
| A02 | [API HTTP](advanced/02-http-api/) | Axum, rutas, JSON, estado | Ninguna |
| A03 | [PostgreSQL](advanced/03-postgres/) | pool, SQL parametrizado | Docker Compose |
| A04 | [Redis](advanced/04-redis-cache/) | caché async, TTL | Docker Compose |
| A05 | [Observabilidad](advanced/05-observability/) | spans, events, contexto | Ninguna |
| A06 | [Zero-copy parsing](advanced/06-zero-copy-parsing/) | slices, lifetimes, parsing | Ninguna |
| A07 | [Typestate](advanced/07-typestate/) | estados válidos en tipos | Ninguna |
| A08 | [`unsafe` seguro](advanced/08-safe-unsafe/) | invariantes y abstracción segura | Ninguna |
| P01 | [Café sin bugs](capstone/coffee-platform/) | API, PostgreSQL, Redis, tracing | Docker Compose |

## Verificar todo

Desde la raíz:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

También puedes entrar a cualquier ejemplo y ejecutar los mismos comandos sin
depender de los demás.

## Agregar un ejemplo

Consulta [`CONTRIBUTING.md`](CONTRIBUTING.md) y usa
[`docs/example-template.md`](docs/example-template.md). Las reglas operativas
para colaboradores y agentes están en [`AGENTS.md`](AGENTS.md).

## Licencia

[MIT](LICENSE)
