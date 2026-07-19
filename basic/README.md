# Nivel básico

Este nivel construye el vocabulario y, más importante, el modelo mental de Rust.
El orden importa: intentar aprender borrowing antes de ownership es como recibir
las reglas para devolver una bicicleta que todavía nadie te ha prestado.

## Ruta

| # | Ejemplo | Aprenderás |
| --- | --- | --- |
| 01 | [Hola, Rust](01-hello-rust/) | Cargo, `main`, macros y compilación |
| 02 | [Dependencias](02-hello-rust-dependencies/) | crates externos, imports y `Result` |
| 03 | [Variables y tipos](03-variables-and-types/) | inmutabilidad, inferencia, tipos y tuplas |
| 04 | [Funciones y expresiones](04-functions-and-expressions/) | parámetros, retornos y bloques con valor |
| 05 | [Control de flujo](05-control-flow/) | `for`, `if`, retorno temprano y `match` |
| 06 | [Strings y Unicode](06-strings-and-unicode/) | texto prestado, texto propietario y UTF-8 |
| 07 | [Ownership](07-ownership/) | movimiento, alcance y liberación automática |
| 08 | [Borrowing](08-borrowing/) | referencias compartidas y mutables |
| 09 | [Structs y métodos](09-structs-and-methods/) | datos con nombre, constructores y métodos |
| 10 | [Enums y match](10-enums-and-match/) | estados válidos, patrones y exhaustividad |
| 11 | [`Option` y `Result`](11-option-and-result/) | ausencia, errores propios y propagación |
| 12 | [Colecciones e iteradores](12-collections-and-iterators/) | agregación, closures y transformaciones |

Cada ejemplo puede ejecutarse desde su directorio con `cargo run`. Lee primero
la predicción de su README, prueba el programa y termina con el desafío.

Al completar las doce lecciones podrás explicar cómo Rust representa datos,
controla la propiedad de la memoria y obliga a tratar estados ausentes o fallos
esperados. Esa base permite entrar al nivel intermedio sin depender de magia ni
de un sacrificio ceremonial al borrow checker.
