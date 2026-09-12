# Metaheuristic TSP Solver (Rust)

A high-performance command-line solver for the **Traveling Salesperson Problem (TSP)** implemented in **Rust**. It features robust implementations of **Simulated Annealing (SA)** and **Tabu Search (TS)** utilizing optimized **2-opt neighborhood** local search heuristics.

Data sourced from the [University of Waterloo TSP World Tour Collection](https://www.math.uwaterloo.ca/tsp/world/countries.html).

---

## Code Architecture & Module Structure

The project is structured with a clean separation of concerns, separating algorithmic logic, mathematical utilities, and I/O management:

```text
src/
├── math_util.rs          # Distance matrix generation, 2-opt operations, and fast delta evaluations
├── annealing.rs          # Cooling schedules, parameters, and temperature update rules
├── simulate_annealing.rs # Simulated Annealing control loop and acceptance criteria
└── tabu_search.rs        # Tabu Search exploration loop and tenure management

```

### Key Components:

* **`math_util.rs` (The Core Engine):** Handles spatial distance computations using Euclidean metrics (`EUC_2D`). It implements the **2-opt swap operator**, cutting and reversing paths to eliminate crossing edges. Crucially, it computes fast **delta changes** rather than re-evaluating the entire tour cost from scratch at each step.
* **`simulate_annealing.rs` & `annealing.rs`:** Implements probabilistic local search escape mechanisms. It features a logarithmic cooling schedule ($T(t, n) = k \cdot \ln(n) \cdot e^{-1/\ln(n)}$), dynamic epoch length scaling based on instance size ($N$), and Metropolis acceptance criteria for suboptimal moves.
* **`tabu_search.rs`:** Extends standard local search by maintaining a short-term memory (tabu tenure proportional to $0.2N$) to prevent cycling and forcefully guide the search out of local optima.

---

## Benchmark Datasets

| Name | Cities ($N$) | File Name |
| --- | --- | --- |
| Western Sahara | 29 | `wi29.tsp` |
| Djibouti | 38 | `dj38.tsp` |
| Qatar | 194 | `qa194.tsp` |
| Uruguay | 734 | `uy734.tsp` |
| Zimbabwe | 929 | `zi929.tsp` |
| Oman | 1,979 | `mu1979.tsp` |
| Canada | 4,663 | `ca4663.tsp` |
| Tanzania | 6,117 | `tz6117.tsp` |
| Egypt | 7,146 | `eg7146.tsp` |
| Ireland | 8,246 | `ei8246.tsp` |

---

## Getting Started

### Prerequisites

* [Rust toolchain](https://rustup.rs/) (Cargo) installed.

### Running the Solver

```bash
# Clone the repository
git clone [https://github.com/YourUsername/your-repo-name.git](https://github.com/YourUsername/your-repo-name.git)
cd your-repo-name

# Build and run in release mode for maximum performance
cargo run --release

```

---

## Input Format (`.tsp`)

Input files follow the standard TSPLIB format containing spatial coordinates for each node:

```tsp
NAME : wi29
COMMENT : 29 locations in Western Sahara
COMMENT : Derived from National Imagery and Mapping Agency data
TYPE : TSP
DIMENSION : 29
EDGE_WEIGHT_TYPE : EUC_2D
NODE_COORD_SECTION
1 20833.3333 17100.0000
2 20900.0000 17066.6667
3 21300.0000 13016.6667
...
EOF

```

---

## Output Structure & Format (`.tour`)

The optimization results are serialized into tour files and stored in the `outputs/` directory:

```text
outputs/
├── BT_wi29.tour
├── BT_dj38.tour
...

```

Each tour file lists the sequence of visited node IDs in order, terminated by `-1`:

```text
NAME : BT_wi29
TYPE : TOUR 
DIMENSION : 29 
TOUR_SECTION 
1 
3 
5 
4 
...
29 
-1 
EOF

```
