# ink-core

A small Rust library + CLI for working with digital ink. A **canvas** holds many **strokes**, and each stroke is a colored list of **points** (with position, timestamp, and pressure). `ink-core` loads a canvas from JSON and provides geometry operations on it, such as bounding boxes, stroke length, and hit-testing. These are the building blocks a drawing app needs for selection, erasing, and rendering.

## Prerequisites
* [Rust & Cargo](https://rustup.rs/)

## What the CLI does

When you run the program, it:

1. Loads `canvas.json` from the current directory (or a file path you pass as an argument).
2. Prints the bounding box of **each stroke** and the bounding box of the **whole canvas**.
3. Enters an interactive loop that asks which stroke you want to measure (`[1-N]`) and prints its total length.
   * Invalid input or out-of-range numbers are rejected with a hint.
   * Press **ENTER** on an empty line to exit.

If the file is missing or the JSON is malformed, the program prints a readable error (e.g. `I/O error: ...` or `JSON formatting error: ...`) instead of crashing.

## Build & Usage

### Pre-built executable
Extract the zip file and keep the executable in the same folder as `canvas.json`, then run it.

### Build from source

1. Clone the repository:
   ```bash
   git clone https://github.com/UnnamedDavid/ink-core.git
   cd ink-core
   ```
2. Edit `canvas.json` in the project root (see [Canvas file format](#canvas-file-format)).
3. Build and run:
   ```bash
   cargo run --release
   ```
   To use a different file:
   ```bash
   cargo run --release -- my_canvas.json
   ```

To build a standalone binary without running it:
```bash
cargo build --release
```
The executable will be generated in `target/release/`.

### Running tests
```bash
cargo test
```

## Canvas file format

```json
{
  "strokes": [
    {
      "color": { "r": 1.0, "g": 0.0, "b": 0.0 },
      "points": [
        { "x": 0.0, "y": 0.0, "timestamp": 100, "pressure": 0.5 },
        { "x": 3.0, "y": 4.0, "timestamp": 110, "pressure": 0.6 }
      ]
    }
  ]
}
```

| Field | Type | Meaning |
|-------|------|---------|
| `strokes` | array | One object per stroke on the canvas |
| `color` | object | Stroke color; `r`, `g`, `b` are floats from `0.0` to `1.0` |
| `points` | array | Ordered points making up the stroke |
| `x`, `y` | float | 2D position of the point |
| `timestamp` | unsigned integer | When the point was captured (e.g. milliseconds) |
| `pressure` | float | Pen pressure at that point (e.g. `0.0`–`1.0`) |

All fields are required. A stroke may have an empty `points` array; its bounding box is reported as `None`.

## Project structure

| File | Contents |
|------|----------|
| `src/main.rs` | Interactive CLI |
| `src/lib.rs` | Module declarations and re-exports |
| `src/canvas.rs` | `Canvas`: JSON I/O and canvas-wide operations |
| `src/stroke.rs` | `Stroke`: bounding box, length, hit-testing (+ unit tests) |
| `src/geometry.rs` | `Point` and `BoundingBox` math |
| `src/color.rs` | `Color` (RGB) |
| `src/error.rs` | `InkError` enum for I/O, JSON, and index errors |

## Library API

Everything below is public and can be used from Rust via `use ink_core::{Canvas, Stroke, Point, ...};`. Items marked **✅ CLI** are used by the command-line program. Items marked **🚧 Library only** were added to the library but are **not yet available from the CLI**.

### `Canvas`
| Method | Description | Status |
|--------|-------------|--------|
| `from_json_file(path)` | Load a canvas from a JSON file | ✅ CLI |
| `each_bounding_box()` | Bounding box of every stroke | ✅ CLI |
| `overall_bounding_box()` | Bounding box enclosing all strokes | ✅ CLI |
| `to_json_file(path)` | Save the canvas back to a JSON file | 🚧 Library only |
| `remove_stroke(index)` | Remove a stroke by index; returns `InkError::StrokeNotFound` if out of range | 🚧 Library only |
| `clear()` | Remove every stroke from the canvas | 🚧 Library only |
| `translate(dx, dy)` | Shift every point on the canvas by an offset | 🚧 Library only |
| `hit_test(point, tolerance)` | Check whether a point touches any stroke | 🚧 Library only |

### `Stroke`
| Method | Description | Status |
|--------|-------------|--------|
| `length()` | Total length of the stroke (sum of segment lengths) | ✅ CLI |
| `find_bounding_box()` | Smallest box enclosing the stroke | ✅ CLI (via `Canvas`) |
| `new(color)` | Create an empty stroke | 🚧 Library only |
| `add_point(point)` | Append a point to the stroke | 🚧 Library only |
| `hit_test(point, tolerance)` | Check whether a point is within `tolerance` of the stroke | 🚧 Library only |

### `Point`, `BoundingBox`, `Color`
| Item | Description | Status |
|------|-------------|--------|
| `Point::new(x, y, timestamp, pressure)` | Constructor | 🚧 Library only |
| `Point::distance_squared(other)` | Squared distance between two points | 🚧 Library only |
| `Point::distance_squared_to_segment(a, b)` | Squared distance from a point to the segment `a`–`b` | 🚧 Library only |
| `BoundingBox::from_point(x, y)` / `join(other)` | Build and merge bounding boxes | ✅ CLI (internally) |
| `BoundingBox::intersects(other)` | Check whether two boxes overlap | 🚧 Library only |
| `Color::new(r, g, b)` | Constructor | 🚧 Library only |