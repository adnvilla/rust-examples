# 07 — CLI: argumentos sin telepatía

Aprenderás `env::args`, configuración tipada, stderr y `ExitCode`. **Tiempo:** 25
min. **Infraestructura:** ninguna.

Una CLI es un mostrador con formulario: separar parsing de ejecución evita que
la lógica tenga que entrevistar al sistema operativo durante las pruebas.

Predice el código de salida cuando falta la cantidad.

```bash
cargo run -- café 3
cargo run -- café || true
cargo test
```

`main` adapta el entorno; `parse_args` solo recibe un iterador. Añade un argumento
extra y observa el error. Como desafío, soporta `--priority` sin aceptar flags
desconocidos. En PowerShell, inspecciona `$LASTEXITCODE` en lugar de usar `||`.

¿Qué debería pertenecer a configuración y qué a negocio? Sigue con
[Serde](../08-serde-json/).

