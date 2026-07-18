# Lab 2 — Conway's Game of Life

Implementación del **Juego de la Vida de Conway** en Rust usando [raylib](https://www.raylib.com/), desarrollada para el Laboratorio 2 de Gráficos por Computadora.

Autor: **Javier Alvarado**

---

## 📖 Descripción

El Juego de la Vida es un autómata celular ideado por el matemático John Conway en 1970. Se juega sobre una grilla de celdas donde cada una puede estar **viva** o **muerta**, y su estado evoluciona en pasos discretos según reglas muy simples:

1. Una celda **viva** con **2 o 3** vecinas vivas **sobrevive**.
2. Una celda **viva** con menos de 2 o más de 3 vecinas **muere** (soledad o sobrepoblación).
3. Una celda **muerta** con exactamente **3** vecinas vivas **nace**.

A partir de estas reglas emergen patrones complejos: estructuras estáticas, osciladores y naves que se desplazan por la grilla.

En esta implementación la grilla es **toroidal** (los bordes se conectan entre sí), por lo que los patrones que salen por un lado reaparecen por el opuesto.

---

## 🖼️ Captura

![Conway's Game of Life en ejecución](screenshot.png)

---

## ✨ Características

- Grilla de **100 × 100** celdas, escalada a **8 px** por celda (ventana de 800 × 800).
- Bordes con envolvente (*wrap-around*), la grilla se comporta como un toroide.
- Un paso de simulación cada **6 frames** (a 60 FPS) para una animación fluida.
- Contador de FPS en pantalla.
- Escena inicial poblada con una gran variedad de patrones clásicos.

### Patrones incluidos

| Categoría | Patrones |
|-----------|----------|
| Naves | glider, LWSS (nave ligera) |
| Cañones | gosper glider gun |
| Osciladores | pulsar, pentadecathlon, beacon, toad |
| Metuselah | R-pentomino, diehard, acorn |
| Estáticos | block |

---

## 📂 Estructura del proyecto

```
Lab2/
├── Cargo.toml           # Configuración del paquete y dependencias
└── src/
    ├── main.rs          # Ventana, bucle principal y siembra de patrones
    ├── life.rs          # Lógica del autómata (reglas y vecindad)
    ├── patterns.rs      # Definición de los patrones clásicos
    └── framebuffer.rs   # Framebuffer y colores de las celdas
```

---

## 🛠️ Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edición 2024) con `cargo`.
- Dependencias de sistema de raylib (compilador C, CMake y librerías de OpenGL/X11 en Linux).

En Ubuntu/Debian:

```bash
sudo apt install build-essential cmake libx11-dev libxrandr-dev \
    libxi-dev libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev libxinerama-dev
```

---

## 🚀 Ejecución

Desde la carpeta `Lab2/`:

```bash
# Modo desarrollo
cargo run

# Modo optimizado (recomendado para mejor rendimiento)
cargo run --release
```

La primera compilación descarga y construye raylib, por lo que puede tardar unos minutos.

---

## ⚙️ Configuración

Los parámetros principales se pueden ajustar en `src/main.rs`:

| Constante | Valor | Descripción |
|-----------|-------|-------------|
| `GRID_WIDTH` / `GRID_HEIGHT` | `100` | Dimensiones de la grilla en celdas |
| `SCALE` | `8` | Tamaño en píxeles de cada celda |
| `FRAMES_PER_STEP` | `6` | Frames entre cada paso de la simulación (menor = más rápido) |

Para modificar la escena inicial, edita la función `seed_pattern` en `src/main.rs` cambiando los patrones y sus posiciones `(x, y)`.
