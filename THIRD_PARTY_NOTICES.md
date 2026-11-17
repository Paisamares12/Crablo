# Avisos de software de terceros

El código, la documentación y las imágenes del juego son de autoría de los desarrolladores de Crablo y están bajo la [licencia MIT](LICENSE). Las bibliotecas, herramientas y fuentes de terceros conservan sus licencias y atribuciones originales.

## Rust

Rust usa principalmente MIT o Apache-2.0. Se incluyen los textos y los avisos de su biblioteca estándar en [licenses/rust](licenses/rust/), junto con la versión de la herramienta instalada. La información oficial está en [la política de licencias de Rust](https://rust-lang.org/policies/licenses/).

## Macroquad y sus dependencias

Macroquad 0.4.15 declara `MIT OR Apache-2.0`. Sus textos originales se conservan en [licenses/third-party/macroquad-0.4.15](licenses/third-party/macroquad-0.4.15/), con el aviso de Fedor Logachev.

Este inventario corresponde a `Cargo.lock` y a la compilación para **Windows x86_64 MSVC**, incluyendo dependencias de compilación. Se obtuvo de los metadatos publicados de cada paquete:

```sh
cargo metadata --locked --filter-platform x86_64-pc-windows-msvc --format-version 1
```

En la columna de licencias se conserva la declaración de cada paquete. `OR` indica alternativas; algunos paquetes antiguos expresan MIT o Apache-2.0 con `MIT/Apache-2.0`. Cada carpeta contiene sus textos originales y `Cargo.toml.orig`, con la declaración de licencia y los autores.

| Dependencia | Versión | Licencia declarada | Textos y avisos |
| --- | --- | --- | --- |
| [adler2](https://docs.rs/crate/adler2/2.0.1) | 2.0.1 | `0BSD OR MIT OR Apache-2.0` | [adler2-2.0.1](licenses/third-party/adler2-2.0.1/) |
| [allocator-api2](https://docs.rs/crate/allocator-api2/0.2.21) | 0.2.21 | `MIT OR Apache-2.0` | [allocator-api2-0.2.21](licenses/third-party/allocator-api2-0.2.21/) |
| [autocfg](https://docs.rs/crate/autocfg/1.5.1) | 1.5.1 | `Apache-2.0 OR MIT` | [autocfg-1.5.1](licenses/third-party/autocfg-1.5.1/) |
| [bitflags](https://docs.rs/crate/bitflags/1.3.2) | 1.3.2 | `MIT/Apache-2.0` | [bitflags-1.3.2](licenses/third-party/bitflags-1.3.2/) |
| [bytemuck](https://docs.rs/crate/bytemuck/1.25.0) | 1.25.0 | `Zlib OR Apache-2.0 OR MIT` | [bytemuck-1.25.0](licenses/third-party/bytemuck-1.25.0/) |
| [byteorder](https://docs.rs/crate/byteorder/1.5.0) | 1.5.0 | `Unlicense OR MIT` | [byteorder-1.5.0](licenses/third-party/byteorder-1.5.0/) |
| [cfg-if](https://docs.rs/crate/cfg-if/1.0.4) | 1.0.4 | `MIT OR Apache-2.0` | [cfg-if-1.0.4](licenses/third-party/cfg-if-1.0.4/) |
| [color_quant](https://docs.rs/crate/color_quant/1.1.0) | 1.1.0 | `MIT` | [color_quant-1.1.0](licenses/third-party/color_quant-1.1.0/) |
| [crc32fast](https://docs.rs/crate/crc32fast/1.5.0) | 1.5.0 | `MIT OR Apache-2.0` | [crc32fast-1.5.0](licenses/third-party/crc32fast-1.5.0/) |
| [equivalent](https://docs.rs/crate/equivalent/1.0.2) | 1.0.2 | `Apache-2.0 OR MIT` | [equivalent-1.0.2](licenses/third-party/equivalent-1.0.2/) |
| [fdeflate](https://docs.rs/crate/fdeflate/0.3.7) | 0.3.7 | `MIT OR Apache-2.0` | [fdeflate-0.3.7](licenses/third-party/fdeflate-0.3.7/) |
| [flate2](https://docs.rs/crate/flate2/1.1.9) | 1.1.9 | `MIT OR Apache-2.0` | [flate2-1.1.9](licenses/third-party/flate2-1.1.9/) |
| [foldhash](https://docs.rs/crate/foldhash/0.1.5) | 0.1.5 | `Zlib` | [foldhash-0.1.5](licenses/third-party/foldhash-0.1.5/) |
| [fontdue](https://docs.rs/crate/fontdue/0.9.3) | 0.9.3 | `MIT OR Apache-2.0 OR Zlib` | [fontdue-0.9.3](licenses/third-party/fontdue-0.9.3/) |
| [glam](https://docs.rs/crate/glam/0.27.0) | 0.27.0 | `MIT OR Apache-2.0` | [glam-0.27.0](licenses/third-party/glam-0.27.0/) |
| [hashbrown](https://docs.rs/crate/hashbrown/0.15.5) | 0.15.5 | `MIT OR Apache-2.0` | [hashbrown-0.15.5](licenses/third-party/hashbrown-0.15.5/) |
| [image](https://docs.rs/crate/image/0.24.9) | 0.24.9 | `MIT OR Apache-2.0` | [image-0.24.9](licenses/third-party/image-0.24.9/) |
| [macroquad](https://docs.rs/crate/macroquad/0.4.15) | 0.4.15 | `MIT OR Apache-2.0` | [macroquad-0.4.15](licenses/third-party/macroquad-0.4.15/) |
| [macroquad_macro](https://docs.rs/crate/macroquad_macro/0.1.8) | 0.1.8 | `MIT/Apache-2.0` | [macroquad_macro-0.1.8](licenses/third-party/macroquad_macro-0.1.8/) |
| [miniquad](https://docs.rs/crate/miniquad/0.4.10) | 0.4.10 | `MIT OR Apache-2.0` | [miniquad-0.4.10](licenses/third-party/miniquad-0.4.10/) |
| [miniz_oxide](https://docs.rs/crate/miniz_oxide/0.8.9) | 0.8.9 | `MIT OR Zlib OR Apache-2.0` | [miniz_oxide-0.8.9](licenses/third-party/miniz_oxide-0.8.9/) |
| [num-traits](https://docs.rs/crate/num-traits/0.2.19) | 0.2.19 | `MIT OR Apache-2.0` | [num-traits-0.2.19](licenses/third-party/num-traits-0.2.19/) |
| [png](https://docs.rs/crate/png/0.17.16) | 0.17.16 | `MIT OR Apache-2.0` | [png-0.17.16](licenses/third-party/png-0.17.16/) |
| [quad-rand](https://docs.rs/crate/quad-rand/0.2.3) | 0.2.3 | `MIT` | [quad-rand-0.2.3](licenses/third-party/quad-rand-0.2.3/) |
| [simd-adler32](https://docs.rs/crate/simd-adler32/0.3.9) | 0.3.9 | `MIT` | [simd-adler32-0.3.9](licenses/third-party/simd-adler32-0.3.9/) |
| [ttf-parser](https://docs.rs/crate/ttf-parser/0.21.1) | 0.21.1 | `MIT OR Apache-2.0` | [ttf-parser-0.21.1](licenses/third-party/ttf-parser-0.21.1/) |
| [winapi](https://docs.rs/crate/winapi/0.3.9) | 0.3.9 | `MIT/Apache-2.0` | [winapi-0.3.9](licenses/third-party/winapi-0.3.9/) |

`quad-rand` 0.2.3 declara MIT en su manifiesto, pero su paquete publicado no incluye un archivo LICENSE independiente. Su carpeta conserva esa declaración y sus autores; [licenses/MIT.txt](licenses/MIT.txt) contiene un texto genérico de referencia, sin asignar nuevos titulares ni sustituir los avisos del autor.

Las dependencias específicas de Linux, macOS, Android u otros destinos no están cubiertas por este inventario de Windows. Si se cambia de plataforma o se actualiza `Cargo.lock`, hay que volver a obtener el inventario y conservar los avisos correspondientes.

## Fuente incluida por Macroquad

Macroquad incluye la fuente ProggyClean de Tristan Grimmer, bajo MIT. Se conserva su aviso en [licenses/proggyfonts](licenses/proggyfonts/).

## Imágenes del juego

Los archivos `assets/characters.png` y `assets/tiles.png` son recursos propios de los desarrolladores de Crablo y se distribuyen bajo MIT. Consulta [assets/LICENSING.md](assets/LICENSING.md).

## Distribución

Incluye `LICENSE`, este archivo y la carpeta `licenses` al compartir una compilación, conservando las atribuciones de los desarrolladores de Crablo y de los componentes de terceros.
