// AI GENERATED:

use std::time::Duration;

pub struct ProblemSolutionConfig {
    pub len: usize,
    pub num_threads: usize,
    pub num_threads_c: usize,
    pub gap: usize,
    pub delayed: usize,
    pub sparsed: bool,
}

impl ProblemSolutionConfig {
    pub const HEADER: &'static str =
        "Matrix Size,Num Threads,Num Threads Checkup,Gap,Delayed,Sparsed";

    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.len, self.num_threads, self.num_threads_c, self.gap, self.delayed, self.sparsed
        )
    }
}

pub struct PerformanceAndQuality {
    pub num_iterations: u64,
    pub time_spent: Duration,
    pub final_residual: f64,
    pub converged: bool,
}

impl PerformanceAndQuality {
    pub const HEADER: &'static str = "Num Iterations,Time Spent,Final Residual,Converged";

    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{}",
            self.num_iterations,
            self.time_spent.as_secs_f64(),
            self.final_residual,
            self.converged
        )
    }
}

pub struct Metadata {
    pub id: u8,
    pub algorithm_name: String,
    pub file_name: String,
}

impl Metadata {
    pub const HEADER: &'static str = "Id,Algorithm Name,File Name";

    pub fn to_csv_line(&self) -> String {
        format!("{},{},{}", self.id, self.algorithm_name, self.file_name)
    }
}

pub struct ExperimentResult {
    pub metadata: Metadata,
    pub problem_solution_config: ProblemSolutionConfig,
    pub performance_and_quality: PerformanceAndQuality,
}

impl ExperimentResult {
    pub const HEADER: &'static str = concat!(
        "Id,Algorithm Name,File Name",
        ",",
        "Matrix Size,Num Threads,Num Threads Checkup,Gap,Delayed,Sparsed",
        ",",
        "Num Iterations,Time Spent,Final Residual,Converged"
    );

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{}",
            self.metadata.to_csv_line(),
            self.problem_solution_config.to_csv_line(),
            self.performance_and_quality.to_csv_line()
        )
    }
}
