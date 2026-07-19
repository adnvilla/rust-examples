# Contribuir a Rust Examples

Gracias por ayudar a convertir conceptos difíciles en experimentos pequeños.
Antes de empezar, lee [`AGENTS.md`](AGENTS.md): allí están las reglas de
estructura, estilo pedagógico, dependencias e infraestructura.

## Crear una lección

1. Elige el nivel y el lugar correcto en la secuencia.
2. Crea un paquete con un nombre único:

   ```bash
   cargo new basic/03-nombre-del-tema --vcs none
   ```

3. Hereda los metadatos y lints del workspace en su `Cargo.toml`:

   ```toml
   [package]
   name = "nombre-unico"
   version.workspace = true
   edition.workspace = true
   rust-version.workspace = true
   license.workspace = true
   publish.workspace = true

   [lints]
   workspace = true
   ```

4. Copia la estructura de [`docs/example-template.md`](docs/example-template.md)
   en el README de la lección.
5. Agrega el ejemplo a la tabla del README raíz y del nivel.
6. Ejecuta todas las comprobaciones descritas en `AGENTS.md`.

## Criterios de revisión

Una lección está lista cuando una persona puede:

- saber qué conocimiento necesita antes de empezar;
- ejecutarla siguiendo solamente su README;
- explicar qué ventaja o decisión de Rust demuestra;
- modificarla para responder al menos una pregunta;
- verificar su comportamiento mediante pruebas cuando corresponda;
- limpiar por completo cualquier infraestructura creada.

