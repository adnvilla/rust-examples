# 06 — Pruebas: tres inspectores para el mismo cupón

Aprenderás pruebas unitarias, integración, doc tests y diseño comprobable.
**Tiempo:** 25 min. **Infraestructura:** ninguna.

Las pruebas son inspectores con perspectivas distintas: uno mira piezas, otro
la puerta pública y otro verifica que el manual no mienta.

Predice cuántas categorías muestra `cargo test`.

```bash
cargo test
```

La prueba unitaria vive junto al código, `tests/public_api.rs` consume la API
como otro crate y el bloque del comentario se compila como doc test. Rompe el
ejemplo de documentación para verlo fallar. Como desafío, cubre precio cero y
descuento total mediante una tabla de casos.

¿Qué bug detecta mejor cada nivel? Continúa con [CLI](../07-command-line-app/).

