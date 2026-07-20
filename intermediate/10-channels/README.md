# 10 — Channels: la cocina deja de gritar

Aprenderás `mpsc`, movimiento a threads, cierre del canal e iteración receptora.
**Tiempo:** 30 min. **Infraestructura:** ninguna.

Un channel es una banda de pedidos: transfiere mensajes sin compartir la libreta.
La versión estándar tiene múltiples productores y un consumidor.

Predice por qué `receiver.into_iter()` termina sin un mensaje especial.

```bash
cargo run
cargo test
```

El sender se mueve al worker y se destruye al terminar; el receptor interpreta
el cierre como fin. Añade un segundo productor. Como desafío, envía un enum
`KitchenEvent` en vez de strings y maneja pedido listo o rechazado.

¿Cuándo preferirías mensajes sobre memoria compartida? Sigue con
[smart pointers](../11-smart-pointers/).

