#!/bin/bash

echo "========================================"
echo "      TSP Solver - Linux Script"
echo "========================================"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust/Cargo not found! Please install Rust from https://rustup.rs/"
    read -p "Press Enter to exit..."
    exit 1
fi

# Check if data folder exists
if [ ! -d "data" ]; then
    echo "[WARNING] data/ folder not found. Creating it..."
    mkdir data
fi

# Build the project
echo "Building TSP Solver..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "[ERROR] Build failed!"
    read -p "Press Enter to exit..."
    exit 1
fi

echo "[SUCCESS] Build successful!"
echo

# Function to create sample files
create_sample_in_data() {
    echo "Creating sample files in data/ directory..."
    cargo run --release sample -o "data/sample_small.txt" -k small
    cargo run --release sample -o "data/sample_medium.txt" -k medium  
    cargo run --release sample -o "data/sample_large.txt" -k large
    echo
    echo "[SUCCESS] Sample files created in data/:"
    echo "- sample_small.txt (3 cities - small example)"
    echo "- sample_medium.txt (5 cities - medium example)"
    echo "- sample_large.txt (7 cities - large example)"
}

# Main menu loop
while true; do
    echo "========================================"
    echo "           TSP Solver Menu"
    echo "========================================"
    echo "1. List Test Cases in data/"
    echo "2. Solve Specific Test Case"
    echo "3. Solve All Test Cases"
    echo "4. Create Sample Files"
    echo "5. Validate Test Case"
    echo "6. Run Benchmark"
    echo "7. Help/Usage"
    echo "8. Exit"
    echo "========================================"
    read -p "Enter your choice (1-8): " choice

    case $choice in
        1)
            echo
            echo "Test Cases in data/ folder:"
            echo "========================================"
            if ls data/*.txt 1> /dev/null 2>&1; then
                ls -1 data/*.txt | sed 's|data/||'
                echo
                echo "Total files:"
                ls -1 data/*.txt 2>/dev/null | wc -l
            else
                echo "[INFO] No .txt files found in data/ folder"
                echo
                echo "Creating sample test cases..."
                create_sample_in_data
            fi
            echo
            read -p "Press Enter to continue..."
            ;;
        2)
            echo
            echo "Solve Specific Test Case"
            echo "========================================"
            echo "Available test cases:"
            if ls data/*.txt 1> /dev/null 2>&1; then
                ls -1 data/*.txt | sed 's|data/||'
            else
                echo "[INFO] No test cases found in data/ folder"
                echo "Creating sample files first..."
                create_sample_in_data
                echo
                echo "Available test cases:"
                ls -1 data/*.txt | sed 's|data/||'
            fi
            echo
            read -p "Enter test case filename (without data/ prefix): " filename
            if [ ! -f "data/$filename" ]; then
                echo "[ERROR] File 'data/$filename' not found!"
                echo
                read -p "Press Enter to continue..."
                continue
            fi
            echo
            read -p "Do you want verbose output? (y/N): " verbose_choice
            verbose_flag=""
            if [[ "$verbose_choice" =~ ^[Yy] ]]; then
                verbose_flag="--verbose"
            fi

            echo
            echo "[START] Solving TSP from data/$filename..."
            cargo run --release solve -f "data/$filename" $verbose_flag
            echo
            read -p "Press Enter to continue..."
            ;;
        3)
            echo
            echo "Solve All Test Cases"
            echo "========================================"
            echo "[INFO] Processing all .txt files in data/ folder..."
            echo
            if ! ls data/*.txt 1> /dev/null 2>&1; then
                echo "[INFO] No test cases found! Creating sample files first..."
                create_sample_in_data
                echo
            fi

            for file in data/*.txt; do
                if [ -f "$file" ]; then
                    echo "========================================"
                    echo "[START] Solving: $file"
                    echo "========================================"
                    cargo run --release solve -f "$file" --verbose
                    echo
                    echo "[SUCCESS] Completed: $file"
                    echo
                    sleep 2
                fi
            done
            echo "[SUCCESS] All test cases completed!"
            echo
            read -p "Press Enter to continue..."
            ;;
        4)
            echo
            echo "Creating Sample Files in data/"
            echo "========================================"
            create_sample_in_data
            echo
            read -p "Press Enter to continue..."
            ;;
        5)
            echo
            echo "Validate Test Case"
            echo "========================================"
            echo "Available test cases:"
            if ls data/*.txt 1> /dev/null 2>&1; then
                ls -1 data/*.txt | sed 's|data/||'
            else
                echo "[INFO] No test cases found in data/ folder"
                echo "Creating sample files first..."
                create_sample_in_data
                echo
                echo "Available test cases:"
                ls -1 data/*.txt | sed 's|data/||'
            fi
            echo
            read -p "Enter filename to validate (without data/ prefix): " filename
            if [ ! -f "data/$filename" ]; then
                echo "[ERROR] File 'data/$filename' not found!"
                echo
                read -p "Press Enter to continue..."
                continue
            fi
            echo
            echo "[START] Validating data/$filename..."
            cargo run --release validate -f "data/$filename"
            echo
            read -p "Press Enter to continue..."
            ;;
        6)
            echo
            echo "TSP Benchmark Information"
            echo "========================================"
            echo "[INFO] Benchmark functionality requires test case files."
            echo "[INFO] Use 'Create Sample Files' to generate test cases first."
            echo "[INFO] Then use 'Solve All Test Cases' to benchmark multiple files."
            echo
            echo "Available sample sizes:"
            echo "- Small (3 cities): Fast solving, good for testing"
            echo "- Medium (5 cities): Moderate complexity"
            echo "- Large (7 cities): Higher complexity, takes more time"
            echo
            echo "For custom benchmarking:"
            echo "1. Create multiple test files with different sizes"
            echo "2. Use 'Solve All Test Cases' to process them"
            echo "3. Compare the results and timing manually"
            echo
            read -p "Press Enter to continue..."
            ;;
        7)
            echo
            echo "TSP Solver Help"
            echo "========================================"
            echo
            echo "Available Commands:"
            echo
            echo "1. LIST TEST CASES:"
            echo "   - Shows all .txt files in data/ folder"
            echo "   - Displays count of available test cases"
            echo
            echo "2. SOLVE SPECIFIC:"
            echo "   - Solves one test case from data/ folder"
            echo "   - Option for verbose output"
            echo "   - Shows detailed output with visualization"
            echo
            echo "3. SOLVE ALL:"
            echo "   - Processes all test cases in data/ folder"
            echo "   - Runs each one sequentially with verbose output"
            echo "   - Good for batch processing"
            echo
            echo "4. CREATE SAMPLES:"
            echo "   - Creates example files in data/ folder"
            echo "   - Three types: small (3 cities), medium (5 cities), large (7 cities)"
            echo
            echo "5. VALIDATE:"
            echo "   - Checks if test case file is valid"
            echo "   - Reports errors and warnings"
            echo "   - Verifies graph structure and TSP requirements"
            echo
            echo "6. BENCHMARK:"
            echo "   - Displays benchmark information"
            echo "   - Note: Actual benchmarking requires sample files in data/ folder"
            echo "   - Use 'Create Sample Files' first to generate test cases"
            echo
            echo "File Format for data/ folder:"
            echo "  # Comments start with #"
            echo "  CITIES: City1, City2, City3, City4"
            echo "  MATRIX:"
            echo "  0 10 15 20"
            echo "  5  0  9 10"
            echo "  6 13  0 12"
            echo "  8  8  9  0"
            echo
            echo "Notes:"
            echo "  - Matrix must be square (n x n)"
            echo "  - Diagonal elements must be 0"
            echo "  - All non-diagonal elements must be > 0"
            echo "  - CITIES line is optional (will auto-generate names)"
            echo
            echo "Sample Types:"
            echo "  - small: 3 cities (fast solving)"
            echo "  - medium: 5 cities (moderate complexity)"
            echo "  - large: 7 cities (higher complexity, slower)"
            echo
            echo "Manual Usage:"
            echo "  cargo run --release solve -f data/test1.txt"
            echo "  cargo run --release solve -f data/test1.txt --verbose"
            echo "  cargo run --release validate -f data/problem.txt"
            echo "  cargo run --release sample -o data/my_test.txt -k medium"
            echo
            read -p "Press Enter to continue..."
            ;;
        8)
            echo
            echo "[INFO] Thank you for using TSP Solver!"
            echo
            echo "To create your own test cases:"
            echo "1. Create a .txt file in the data/ folder"
            echo "2. Follow the format shown in Help section"
            echo "3. Use 'Validate' to check your file"
            echo "4. Use 'Solve Specific' to run your test case"
            echo
            echo "Goodbye!"
            exit 0
            ;;
        *)
            echo "[ERROR] Invalid choice. Please try again."
            echo
            ;;
    esac
done