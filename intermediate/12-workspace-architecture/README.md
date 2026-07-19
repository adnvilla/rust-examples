# 12 — Workspace: una empresa, varios talleres

Aprenderás paquetes múltiples, dependencias de ruta, librerías reutilizables y
comandos globales. **Tiempo:** 30 min. **Infraestructura:** ninguna.

Un workspace es un parque industrial: cada paquete tiene su taller y contrato,
pero comparten administración, lockfile y almacén de compilación. La metáfora no
implica que deban desplegarse juntos.

Este ejemplo contiene una aplicación y dos paquetes internos:

```text
workspace-architecture
├── src/main.rs
└── crates
    ├── coffee-domain
    └── coffee-report → coffee-domain
```

Antes de ejecutar, predice qué paquetes recompila Cargo si solo cambia el
formato de `coffee-report`.

```bash
cargo run
cargo test
```

La aplicación depende de dominio y reporte; reporte depende solo de dominio.
Cargo incorpora automáticamente las dependencias de ruta bajo el workspace raíz.
Modifica `coffee-domain` y observa la cadena de recompilación. Como desafío,
añade un paquete `coffee-export` que genere CSV sin depender de la aplicación.

¿Qué fronteras merecen un crate y cuáles serían ceremonia? Has completado el
nivel intermedio: ya puedes diseñar el sistema, no solo convencerlo de compilar.
