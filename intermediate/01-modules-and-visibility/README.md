# 01 — Módulos: cocina con puertas

Aprenderás módulos, rutas, `pub`, imports y APIs pequeñas. **Tiempo:** 20 min.
**Infraestructura:** ninguna.

Una cocina no deja que el cliente reorganice sus cuchillos: expone una ventanilla.
Los módulos son habitaciones y la visibilidad son sus puertas; la metáfora no
explica crates, pero sí por qué ocultar campos protege decisiones internas.

Antes de ejecutar, predice si `main` puede leer `ticket.table` directamente.

```bash
cargo run
cargo test
```

`kitchen` crea tickets válidos y `service` consume solo su API pública. Intenta
hacer público un campo y observa qué acoplamiento aparece. Como desafío, añade
un módulo `billing` que calcule el precio sin acceder a campos privados.

¿Qué cambios internos podrías hacer sin romper consumidores? Continúa con
[traits y genéricos](../02-traits-and-generics/).

