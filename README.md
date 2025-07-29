# HarmonyArch

**HarmonyArch** is a semantic architectural modeling engine written in Rust.
It generates structured 2D/3D geometry by projecting symbolic building elements — such as bays, walls, thresholds, and pilasters — through a layered scale hierarchy. The system prioritizes **meaning and structure over meshes**, with an emphasis on **inheritance, constraint propagation, and tier-aware geometry**.

HarmonyArch treats architecture as **logic, not drafting**. It is CLI-native, cleanly modular, and designed to grow into a humane design language for buildings — readable, programmable, and adaptable by craftsmen and architects alike.

---

## ✨ Project Goals

* Model architecture **semantically**, not visually
* Define each element by its **symbolic role** and **tiered structure**
* Use **geometry as a provisioning tool**, not just a render target
* Enable **projection into multiple output forms**:

  * **2D SVG** for plan, elevation, and section
  * **3D STL/STEP** for massing or fabrication
* Maintain strict **separation of domain vs infrastructure**
* Follow **hexagonal architecture principles** (ports/adapters)
* Remain **CLI-native**, with frontends (e.g. Godot) as optional adapters
* Stay minimal enough that a **user can sketch a meaningful building in 15 minutes**

---

## 📐 Key Concepts

| Concept                                | Description                                                                                 |
| -------------------------------------- | ------------------------------------------------------------------------------------------- |
| `Scene`                                | A full architectural composition defined by nested tiers                                    |
| `Tier`                                 | A scale-specific frame (site, footprint, façade, floorplan, etc.)                           |
| `Bay`, `Wall`, `Threshold`, `Pilaster` | Symbolic structural elements with roles                                                     |
| `TierContext`                          | Inherited constraints and expectations passed top-down                                      |
| `PatternDraft`                         | A flexible template with room for adaptation and idiom                                      |
| `GeometrySolver`                       | A swappable port for generating concrete geometry                                           |
| `ProjectedShape`                       | Geometry annotated with symbolic roles and IDs                                              |
| `Allowance` / `Break`                  | Bottom-up signals that inform upstream adaptability                                         |
| `RenderProfile`                        | An environment-specific rendering contract (e.g., output formats, target resolution, units) |

---

## 🧱 Folder Structure

```bash
harmonyarch/
├── src/
│   ├── domain/           # Pure architectural logic, no side effects
│   ├── application/      # Orchestration and use-case logic
│   ├── interface/        # CLI interface (TUI coming soon)
│   ├── infrastructure/   # Geometry adapters (e.g., CadQuery, SVG writer)
│   └── composition/      # Entry point & DI bootstrap
├── examples/             # JSON pattern inputs & config demos
├── tests/                # Unit + integration tests (100% coverage target)
├── Cargo.toml
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

* [Rust](https://rustup.rs/) (`cargo` via `rustup`)
* Optional:

  * [Inkscape](https://inkscape.org/) for SVG output
  * [MeshLab](https://www.meshlab.net/) for STL/OBJ viewing

### Clone and Run

```bash
git clone https://github.com/yourname/harmonyarch.git
cd harmonyarch
cargo run -- --input examples/chapel.json --output out/chapel.svg
```

---

## 🛠 Development Status

* [x] Domain layer bootstrapped (Scene → Tier → Element)
* [ ] Initial GeometrySolver implementation (SVG + STL)
* [ ] Rust test harness + mutation testing setup
* [ ] CLI scaffolding for basic input/output
* [ ] Constraint handling and bottom-up allowances
* [ ] Tier pattern defaults and overridable constraints

---

## 🧩 Planned Adapters

| Target     | Status | Description                                    |
| ---------- | ------ | ---------------------------------------------- |
| CadQuery   | 🔜     | Python-based 3D modeling from parametric input |
| SVG Writer | ✅      | 2D projection for plan/elevation output        |
| Unity      | 🔜     | Visual frontend via geometry loading           |
| Web Viewer | 🧪     | React or Godot WASM experiment (eventually)    |

---

## 📄 License

MIT (or dual MIT/Apache-2.0 TBD) — designed for open collaboration and long-term survivability.
