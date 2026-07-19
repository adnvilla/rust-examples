# 05 — Errores propios: “algo salió mal” no es diagnóstico

Aprenderás enums de error, `Display`, `Error::source`, contexto y `?`. **Tiempo:**
30 min. **Infraestructura:** ninguna.

Un error propio es un parte médico: describe la causa y conserva estudios
anteriores. No todos los fallos requieren una enciclopedia, pero sí información
accionable.

Predice la variante para `Ferris,30` y para `Ferris,muchos`.

```bash
cargo run
cargo test
```

`InvalidPartySize` conserva `ParseIntError`; `TooLarge` guarda valores útiles.
Prueba entradas vacías. Como desafío, añade una regla para cantidad cero sin
convertir todo a texto prematuramente.

¿Qué capa debe traducir el error al usuario? Continúa con
[estrategias de pruebas](../06-testing-strategies/).

