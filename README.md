# ink-core

A small Rust library + CLI for working with digital ink. A **canvas** holds many **strokes**, and each stroke is a colored list of **points** (with position, timestamp, and pressure). `ink-core` loads a canvas from JSON and provides geometry operations on it, such as bounding boxes, stroke length, and hit-testing. These are the building blocks a drawing app needs for selection, erasing, and rendering.

## Prerequisites
* [Rust & Cargo](https://rustup.rs/)

## What the CLI does

When you run the program, it:

1. Loads `canvas.json` from the current directory (or a file path you pass as an argument).
2. Starts the interactive CLI, type ``help`` to see available commands

```
Available commands:
  canvas info                       - number of strokes and overall bounding box
  canvas translate <dx> <dy>        - shift every point on the canvas
  stroke list                       - list every stroke with index, length and color
  stroke bbox <index>               - bounding box of a stroke
  stroke add-point <index> <x> <y>  - append a point to a stroke
  hit <x> <y> [tolerance]           - check if a point touches any stroke
  save <file.json | file.svg>       - save the canvas as JSON or SVG
  help                              - show this message
  quit                              - exit the program
```

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
Note: Currently a few functions does not implement test yet, I am working on it to include them.

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
| `src/error.rs` | `InkError` enum for I/O, JSON, and index errors and `ParseArgError` enum for command parsing errors |
| `src/parser.rs` | Processes user inputted commands and separate options and positionals |