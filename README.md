# 🦀 Crablo

**Crablo** es un pequeño roguelike de acción con perspectiva isométrica desarrollado en **Rust** usando **Macroquad**. Nació como un proyecto para aprender desarrollo de videojuegos mientras se exploran conceptos como pathfinding, IA, gestión de estados y arquitectura de juegos.

## ✨ Características

* 🎮 Menú principal, partida y pantalla de Game Over.
* 🖱️ Movimiento por clic con **Breadth-First Search (BFS)**.
* ⚔️ Combate cuerpo a cuerpo con sistema de *cooldown*.
* 🤖 Enemigos con IA que persiguen al jugador utilizando **BFS**.
* 💰 Monedas coleccionables y sistema de puntuación.
* ❤️ HUD con vida y puntuación en tiempo real.
* 💥 Textos flotantes para daño y recolección de monedas.
* 🧱 Mapas estáticos con obstáculos y muros diseñados manualmente.
* 🦀 Protagonista inspirado en un cangrejo y enemigos inspirados en el logo de **C++**.

## 🚀 Tecnologías

* **Rust**
* **Macroquad**
* **Cargo**

## 📌 Objetivos del proyecto

Crablo está pensado como una base sobre la cual seguir construyendo un ARPG inspirado en juegos como *Diablo*. Algunas características planeadas incluyen:

* 📦 Sistema de inventario.
* 🛡️ Equipamiento y loot.
* 🧙 Clases y habilidades.
* 👹 Nuevos enemigos y jefes.
* 🗺️ Generación procedural de mapas.
* 💾 Sistema de guardado.

## Project Structure

```text
.
├── src/
│   ├── main.rs
│   └── ...
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Getting Started

Clonar el repositorio:

```bash
git clone <repository-url>
cd <project-folder>
```

Construir el proyecto:

```bash
cargo build
```

Correr el Proyecto:

```bash
cargo run
```

## 📖 Estado

🚧 Proyecto en desarrollo activo. Nuevas mecánicas y contenido se añadirán progresivamente.
