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
| `intermediate` | Traits, pruebas, CLI y concurrencia | Planeado |
| `advanced` | Async, redes, rendimiento, FFI y sistemas | Planeado |

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
