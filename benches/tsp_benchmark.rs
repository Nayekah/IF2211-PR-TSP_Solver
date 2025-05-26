use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tsp_solver::{Graph, TSPSolver};

fn create_test_graph(size: usize) -> Graph {
    let mut matrix = vec![vec![0; size]; size];
    
    for i in 0..size {
        for j in 0..size {
            if i != j {
                let distance = ((i * 7 + j * 11) % 19) + 1;
                matrix[i][j] = distance as i32;
            }
        }
    }
    
    Graph::new(matrix).unwrap()
}

fn benchmark_tsp_3(c: &mut Criterion) {
    let graph = create_test_graph(3);
    
    c.bench_function("tsp_3_nodes", |b| {
        b.iter(|| {
            let mut solver = TSPSolver::new(black_box(graph.clone())).unwrap();
            solver.solve().unwrap()
        })
    });
}

fn benchmark_tsp_4(c: &mut Criterion) {
    let graph = create_test_graph(4);
    
    c.bench_function("tsp_4_nodes", |b| {
        b.iter(|| {
            let mut solver = TSPSolver::new(black_box(graph.clone())).unwrap();
            solver.solve().unwrap()
        })
    });
}

fn benchmark_tsp_5(c: &mut Criterion) {
    let graph = create_test_graph(5);
    
    c.bench_function("tsp_5_nodes", |b| {
        b.iter(|| {
            let mut solver = TSPSolver::new(black_box(graph.clone())).unwrap();
            solver.solve().unwrap()
        })
    });
}

fn benchmark_tsp_6(c: &mut Criterion) {
    let graph = create_test_graph(6);
    
    c.bench_function("tsp_6_nodes", |b| {
        b.iter(|| {
            let mut solver = TSPSolver::new(black_box(graph.clone())).unwrap();
            solver.solve().unwrap()
        })
    });
}

fn benchmark_document_example(c: &mut Criterion) {
    let adjacency_matrix = vec![
        vec![0, 10, 15, 20],
        vec![5,  0,  9, 10],
        vec![6, 13,  0, 12],
        vec![8,  8,  9,  0],
    ];
    
    let graph = Graph::new(adjacency_matrix).unwrap();
    
    c.bench_function("tsp_document_example", |b| {
        b.iter(|| {
            let mut solver = TSPSolver::new(black_box(graph.clone())).unwrap();
            solver.solve().unwrap()
        })
    });
}

fn benchmark_graph_creation(c: &mut Criterion) {
    let matrix = vec![
        vec![0, 10, 15, 20],
        vec![5,  0,  9, 10],
        vec![6, 13,  0, 12],
        vec![8,  8,  9,  0],
    ];
    
    c.bench_function("graph_creation", |b| {
        b.iter(|| {
            Graph::new(black_box(matrix.clone())).unwrap()
        })
    });
}

criterion_group!(
    benches,
    benchmark_tsp_3,
    benchmark_tsp_4,
    benchmark_tsp_5,
    benchmark_tsp_6,
    benchmark_document_example,
    benchmark_graph_creation
);

criterion_main!(benches);