# Guía de trabajo para agentes y colaboradores

Este repositorio enseña Rust mediante ejemplos pequeños, ejecutables y
deliberadamente curiosos. Toda contribución debe proteger esa experiencia de
aprendizaje.

## Antes de cambiar algo

1. Lee el README raíz, el README del nivel y el README del ejemplo afectado.
2. Ejecuta el ejemplo antes de editarlo para conocer su comportamiento actual.
3. Conserva cada cambio dentro del ejemplo correspondiente, salvo que la tarea
   modifique una convención común.
4. No introduzcas dependencias ni infraestructura si la biblioteca estándar
   permite enseñar el mismo concepto con claridad.

## Organización

- `basic/`: sintaxis y modelo mental esencial. No presupongas experiencia con
  ownership, traits ni async antes de su lección.
- `intermediate/`: diseño modular, traits, genéricos, pruebas, CLI y
  concurrencia.
- `advanced/`: async, servicios, persistencia, observabilidad, rendimiento,
  FFI, WASM y uso razonado de `unsafe`.
- Los ejemplos se nombran `NN-tema-en-kebab-case` y se ordenan por prerrequisito.
- Todo ejemplo es un paquete Cargo independiente y miembro del workspace raíz.
- No crees dependencias de ruta entre lecciones. Un ejemplo debe poder
  entenderse, compilarse y ejecutarse por sí solo.

## Contenido obligatorio de un ejemplo

Cada ejemplo debe incluir:

- `Cargo.toml` con nombre único y metadatos heredados del workspace;
- `README.md` basado en `docs/example-template.md`;
- código en `src/`;
- al menos una prueba útil cuando exista lógica comprobable;
- `compose.yaml`, `.env.example` y comandos de limpieza si requiere servicios;
- comentarios que expliquen el motivo de una decisión, no que traduzcan la
  sintaxis línea por línea.

El programa debe producir una salida determinista siempre que sea razonable.
Los errores esperados deben explicarse; evita `unwrap()` en ejemplos posteriores
a la lección de manejo de errores, salvo que el README justifique por qué es
seguro o pedagógicamente intencional.

## Estilo pedagógico

- Escribe en español claro y cercano.
- Presenta primero un problema concreto y después la herramienta de Rust.
- Usa una metáfora breve para construir el modelo mental, y aclara dónde deja de
  ser exacta.
- Incluye un caso real con un toque de humor que no eclipse el concepto.
- Formula preguntas que obliguen a predecir el comportamiento antes de ejecutar.
- Incluye una modificación que el lector pueda hacer y un desafío pequeño.
- Explica el valor particular de Rust: seguridad, expresividad, rendimiento,
  concurrencia o interoperabilidad. No conviertas la lección en un catálogo de
  sintaxis.
- No antropomorfices al compilador como enemigo; sus mensajes forman parte del
  material de aprendizaje.

## Dependencias

- Prefiere la biblioteca estándar en el nivel básico.
- Añade crates únicamente cuando sean parte del concepto o reduzcan ruido ajeno
  a la lección.
- Explica en el README qué aporta cada dependencia relevante.
- Mantén `Cargo.lock` versionado y no edites el lockfile manualmente.
- Evita features por defecto innecesarias en ejemplos más avanzados.

## Infraestructura efímera

Si un ejemplo necesita PostgreSQL, Redis, NATS u otro servicio:

- define la infraestructura dentro del mismo ejemplo mediante `compose.yaml`;
- fija versiones de imágenes y añade `healthcheck`;
- usa credenciales exclusivamente locales documentadas en `.env.example`;
- permite iniciar con `docker compose up -d --wait`;
- documenta la limpieza con `docker compose down --volumes`;
- no dependas de servicios compartidos ni de configuración global del equipo;
- no incluyas secretos, datos personales ni volúmenes permanentes por defecto.

## Validación requerida

Antes de considerar terminado un cambio, ejecuta desde la raíz:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

Además, entra al ejemplo nuevo o modificado y confirma que el comando indicado
en su README funciona. Para ejemplos con Compose, valida también el arranque,
los healthchecks y la limpieza.

Si una comprobación no puede ejecutarse por restricciones del entorno, informa
exactamente cuál faltó y por qué. No describas como validado algo que solamente
fue inspeccionado.

## Alcance de los cambios

- No reformatees ejemplos no relacionados.
- No modifiques ni elimines trabajo existente sin entender su intención.
- Actualiza la tabla del README raíz y el README del nivel al agregar, mover o
  retirar una lección.
- Mantén los comandos compatibles con macOS, Linux y Windows cuando sea
  razonable; documenta cualquier excepción.
- Nunca incluyas `target/`, archivos `.env`, credenciales ni artefactos de
  infraestructura en Git.

