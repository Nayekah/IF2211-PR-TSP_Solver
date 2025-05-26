/// Libraries
use crate::{Result, TSPError};
use colored::*;
use prettytable::{Cell, Row, Table};

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: Vec<usize>,
    pub adjacency_matrix: Vec<Vec<i32>>,
    pub size: usize,
    pub node_names: Vec<String>,
}

impl Graph {
    /// Adjacency matrix (yeah, I know, it's not the most efficient way to represent a graph)
    pub fn new(adjacency_matrix: Vec<Vec<i32>>) -> Result<Self> {
        let size = adjacency_matrix.len();
        
        // Validate
        if size == 0 {
            return Err(TSPError::InvalidGraph("Matrix tidak boleh kosong".to_string()));
        }

        for (i, row) in adjacency_matrix.iter().enumerate() {
            if row.len() != size {
                return Err(TSPError::InvalidGraph(
                    format!("Matrix harus persegi. Baris {} memiliki {} kolom, diharapkan {}", 
                            i, row.len(), size)
                ));
            }
        }

        for i in 0..size {
            if adjacency_matrix[i][i] != 0 {
                return Err(TSPError::InvalidGraph(
                    format!("Jarak dari node {} ke dirinya sendiri harus 0", i + 1)
                ));
            }
        }

        let nodes = (0..size).collect();
        let node_names: Vec<String> = (1..=size).map(|i| format!("Kota_{}", i)).collect();
        
        Ok(Graph {
            nodes,
            adjacency_matrix,
            size,
            node_names,
        })
    }

    /// Adding node names to the graph
    pub fn with_node_names(adjacency_matrix: Vec<Vec<i32>>, names: Vec<String>) -> Result<Self> {
        let mut graph = Self::new(adjacency_matrix)?;
        
        if names.len() != graph.size {
            return Err(TSPError::InvalidGraph(
                format!("Jumlah nama ({}) tidak sesuai dengan ukuran graf ({})", 
                        names.len(), graph.size)
            ));
        }
        
        graph.node_names = names;
        Ok(graph)
    }

    /// Distances between two nodes
    pub fn get_distance(&self, from: usize, to: usize) -> i32 {
        if from >= self.size || to >= self.size {
            panic!("Node index out of bounds: from={}, to={}, size={}", from, to, self.size);
        }
        self.adjacency_matrix[from][to]
    }

    pub fn validate_for_tsp(&self) -> Result<()> {
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self.adjacency_matrix[i][j] <= 0 {
                    return Err(TSPError::InvalidGraph(
                        format!("Tidak ada path dari {} ke {} (nilai: {})", 
                                self.node_names[i], self.node_names[j], 
                                self.adjacency_matrix[i][j])
                    ));
                }
            }
        }
        Ok(())
    }

    /// Adjacency matrix (table display, hehe)
    pub fn display(&self) {
        println!("{}", "=== GRAF TSP ===".bright_blue().bold());
        println!("Jumlah kota: {}", self.size.to_string().bright_green());
        println!();

        let mut table = Table::new();
        
        let mut header = vec![Cell::new("Dari/Ke")];
        for name in &self.node_names {
            header.push(Cell::new(name).style_spec("Fc"));
        }
        table.add_row(Row::new(header));

        for (i, row) in self.adjacency_matrix.iter().enumerate() {
            let mut table_row = vec![Cell::new(&self.node_names[i]).style_spec("Fb")];
            for (j, &val) in row.iter().enumerate() {
                let cell_content = if i == j {
                    "0".dimmed().to_string()
                } else if val == i32::MAX || val <= 0 {
                    "∞".red().to_string()
                } else {
                    val.to_string()
                };
                table_row.push(Cell::new(&cell_content));
            }
            table.add_row(Row::new(table_row));
        }

        table.printstd();
        println!();
    }

    pub fn visualize_ascii(&self) {
        println!("{}", "=== REPRESENTASI GRAF ===".bright_blue().bold());
        println!("Edge yang tersedia:");
        
        let mut edge_count = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self.adjacency_matrix[i][j] > 0 {
                    println!("  {} {} {} {}", 
                           self.node_names[i].bright_cyan(),
                           "-->".yellow(),
                           self.node_names[j].bright_cyan(),
                           format!("({})", self.adjacency_matrix[i][j]).green());
                    edge_count += 1;
                }
            }
        }
        println!("Total edges: {}", edge_count.to_string().bright_green());
        println!();
    }

    /// Total cost of a path
    pub fn calculate_path_cost(&self, path: &[usize]) -> Result<i32> {
        if path.len() < 2 {
            return Err(TSPError::InvalidPath("Path harus memiliki minimal 2 node".to_string()));
        }

        let mut total_cost = 0;
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[i + 1];
            
            if from >= self.size || to >= self.size {
                return Err(TSPError::InvalidPath(
                    format!("Node index out of bounds: {}, {}", from, to)
                ));
            }

            let cost = self.get_distance(from, to);
            if cost <= 0 {
                return Err(TSPError::InvalidPath(
                    format!("Tidak ada edge dari {} ke {}", 
                            self.node_names[from], self.node_names[to])
                ));
            }
            total_cost += cost;
        }

        Ok(total_cost)
    }

    /// Hamilton tour validation
    pub fn is_valid_tour(&self, path: &[usize]) -> bool {
        if path.len() != self.size + 1 {
            return false;
        }

        if path[0] != path[path.len() - 1] {
            return false;
        }

        let mut visited = vec![false; self.size];
        for &node in &path[..path.len() - 1] {
            if node >= self.size || visited[node] {
                return false;
            }
            visited[node] = true;
        }

        visited.iter().all(|&v| v)
    }

    /// Stats
    pub fn get_stats(&self) -> GraphStats {
        let mut total_edges = 0;
        let mut min_edge = i32::MAX;
        let mut max_edge = 0;
        let mut total_weight = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self.adjacency_matrix[i][j] > 0 {
                    let weight = self.adjacency_matrix[i][j];
                    total_edges += 1;
                    total_weight += weight;
                    min_edge = min_edge.min(weight);
                    max_edge = max_edge.max(weight);
                }
            }
        }

        let avg_edge = if total_edges > 0 {
            total_weight as f64 / total_edges as f64
        } else {
            0.0
        };

        GraphStats {
            nodes: self.size,
            edges: total_edges,
            min_edge_weight: if min_edge == i32::MAX { 0 } else { min_edge },
            max_edge_weight: max_edge,
            avg_edge_weight: avg_edge,
            total_weight,
        }
    }
}


/// Stats (return summary of the graph)
#[derive(Debug)]
pub struct GraphStats {
    pub nodes: usize,
    pub edges: usize,
    pub min_edge_weight: i32,
    pub max_edge_weight: i32,
    pub avg_edge_weight: f64,
    pub total_weight: i32,
}

impl std::fmt::Display for GraphStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Nodes: {}, Edges: {}, Min Weight: {}, Max Weight: {}, Avg Weight: {:.2}, Total Weight: {}",
            self.nodes, self.edges, self.min_edge_weight, 
            self.max_edge_weight, self.avg_edge_weight, self.total_weight
        )
    }
}