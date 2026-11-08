#!/usr/bin/env bash
#SBATCH --job-name=numerical-methods
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=64
#SBATCH --output=slurm-%x-%j.log

set -e

mkdir -p data/out
srun ./target/release/dumb-rematrix > "results-${MATRIX_NAME}-${SLURM_JOB_ID}.csv"
