# Numerical methods experiments

Run commands from the repository root, with the existing `table` and matrix/B
files available at the configured paths.

`MATRIX_NAME` selects the matrix by its name without `.mtx`, for example
`sherman1` or `494_bus`. The existing table marks `LONG` matrices and determines
which A directory is used. A `LONG` matrix runs three repetitions; other
matrices use `NUM_SHOTS` (currently ten).

```bash
cargo build --release --locked
mkdir -p data/out
MATRIX_NAME=sherman1 ./target/release/dumb-rematrix > results-sherman1.csv
```

## NPAD

Build once, then submit one matrix per job from the repository root:

```bash
sbatch --export=ALL,MATRIX_NAME=sherman1 run_matrix.sbatch
sbatch --export=ALL,MATRIX_NAME=494_bus run_matrix.sbatch
```

Pass partition, time and other allocation options to `sbatch` as appropriate
for the intended job. The supplied script requests one task and 64 CPUs,
matching the largest configured thread count. CSV output is saved as
`results-<matrix>-<job-id>.csv`; solution files use the existing `data/out/`
format.

## Experiment grid

Every repetition runs the configured thread counts and gaps for:

- Jacobi: classical and even external gap.
- Seidel: classical and even external gap.
- JASPIoM: classical, internal gap, external gap and mixed gaps.

Mixed JASPIoM uses the Cartesian product of `TESTING_GAPS` for its internal
and external gaps. Experiments run sequentially, with parallelism inside each
method. A and B are loaded once and shared across all runs for that matrix.
The existing B lookup remains `data/bs/<matrix-name>`, without `.mtx`.

With the current configuration, each repetition has 174 runs: 1,740 runs for
a regular matrix and 522 for a `LONG` matrix.

The CSV preserves the existing columns and appends `Shot`, `Gap In` and
`Gap Out`. The original `Gap` column contains the internal gap for internal-only
JASPIoM, and the external gap for external/mixed variants. Iteration counts
retain each method's current convention. Timing covers the solver call;
residual calculation and output happen afterward.
