/// Libraries
use crate::{Graph, Result, TSPError};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

pub struct TSPSolver {
    graph: Graph,
    memo: HashMap<(usize, u64), (i32, Option<usize>)>, // (cost, next_node)
    verbose: bool,
    stats: SolverStats,
}

/// Stats
#[derive(Debug, Default)]
pub struct SolverStats {
    pub start_time: Option<Instant>,
    pub solve_duration: Option<Duration>,
    pub states_computed: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub max_memory_states: usize,
}

/// Result
#[derive(Debug)]
pub struct TSPSolution {
    pub optimal_cost: i32,
    pub optimal_path: Vec<usize>,
    pub stats: SolverStats,
    pub is_valid: bool,
}

impl TSPSolver {
    pub fn new(graph: Graph) -> Result<Self> {
        graph.validate_for_tsp()?;

        if graph.size < 2 {
            return Err(TSPError::SolverError(
                "Graf harus memiliki minimal 2 node".to_string()
            ));
        }

        if graph.size > 20 {
            return Err(TSPError::SolverError(
                "Graf terlalu besar (>20 node). Kompleksitas O(n²2ⁿ) akan terlalu tinggi".to_string()
            ));
        }

        Ok(TSPSolver {
            graph,
            memo: HashMap::new(),
            verbose: false,
            stats: SolverStats::default(),
        })
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn solve(&mut self) -> Result<TSPSolution> {
        self.stats.start_time = Some(Instant::now());
        self.memo.clear();
        self.stats = SolverStats {
            start_time: Some(Instant::now()),
            ..Default::default()
        };

        if self.verbose {
            println!("{}", "=== MEMULAI TSP SOLVER ===".bright_blue().bold());
            println!("Graf: {} node", self.graph.size);
            println!("Kompleksitas: O(n²2ⁿ) = O({}×2^{}) = O({})", 
                    self.graph.size * self.graph.size,
                    self.graph.size,
                    self.graph.size * self.graph.size * (1 << self.graph.size));
            println!();
        }

        // Phase 1: Basis - f(i, ∅) = c_i,0 for i = 1, 2, ..., n-1
        self.compute_base_cases()?;

        // Phase 2-n: Iterate for any subset size from 1 to n-1
        for subset_size in 1..self.graph.size - 1 {
            self.compute_subset_size(subset_size)?;
        }

        // Final Phase: Compute f(0, {1, 2, ..., n-1})
        let (optimal_cost, first_next) = self.compute_final_result()?;

        // Reconstruct
        let optimal_path = self.reconstruct_path(first_next)?;

        // Validate
        let is_valid = self.validate_solution(&optimal_path, optimal_cost)?;

        self.stats.solve_duration = self.stats.start_time.map(|t| t.elapsed());
        self.stats.max_memory_states = self.memo.len();

        Ok(TSPSolution {
            optimal_cost,
            optimal_path,
            stats: std::mem::take(&mut self.stats),
            is_valid,
        })
    }

    /// Compute base cases: f(i, ∅) = c_i,0
    fn compute_base_cases(&mut self) -> Result<()> {
        if self.verbose {
            println!("{}", "Tahap 1 - Basis: f(i, ∅) = c_i,0".yellow().bold());
        }

        for i in 1..self.graph.size {
            let cost = self.graph.get_distance(i, 0);
            self.memo.insert((i, 0), (cost, None));
            self.stats.states_computed += 1;

            if self.verbose {
                println!("f({}, ∅) = {} (jarak dari {} ke {})", 
                        i + 1, cost, 
                        self.graph.node_names[i].bright_cyan(),
                        self.graph.node_names[0].bright_cyan());
            }
        }

        if self.verbose {
            println!();
        }
        Ok(())
    }

    fn compute_subset_size(&mut self, subset_size: usize) -> Result<()> {
        if self.verbose {
            println!("{}", 
                    format!("Tahap {} - Subset berukuran {}:", subset_size + 1, subset_size)
                    .yellow().bold());
        }

        let available_nodes: Vec<usize> = (1..self.graph.size).collect();
        let subsets = self.generate_subsets(&available_nodes, subset_size);

        for subset in subsets {
            let mask = self.set_to_mask(&subset);
            
            for i in 1..self.graph.size {
                if subset.contains(&i) {
                    continue;
                }

                let result = self.compute_dp_state(i, &subset)?;
                if let Some((cost, next)) = result {
                    self.memo.insert((i, mask), (cost, next));
                    self.stats.states_computed += 1;

                    if self.verbose {
                        println!("f({}, {:?}) = {} (next: {:?})", 
                               i + 1, 
                               subset.iter().map(|&x| x + 1).collect::<Vec<_>>(),
                               cost,
                               next.map(|x| x + 1));
                    }
                }
            }
        }

        if self.verbose {
            println!();
        }
        Ok(())
    }

    /// Compute single DP state: f(i, S) = min{c_ij + f(j, S - {j})}
    fn compute_dp_state(&mut self, i: usize, subset: &HashSet<usize>) -> Result<Option<(i32, Option<usize>)>> {
        let mut min_cost = i32::MAX;
        let mut best_next = None;

        for &j in subset {
            let mut new_subset = subset.clone();
            new_subset.remove(&j);
            let new_mask = self.set_to_mask(&new_subset);

            let prev_result = if let Some(&result) = self.memo.get(&(j, new_mask)) {
                self.stats.cache_hits += 1;
                result
            } else {
                self.stats.cache_misses += 1;
                return Ok(None);
            };

            let total_cost = self.graph.get_distance(i, j) + prev_result.0;
            if total_cost < min_cost {
                min_cost = total_cost;
                best_next = Some(j);
            }
        }

        if min_cost == i32::MAX {
            Ok(None)
        } else {
            Ok(Some((min_cost, best_next)))
        }
    }

    /// Compute final: f(0, {1, 2, ..., n-1})
    fn compute_final_result(&mut self) -> Result<(i32, usize)> {
        let full_set: HashSet<usize> = (1..self.graph.size).collect();
        
        let mut min_cost = i32::MAX;
        let mut first_next = None;

        if self.verbose {
            println!("{}", 
                    format!("Tahap Final - Menghitung f(1, {{2,3,...,{}}}):", self.graph.size)
                    .green().bold());
        }

        for k in 1..self.graph.size {
            let mut reduced_set = full_set.clone();
            reduced_set.remove(&k);
            let reduced_mask = self.set_to_mask(&reduced_set);

            if let Some(&(prev_cost, _)) = self.memo.get(&(k, reduced_mask)) {
                let total_cost = self.graph.get_distance(0, k) + prev_cost;
                
                if self.verbose {
                    println!("  c_1,{} + f({}, subset) = {} + {} = {}", 
                           k + 1, k + 1, self.graph.get_distance(0, k), prev_cost, total_cost);
                }
                
                if total_cost < min_cost {
                    min_cost = total_cost;
                    first_next = Some(k);
                }
                self.stats.cache_hits += 1;
            } else {
                self.stats.cache_misses += 1;
            }
        }

        let first_next = first_next.ok_or_else(|| 
            TSPError::SolverError("Tidak dapat menemukan solusi optimal".to_string())
        )?;

        if self.verbose {
            println!("\n{} {}", "Biaya minimum tour:".green().bold(), 
                    min_cost.to_string().bright_green());
        }

        Ok((min_cost, first_next))
    }

    fn reconstruct_path(&self, start: usize) -> Result<Vec<usize>> {
        let mut path = vec![0, start];
        let mut current = start;
        let mut remaining: HashSet<usize> = (1..self.graph.size).collect();
        remaining.remove(&start);

        if self.verbose {
            println!("\n{}", "=== REKONSTRUKSI JALUR ===".green().bold());
            println!("Mulai dari: {} -> {}", 
                    self.graph.node_names[0].bright_cyan(),
                    self.graph.node_names[start].bright_cyan());
        }

        while !remaining.is_empty() {
            let mask = self.set_to_mask(&remaining);
            if let Some(&(_, Some(next))) = self.memo.get(&(current, mask)) {
                if self.verbose {
                    println!("Dari {} ke {}", 
                            self.graph.node_names[current].bright_cyan(),
                            self.graph.node_names[next].bright_cyan());
                }
                path.push(next);
                remaining.remove(&next);
                current = next;
            } else {
                return Err(TSPError::SolverError("Gagal merekonstruksi path".to_string()));
            }
        }

        path.push(0);
        if self.verbose {
            println!("Kembali ke: {}", self.graph.node_names[0].bright_cyan());
        }

        Ok(path)
    }

    fn validate_solution(&self, path: &[usize], expected_cost: i32) -> Result<bool> {
        if !self.graph.is_valid_tour(path) {
            return Ok(false);
        }

        let actual_cost = self.graph.calculate_path_cost(path)?;
        Ok(actual_cost == expected_cost)
    }

    fn set_to_mask(&self, set: &HashSet<usize>) -> u64 {
        let mut mask = 0u64;
        for &node in set {
            mask |= 1u64 << node;
        }
        mask
    }

    fn generate_subsets(&self, items: &[usize], size: usize) -> Vec<HashSet<usize>> {
        if size == 0 {
            return vec![HashSet::new()];
        }
        if size > items.len() {
            return vec![];
        }

        let mut result = Vec::new();
        self.generate_subsets_recursive(items, size, 0, &mut HashSet::new(), &mut result);
        result
    }

    fn generate_subsets_recursive(
        &self,
        items: &[usize],
        size: usize,
        start: usize,
        current: &mut HashSet<usize>,
        result: &mut Vec<HashSet<usize>>,
    ) {
        if current.len() == size {
            result.push(current.clone());
            return;
        }

        for i in start..items.len() {
            current.insert(items[i]);
            self.generate_subsets_recursive(items, size, i + 1, current, result);
            current.remove(&items[i]);
        }
    }

    pub fn get_stats(&self) -> &SolverStats {
        &self.stats
    }
}

impl std::fmt::Display for SolverStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== SOLVER STATISTICS ===")?;
        if let Some(duration) = self.solve_duration {
            writeln!(f, "Solve Time: {:.2?}", duration)?;
        }
        writeln!(f, "States Computed: {}", self.states_computed)?;
        writeln!(f, "Cache Hits: {}", self.cache_hits)?;
        writeln!(f, "Cache Misses: {}", self.cache_misses)?;
        writeln!(f, "Max Memory States: {}", self.max_memory_states)?;
        
        let cache_total = self.cache_hits + self.cache_misses;
        if cache_total > 0 {
            let hit_rate = self.cache_hits as f64 / cache_total as f64 * 100.0;
            writeln!(f, "Cache Hit Rate: {:.1}%", hit_rate)?;
        }
        
        Ok(())
    }
}

impl std::fmt::Display for TSPSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== TSP SOLUTION ===")?;
        writeln!(f, "Optimal Cost: {}", self.optimal_cost)?;
        writeln!(f, "Optimal Path: {}", 
                self.optimal_path.iter()
                    .map(|&i| (i + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(" -> "))?;
        writeln!(f, "Valid: {}", if self.is_valid { "Yes" } else { "No" })?;
        writeln!(f, "{}", self.stats)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_tsp() -> Result<()> {
        let matrix = vec![
            vec![0, 2, 3],
            vec![4, 0, 1],
            vec![5, 6, 0],
        ];

        let graph = Graph::new(matrix)?;
        let mut solver = TSPSolver::new(graph)?;
        let solution = solver.solve()?;

        assert!(solution.is_valid);
        assert_eq!(solution.optimal_path.len(), 4);
        assert_eq!(solution.optimal_path[0], 0);
        assert_eq!(solution.optimal_path[3], 0);

        Ok(())
    }

    #[test]
    fn test_document_example() -> Result<()> {
        let matrix = vec![
            vec![0, 10, 15, 20],
            vec![5,  0,  9, 10],
            vec![6, 13,  0, 12],
            vec![8,  8,  9,  0],
        ];

        let graph = Graph::new(matrix)?;
        let mut solver = TSPSolver::new(graph)?;
        let solution = solver.solve()?;

        assert!(solution.is_valid);
        assert_eq!(solution.optimal_cost, 35);

        Ok(())
    }
}