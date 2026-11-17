# Crablo — La gran fuga

Juego de acción con perspectiva isométrica, desarrollado en Rust con Macroquad. Elige al personaje rojo o azul y atraviesa cinco sectores enfrentándote a las patrullas de policías verdes. En el último nivel también tendrás que derrotar al capitán para completar la fuga.

## Cómo jugar

Selecciona un personaje en el menú y pulsa **Jugar** o **Enter**. Puedes desplazarte haciendo clic en el suelo o usando WASD y las flechas. Haz clic sobre un enemigo para perseguirlo y atacarlo al acercarte, o mantén Espacio para golpear a un enemigo adyacente.

Derrota a todas las patrullas del sector para activar la salida verde y camina hasta ella. Al pasar al siguiente nivel conservas el personaje y la puntuación, recuperas hasta 25 puntos de vida y recibes 250 puntos de bonificación. La vida máxima es 100. Si llega a cero, termina la partida.

Las monedas son opcionales y suman puntos. La victoria se consigue al despejar el quinto sector y llegar a su salida.

## Controles

| Control | Acción |
| --- | --- |
| Clic izquierdo en el suelo | Caminar hasta la casilla seleccionada |
| Clic izquierdo sobre un enemigo | Perseguirlo y atacar cuerpo a cuerpo |
| WASD o flechas | Moverse por la cuadrícula isométrica |
| Espacio, mantenido | Atacar a un enemigo adyacente |
| Esc | Pausar o reanudar la partida |
| F11 | Alternar entre ventana y pantalla completa |
| 1 / 2, en el menú | Elegir personaje rojo / azul |
| Enter | Iniciar la partida, avanzar tras completar un sector o volver al menú al terminar |

El juego abre en una ventana redimensionable. También puedes cambiar a pantalla completa desde el botón del menú.

## Niveles

1. **Ciudad Verde**
2. **Bosa**
3. **Distrito Grafiti**
4. **El Sena**
5. **Lomas de Cazuca**

Cada sector tiene una distribución de obstáculos distinta. La cantidad de enemigos aumenta de tres a siete y las patrullas se vuelven más resistentes y rápidas. El capitán aparece en Última Frontera.

## Puntuación

| Objetivo | Puntos |
| --- | ---: |
| Recoger una moneda | 100 |
| Derrotar a una patrulla | 50 |
| Derrotar al capitán | 300 |
| Pasar al siguiente nivel | 250 |

## Características

- Escenarios con bloques isométricos en tres paletas de color.
- Personajes y policías con sprites animados.
- Movimiento por clic y persecución de enemigos mediante búsqueda en anchura (BFS).
- Combate cuerpo a cuerpo con tiempo de espera entre ataques.
- Indicadores de vida, puntos, patrullas restantes y progreso de la campaña.
- Textos flotantes al recibir daño, atacar y recoger monedas.
- Avisos ambientales durante el combate, como «Lanza papa bomba» y «Ataca al tombo»: aparecen cada seis segundos y duran tres. Su temporizador se detiene durante la pausa.
- Menú de selección de personaje y pantallas de pausa, cambio de nivel, victoria y derrota.

## Ejecutar el proyecto

Necesitas Rust y Cargo instalados. Desde la carpeta del proyecto:

```sh
cargo run
```

Para compilar una versión optimizada y ejecutarla:

```sh
cargo run --release
```

Las imágenes se incluyen en el ejecutable durante la compilación; no hace falta copiar la carpeta `assets` junto al programa compilado.

## Desarrollo

```sh
cargo check
cargo test
cargo fmt --check
```

Las pruebas cubren la accesibilidad de los mapas, la búsqueda de rutas, la selección de casillas, la progresión entre niveles, la recogida de monedas y la rotación de los avisos.

## Estructura

```text
Crablo/
├── assets/
│   ├── characters.png  # Sprites de protagonistas y policías
│   └── tiles.png       # Bloques y suelos isométricos
├── src/
│   ├── main.rs         # Ventana, menú y estados de la aplicación
│   ├── game.rs         # Movimiento, combate, puntuación y avisos
│   ├── levels.rs       # Mapas de la campaña y búsqueda de rutas
│   └── art.rs          # Dibujo de sprites, escenarios e interfaz
├── Cargo.toml
├── Cargo.lock
└── README.md
```

El proyecto usa Macroquad 0.4.15 y la edición 2024 de Rust.

## Licencias

El código, la documentación y las imágenes del juego son de autoría de los desarrolladores de Crablo y se distribuyen bajo la [licencia MIT](LICENSE). Puedes usarlos, estudiarlos, modificarlos y redistribuirlos, conservando el aviso de copyright y la licencia.

| Componente | Licencia |
| --- | --- |
| Código, documentación e imágenes propios de Crablo | MIT |
| [Rust](https://rust-lang.org/policies/licenses/) | Principalmente MIT o Apache-2.0; algunos componentes tienen avisos adicionales |
| [Macroquad 0.4.15](https://docs.rs/crate/macroquad/0.4.15) | MIT o Apache-2.0 |
| Dependencias y fuente incluida por Macroquad | Licencias de sus respectivos autores |

Los textos de las licencias, las atribuciones y el inventario de dependencias están en [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) y en la carpeta [licenses](licenses/). Al distribuir una compilación, incluye estos avisos junto con `LICENSE`.

Los sprites de los personajes y las texturas de `assets` son recursos propios de los desarrolladores del juego. Su atribución se detalla en [assets/LICENSING.md](assets/LICENSING.md).
