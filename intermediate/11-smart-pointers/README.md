# 11 — Smart pointers: el pastel tiene representantes

Aprenderás `Box`, `Rc`, `RefCell`, conteo de referencias y mutabilidad interior.
**Tiempo:** 30 min. **Infraestructura:** ninguna.

`Rc` entrega varias credenciales de propietario y `RefCell` lleva el registro de
préstamos en runtime. No vuelve thread-safe al valor ni elimina las reglas.

Predice el conteo antes y después de `drop(sidebar)`.

```bash
cargo run
cargo test
```

Dos dueños observan la misma categoría; `borrow_mut` valida exclusividad al
ejecutar. Provoca dos préstamos mutables simultáneos para estudiar el panic. Como
desafío, construye un árbol con hijos `Rc<Category>` y explica cómo evitar ciclos
mediante `Weak`.

¿Por qué `RefCell` desplaza, pero no elimina, el riesgo? Has completado el nivel
intermedio; lo siguiente será async, servicios y observabilidad.
