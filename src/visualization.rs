// Libraries
use crate::{Graph, tsp_solver::TSPSolution};
use colored::*;
use prettytable::{Cell, Row, Table};
use std::collections::HashSet;

pub struct Visualizer;

impl Visualizer {
    pub fn display_solution(graph: &Graph, solution: &TSPSolution) {
        println!("{}", "=== SOLUSI TSP ===".bright_blue().bold());
        
        // Header info
        println!("Graf: {} kota", graph.size.to_string().bright_green());
        println!("Status: {}", 
                if solution.is_valid { 
                    "[VALID]".bright_green() 
                } else { 
                    "[INVALID]".bright_red() 
                });
        println!();

        // Optimal path
        println!("{}", "Jalur Optimal:".bright_yellow().bold());
        let path_str = solution.optimal_path
            .iter()
            .map(|&i| graph.node_names[i].bright_cyan().to_string())
            .collect::<Vec<_>>()
            .join(" → ");
        println!("  {}", path_str);
        println!();

        // Optimal cost
        println!("{} {}", 
                "Biaya Total:".bright_yellow().bold(), 
                solution.optimal_cost.to_string().bright_green().bold());
        println!();

        Self::display_journey_details(graph, &solution.optimal_path, solution.optimal_cost);
        
        println!("{}", solution.stats);
    }

    fn display_journey_details(graph: &Graph, path: &[usize], total_cost: i32) {
        println!("{}", "Detail Perjalanan:".bright_yellow().bold());
        
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Step").style_spec("Fb"),
            Cell::new("Dari").style_spec("Fb"),
            Cell::new("Ke").style_spec("Fb"),
            Cell::new("Jarak").style_spec("Fb"),
            Cell::new("Kumulatif").style_spec("Fb"),
        ]));

        let mut cumulative_cost = 0;
        for (i, window) in path.windows(2).enumerate() {
            let from = window[0];
            let to = window[1];
            let distance = graph.get_distance(from, to);
            cumulative_cost += distance;

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(&graph.node_names[from]),
                Cell::new(&graph.node_names[to]),
                Cell::new(&distance.to_string()).style_spec("Fr"),
                Cell::new(&cumulative_cost.to_string()).style_spec("Fg"),
            ]));
        }

        table.add_row(Row::new(vec![
            Cell::new("TOTAL").style_spec("Fb"),
            Cell::new("").style_spec(""),
            Cell::new("").style_spec(""),
            Cell::new("").style_spec(""),
            Cell::new(&total_cost.to_string()).style_spec("FgB"),
        ]));

        table.printstd();
        println!();
    }

    pub fn display_graph_with_tour(graph: &Graph, path: &[usize]) {
        println!("{}", "=== GRAF DENGAN TOUR OPTIMAL ===".bright_blue().bold());

        let mut tour_edges = HashSet::new();
        for window in path.windows(2) {
            tour_edges.insert((window[0], window[1]));
        }

        println!("Koneksi Graf:");
        println!("  {} = Edge biasa", "---".white());
        println!("  {} = Edge dalam tour optimal", "===".bright_green().bold());
        println!();

        for i in 0..graph.size {
            for j in 0..graph.size {
                if i != j && graph.adjacency_matrix[i][j] > 0 {
                    let weight = graph.adjacency_matrix[i][j];
                    let line = format!("  {} {} {} ({})", 
                                     graph.node_names[i],
                                     if tour_edges.contains(&(i, j)) { "===" } else { "---" },
                                     graph.node_names[j],
                                     weight);
                    
                    if tour_edges.contains(&(i, j)) {
                        println!("{}", line.bright_green().bold());
                    } else {
                        println!("{}", line.white());
                    }
                }
            }
        }
        println!();
    }

    /// Sequence
    pub fn display_tour_sequence(graph: &Graph, path: &[usize]) {
        println!("{}", "=== URUTAN KUNJUNGAN ===".bright_blue().bold());
        
        for (step, &node) in path.iter().enumerate() {
            let action = match step {
                0 => "Mulai di",
                s if s == path.len() - 1 => "Kembali ke",
                _ => "Kunjungi",
            };

            println!("{}. {} {}", 
                    step + 1, 
                    action, 
                    graph.node_names[node].bright_cyan().bold());
        }
        println!();
    }

    pub fn display_comparison(solutions: &[(&str, &TSPSolution)]) {
        if solutions.len() < 2 {
            return;
        }

        println!("{}", "=== PERBANDINGAN SOLUSI ===".bright_blue().bold());
        
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Metode").style_spec("Fb"),
            Cell::new("Biaya").style_spec("Fb"),
            Cell::new("Waktu").style_spec("Fb"),
            Cell::new("States").style_spec("Fb"),
            Cell::new("Valid").style_spec("Fb"),
        ]));

        let mut best_cost = i32::MAX;
        for (_, solution) in solutions {
            if solution.optimal_cost < best_cost {
                best_cost = solution.optimal_cost;
            }
        }

        for (name, solution) in solutions {
            let cost_cell = if solution.optimal_cost == best_cost {
                Cell::new(&format!("{} [BEST]", solution.optimal_cost)).style_spec("FgB")
            } else {
                Cell::new(&solution.optimal_cost.to_string())
            };

            let time_str = solution.stats.solve_duration
                .map(|d| format!("{:.2?}", d))
                .unwrap_or_else(|| "N/A".to_string());

            let valid_str = if solution.is_valid { "[VALID]" } else { "[INVALID]" };

            table.add_row(Row::new(vec![
                Cell::new(name),
                cost_cell,
                Cell::new(&time_str),
                Cell::new(&solution.stats.states_computed.to_string()),
                Cell::new(valid_str),
            ]));
        }

        table.printstd();
        println!();
    }

    /// Result
    pub fn display_performance_summary(solutions: &[(&str, &TSPSolution)]) {
        println!("{}", "=== RINGKASAN PERFORMA ===".bright_blue().bold());
        
        if solutions.is_empty() {
            println!("Tidak ada solusi untuk ditampilkan");
            return;
        }

        let best_solution = solutions
            .iter()
            .min_by_key(|(_, sol)| sol.optimal_cost)
            .unwrap();

        println!("[BEST] {}: {}", 
                best_solution.0.bright_cyan(),
                best_solution.1.optimal_cost.to_string().bright_green());

        if let Some(duration) = best_solution.1.stats.solve_duration {
            println!("Time: {:.2?}", duration);
        }

        println!("States Computed: {}", best_solution.1.stats.states_computed);
        
        let cache_total = best_solution.1.stats.cache_hits + best_solution.1.stats.cache_misses;
        if cache_total > 0 {
            let hit_rate = best_solution.1.stats.cache_hits as f64 / cache_total as f64 * 100.0;
            println!("Cache Hit Rate: {:.1}%", hit_rate);
        }

        println!();
    }

    pub fn display_complexity_info(graph_size: usize) {
        println!("{}", "=== INFORMASI KOMPLEKSITAS ===".bright_blue().bold());
        
        let n = graph_size;
        let states = n * (1 << (n - 1));
        let complexity = format!("O(n²×2ⁿ) = O({}²×2^{}) = O({})", n, n, n * n * (1 << n));
        
        println!("Ukuran Graf: {} node", n.to_string().bright_green());
        println!("Jumlah States: {}", states.to_string().bright_yellow());
        println!("Kompleksitas: {}", complexity.bright_red());

        if n > 15 {
            println!("[WARNING] Graf berukuran > 15 memerlukan waktu komputasi yang sangat lama"); 
        } else if n > 10 {
            println!("[NOTICE] Graf berukuran > 10 memerlukan waktu komputasi yang cukup lama");
        }
        
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Graph, TSPSolver};

    #[test]
    fn test_visualization_with_small_graph() {
        let matrix = vec![
            vec![0, 2, 3],
            vec![4, 0, 1],
            vec![5, 6, 0],
        ];

        let graph = Graph::new(matrix).unwrap();
        let mut solver = TSPSolver::new(graph.clone()).unwrap();
        let solution = solver.solve().unwrap();

        Visualizer::display_solution(&graph, &solution);
        Visualizer::display_graph_with_tour(&graph, &solution.optimal_path);
        Visualizer::display_tour_sequence(&graph, &solution.optimal_path);
    }
}