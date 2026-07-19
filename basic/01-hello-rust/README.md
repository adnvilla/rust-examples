# 01 — Hola, Rust

> Un programa diminuto también puede enseñarnos cómo trabaja todo el taller.

## Qué vas a aprender

- Qué hacen `cargo run`, `fn main` y `println!`.
- La diferencia entre compilar y ejecutar.
- Cómo está organizado un paquete binario.

**Prerrequisitos:** tener Rust instalado.  
**Tiempo estimado:** 10 minutos.  
**Infraestructura:** ninguna.

## La situación

Necesitamos confirmar que Rust funciona antes de construir el próximo sistema
interplanetario de pedidos de tacos. El programa saludará y nos permitirá
recorrer el camino completo desde el código fuente hasta un ejecutable.

## La metáfora

Cargo es el encargado de un taller: lee el plano (`Cargo.toml`), reúne las
herramientas, pide al compilador construir la pieza y finalmente la pone en
marcha. La metáfora termina cuando aparecen workspaces y tareas especializadas;
Cargo hace bastante más que apretar el botón verde.

## Antes de ejecutar: haz una predicción

1. ¿La palabra `Compiling` aparecerá en todas las ejecuciones?
2. ¿Qué parte de la salida pertenece a Cargo y cuál a nuestro programa?

## Ejecutar

Desde este directorio:

```bash
cargo run
```

La salida del programa será:

```text
¡Hola, Rust! Ferris ya puso el agua para el café.
```

Cargo compila el paquete cuando es necesario y guarda los artefactos en el
`target/` compartido del workspace. En ejecuciones sin cambios puede reutilizar
el trabajo anterior.

## Cómo funciona

`fn main()` es el punto de entrada del ejecutable. Dentro llamamos a
`println!`, una macro: el signo `!` es la pista de que no se trata de una función
ordinaria. La macro escribe texto y añade un salto de línea.

`Cargo.toml` describe el paquete. Este ejemplo hereda versión, edición, licencia
y lints del workspace raíz, pero sigue siendo ejecutable por sí mismo.

## El momento Rust

Antes de ejecutar, Rust transforma y comprueba el programa completo. Este saludo
no pone a prueba ownership, pero ya muestra el ciclo de feedback que usaremos en
todas las lecciones: editar, compilar, escuchar al compilador y experimentar.

## Experimenta

1. Cambia el mensaje por tu nombre y vuelve a ejecutar.
2. Quita el `!` de `println!`. Predice el mensaje de error antes de compilar y
   luego comprueba qué pista ofrece Rust.

## Desafío

Imprime tres líneas: tu nombre, algo que quieras construir y una pregunta que
esperas responder al terminar esta ruta.

<details>
<summary>Pista</summary>

Puedes usar tres llamadas a `println!` o incluir `\n` dentro de un único texto.

</details>

## Para pensar

- ¿Por qué Cargo evita recompilar aquello que no cambió?
- ¿Qué ventajas tiene detectar un error antes de iniciar el programa?

## Siguiente paso

Continúa con [dependencias externas](../02-hello-rust-dependencies/), donde
Ferris delegará su discurso a otro crate.

