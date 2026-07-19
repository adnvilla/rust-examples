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
| [`basic`](basic/) | Sintaxis, tipos, ownership y manejo de errores | En construcción |
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
