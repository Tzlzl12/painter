# Simple Plot Library

A lightweight plotting library powered by **tiny-skia** for rendering and **winit** for window management.

## Features
- Fast CPU‑based vector drawing using tiny‑skia.
- Simple API for common chart types.
- Real‑time display with winit.
- No external heavy dependencies.

## Supported Primitives
The library provides several ready‑to‑use plot primitives located in `src/primitive/`:

| Primitive | Description |
|-----------|-------------|
| **ErrorBar** | Horizontal error bars for representing a range (min‑max) with a mean marker. |
| **Scatter** | Scatter plot with optional point sizes (`value`) and colors (`forth_dim`). |
| **HeatMap** | 2‑D heat‑map visualisation (color‑coded matrix). |
| **Area** | Filled area chart supporting line and step modes. |
| **Histrogram** | Histogram (frequency distribution) rendering. |
| **Curve** | Smooth curve (line) plot. |
| **Stair** | Stair‑case style plot for step‑wise data. |
| **Config** | Configuration struct for colors, line width, etc. (used by all primitives). |

## Building & Running Examples
```bash
cargo run --example scatter   # runs the scatter‑plot example
```

Check the `examples/` directory for more demos of each primitive.

## License
MIT
