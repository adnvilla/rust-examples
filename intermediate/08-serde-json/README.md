# 08 — Serde: JSON con revisión de pasaporte

Aprenderás derive, serialización, deserialización, defaults y rechazo de campos
desconocidos. **Tiempo:** 25 min. **Infraestructura:** ninguna.

JSON es equipaje sin etiqueta hasta que Serde lo revisa contra un tipo. El control
es estructural; las reglas de negocio todavía necesitan validación explícita.

Predice el valor de `priority` cuando el campo falta.

```bash
cargo run
cargo test
```

`Deserialize` construye `Order`; `deny_unknown_fields` detecta equipaje extra y
`default` completa el booleano. Cambia `quantity` por texto. Como desafío, crea
un constructor validado que rechace cantidad cero después de deserializar.

Dependencias: `serde` genera implementaciones y `serde_json` maneja el formato.
Continúa con [threads](../09-threads/).

