use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

use tsp_solver::{ConfigReader, Result, TSPSolver, Visualizer};

#[derive(Parser)]
#[command(name = "tsp_solver")]
#[command(about = "Travelling Salesman Problem Solver dengan Dynamic Programming")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        #[arg(short, long)]
        file: PathBuf,
        
        #[arg(short, long)]
        verbose: bool,
    },

    Sample {
        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long, default_value = "medium")]
        kind: String,
    },
    
    Validate {
        #[arg(short, long)]
        file: PathBuf,
    },
    
    Benchmark {
        #[arg(short, long, default_value = "8")]
        max_size: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    print_header();

    match cli.command {
        Commands::Solve { file, verbose } => {
            solve_from_file(file, verbose)
        }
        Commands::Sample { output, kind } => {
            create_sample_file(output, kind)
        }
        Commands::Validate { file } => {
            validate_file(file)
        }
        Commands::Benchmark { max_size } => {
            run_benchmark(max_size)
        }
    }
}

fn print_header() {
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                    TSP SOLVER v1.0                          ║".bright_blue());
    println!("{}", "║            Travelling Salesman Problem Solver               ║".bright_blue());
    println!("{}", "║              Dynamic Programming Algorithm                   ║".bright_blue());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_blue());
    println!();
}

fn solve_from_file(file_path: PathBuf, verbose: bool) -> Result<()> {
    println!("{} {:?}", "Membaca file:".bright_green(), file_path);
    
    let graph = ConfigReader::read_from_file(&file_path)?;
    
    println!("{}", "[SUCCESS] File berhasil dibaca".bright_green());
    
    graph.display();
    graph.visualize_ascii();
    
    Visualizer::display_complexity_info(graph.size);
    
    println!("{}", "[START] Memulai TSP Solver...".bright_yellow());
    let mut solver = TSPSolver::new(graph.clone())?.with_verbose(verbose);
    let solution = solver.solve()?;
    
    println!("{}", "[SUCCESS] Solusi ditemukan!".bright_green());
    
    // Tampilkan hasil
    Visualizer::display_solution(&graph, &solution);
    Visualizer::display_graph_with_tour(&graph, &solution.optimal_path);
    Visualizer::display_tour_sequence(&graph, &solution.optimal_path);
    
    Ok(())
}

fn create_sample_file(output_path: PathBuf, kind: String) -> Result<()> {
    use tsp_solver::config::SampleType;
    
    let sample_type = match kind.as_str() {
        "small" => SampleType::Small,
        "medium" => SampleType::Medium,
        "large" => SampleType::Large,
        _ => {
            println!("{} Jenis sample tidak dikenal: {}", "[ERROR]".bright_red(), kind);
            println!("Gunakan: small, medium, atau large");
            return Ok(());
        }
    };
    
    ConfigReader::create_sample_file(&output_path, sample_type)?;
    println!("{} Sample file berhasil dibuat: {:?}", "[SUCCESS]".bright_green(), output_path);
    
    Ok(())
}

fn validate_file(file_path: PathBuf) -> Result<()> {
    println!("{} {:?}", "Memvalidasi file:".bright_yellow(), file_path);
    
    match ConfigReader::validate_file(&file_path) {
        Ok(report) => {
            println!("{}", report);
        }
        Err(e) => {
            println!("{} {}", "[ERROR]".bright_red(), e);
        }
    }
    
    Ok(())
}

fn run_benchmark(max_size: usize) -> Result<()> {
    println!("{} TSP Solver untuk graf ukuran 3 hingga {}", 
            "[BENCHMARK]".bright_cyan(), max_size);
    
    if max_size > 12 {
        println!("{} Ukuran > 12 akan memakan waktu sangat lama!", 
                "[WARNING]".bright_red());
        println!("Apakah Anda yakin ingin melanjutkan? (y/N)");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Benchmark dibatalkan");
            return Ok(());
        }
    }
    
    println!("{} Benchmark memerlukan file test case di folder data/", "[INFO]".bright_blue());
    println!("Pastikan Anda sudah membuat sample files dengan menu 'Create Sample Files'");
    println!("Benchmark akan menggunakan sample files yang tersedia");
    
    Ok(())
}