/// Libraries
use crate::{Graph, Result, TSPError};
use std::fs;
use std::path::Path;

pub struct ConfigReader;

impl ConfigReader {
    /// Format:
    /// ```
    /// # Komentar dimulai dengan #
    /// # Nama kota (opsional)
    /// CITIES: Kota_A, Kota_B, Kota_C, Kota_D
    /// 
    /// # Matriks adjacency
    /// MATRIX:
    /// 0 10 15 20
    /// 5  0  9 10
    /// 6 13  0 12
    /// 8  8  9  0
    /// ```
    pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> Result<Graph> {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| TSPError::ConfigError(
                format!("Gagal membaca file {:?}: {}", file_path.as_ref(), e)
            ))?;

        Self::parse_content(&content)
    }

    /// Parser
    pub fn parse_content(content: &str) -> Result<Graph> {
        let lines: Vec<&str> = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();

        if lines.is_empty() {
            return Err(TSPError::ConfigError("File kosong atau hanya berisi komentar".to_string()));
        }

        let mut city_names: Option<Vec<String>> = None;
        let mut matrix: Option<Vec<Vec<i32>>> = None;
        let mut in_matrix_section = false;

        for line in lines {
            if line.starts_with("CITIES:") {
                let cities_str = line.strip_prefix("CITIES:").unwrap().trim();
                city_names = Some(
                    cities_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                );
            } else if line == "MATRIX:" {
                in_matrix_section = true;
                matrix = Some(Vec::new());
            } else if in_matrix_section {
                if let Some(ref mut mat) = matrix {
                    let row = Self::parse_matrix_row(line)?;
                    mat.push(row);
                }
            }
        }

        let adjacency_matrix = matrix.ok_or_else(|| 
            TSPError::ConfigError("Tidak ditemukan section MATRIX".to_string())
        )?;

        if adjacency_matrix.is_empty() {
            return Err(TSPError::ConfigError("Matrix tidak boleh kosong".to_string()));
        }

        match city_names {
            Some(names) => Graph::with_node_names(adjacency_matrix, names),
            None => Graph::new(adjacency_matrix),
        }
    }

    fn parse_matrix_row(line: &str) -> Result<Vec<i32>> {
        line.split_whitespace()
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|_| TSPError::ConfigError(
                        format!("Gagal parse angka: '{}'", s)
                    ))
            })
            .collect()
    }

    pub fn create_sample_file<P: AsRef<Path>>(file_path: P, sample_type: SampleType) -> Result<()> {
        let content = match sample_type {
            SampleType::Small => Self::get_small_sample(),
            SampleType::Medium => Self::get_medium_sample(),
            SampleType::Large => Self::get_large_sample(),
        };

        fs::write(&file_path, content)
            .map_err(|e| TSPError::ConfigError(
                format!("Gagal menulis file {:?}: {}", file_path.as_ref(), e)
            ))?;

        Ok(())
    }

    fn get_small_sample() -> String {
        r#"# Contoh TSP sederhana - 3 kota

CITIES: A, B, C

MATRIX:
0 2 3
4 0 1
5 6 0
"#.to_string()
    }

    fn get_medium_sample() -> String {
        r#"# Contoh TSP medium - 5 kota

CITIES: Jakarta, Bandung, Yogyakarta, Surabaya, Medan

# Matriks jarak
MATRIX:
0  2  9 10  7
1  0  6  4  3
15 7  0  8  3
6  3 12  0 11
7  8  4  2  0
"#.to_string()
    }

    fn get_large_sample() -> String {
        r#"# Contoh TSP large - 7 kota

CITIES: A, B, C, D, E, F, G

# Matriks jarak
MATRIX:
0  3  4  2  7  6  8
2  0  5  3  6  4  7
4  5  0  6  8  9  3
2  3  6  0  4  5  9
7  6  8  4  0  2  5
6  4  9  5  2  0  7
8  7  3  9  5  7  0
"#.to_string()
    }
    pub fn validate_file<P: AsRef<Path>>(file_path: P) -> Result<ValidationReport> {
        let graph = Self::read_from_file(&file_path)?;
        let stats = graph.get_stats();
        
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        if let Err(e) = graph.validate_for_tsp() {
            errors.push(format!("Graf tidak valid untuk TSP: {}", e));
        }

        if graph.size > 10 {
            warnings.push(format!(
                "Graf berukuran {} mungkin membutuhkan waktu komputasi yang lama", 
                graph.size
            ));
        }

        if stats.max_edge_weight > stats.min_edge_weight * 10 {
            warnings.push("Bobot edge sangat tidak seimbang, ini mungkin mempengaruhi performa".to_string());
        }

        let is_valid = errors.is_empty();
        
        Ok(ValidationReport {
            graph_size: graph.size,
            stats,
            warnings,
            errors,
            is_valid,
        })
    }
}

#[derive(Debug, Clone)]
pub enum SampleType {
    Small,     // Kecil (3 kota)
    Medium,    // Medium (5 kota)
    Large,     // Large (7 kota)
}

#[derive(Debug)]
pub struct ValidationReport {
    pub graph_size: usize,
    pub stats: crate::graph::GraphStats,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub is_valid: bool,
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== VALIDATION REPORT ===")?;
        writeln!(f, "Graph Size: {}", self.graph_size)?;
        writeln!(f, "Stats: {}", self.stats)?;
        
        if !self.warnings.is_empty() {
            writeln!(f, "\nWarnings:")?;
            for warning in &self.warnings {
                writeln!(f, "  [WARNING] {}", warning)?;
            }
        }

        if !self.errors.is_empty() {
            writeln!(f, "\nErrors:")?;
            for error in &self.errors {
                writeln!(f, "  [ERROR] {}", error)?;
            }
        }

        writeln!(f, "\nStatus: {}", 
                if self.is_valid { "[VALID]" } else { "[INVALID]" })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_matrix() {
        let content = r#"
# Test matrix
MATRIX:
0 1 2
3 0 4
5 6 0
"#;

        let graph = ConfigReader::parse_content(content).unwrap();
        assert_eq!(graph.size, 3);
        assert_eq!(graph.get_distance(0, 1), 1);
        assert_eq!(graph.get_distance(1, 2), 4);
    }

    #[test]
    fn test_parse_with_cities() {
        let content = r#"
CITIES: A, B, C
MATRIX:
0 1 2
3 0 4
5 6 0
"#;

        let graph = ConfigReader::parse_content(content).unwrap();
        assert_eq!(graph.node_names, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_invalid_matrix() {
        let content = r#"
MATRIX:
0 1
2 0 3
"#;

        assert!(ConfigReader::parse_content(content).is_err());
    }
}