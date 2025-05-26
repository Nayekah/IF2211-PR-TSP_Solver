@echo off
setlocal

echo ========================================
echo      TSP Solver - Execution Script
echo ========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo not found! Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Check if data folder exists
if not exist "data" (
    echo [WARNING] data/ folder not found. Creating it...
    mkdir data
)

REM Build the project
echo Building TSP Solver...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    pause
    exit /b 1
)

echo [SUCCESS] Build successful!
echo.

:menu
echo ========================================
echo           TSP Solver Menu
echo ========================================
echo 1. List Test Cases in data/
echo 2. Solve Specific Test Case
echo 3. Solve All Test Cases
echo 4. Create Sample Files
echo 5. Validate Test Case
echo 6. Run Benchmark
echo 7. Help/Usage
echo 8. Exit
echo ========================================
set /p choice="Enter your choice (1-8): "

if "%choice%"=="1" goto list_cases
if "%choice%"=="2" goto solve_specific
if "%choice%"=="3" goto solve_all
if "%choice%"=="4" goto sample
if "%choice%"=="5" goto validate
if "%choice%"=="6" goto benchmark
if "%choice%"=="7" goto help
if "%choice%"=="8" goto exit
echo [ERROR] Invalid choice. Please try again.
echo.
goto menu

:list_cases
echo.
echo Test Cases in data/ folder:
echo ========================================
if exist "data\*.txt" (
    dir /b "data\*.txt" 2>nul
    echo.
    echo Total files:
    dir /b "data\*.txt" 2>nul | find /c /v ""
) else (
    echo [INFO] No .txt files found in data/ folder
    echo.
    echo Creating sample test cases...
    call :create_sample_in_data
)
echo.
pause
goto menu

:solve_specific
echo.
echo Solve Specific Test Case
echo ========================================
echo Available test cases:
if exist "data\*.txt" (
    dir /b "data\*.txt" 2>nul
) else (
    echo [INFO] No test cases found in data/ folder
    echo Creating sample files first...
    call :create_sample_in_data
    echo.
    echo Available test cases:
    dir /b "data\*.txt" 2>nul
)
echo.
set /p filename="Enter test case filename (without data/ prefix): "
if not exist "data\%filename%" (
    echo [ERROR] File 'data\%filename%' not found!
    echo.
    pause
    goto menu
)
echo.
echo Do you want verbose output? (y/N):
set /p verbose_choice=""
set verbose_flag=
if /i "%verbose_choice%"=="y" set verbose_flag=--verbose

echo.
echo [START] Solving TSP from data\%filename%...
cargo run --release solve -f "data\%filename%" %verbose_flag%
echo.
pause
goto menu

:solve_all
echo.
echo Solve All Test Cases
echo ========================================
echo [INFO] Processing all .txt files in data/ folder...
echo.
if not exist "data\*.txt" (
    echo [INFO] No test cases found! Creating sample files first...
    call :create_sample_in_data
    echo.
)

for %%f in (data\*.txt) do (
    echo ========================================
    echo [START] Solving: %%f
    echo ========================================
    cargo run --release solve -f "%%f" --verbose
    echo.
    echo [SUCCESS] Completed: %%f
    echo.
    timeout /t 2 /nobreak >nul
)
echo [SUCCESS] All test cases completed!
echo.
pause
goto menu

:sample
echo.
echo Creating Sample Files in data/
echo ========================================
call :create_sample_in_data
echo.
pause
goto menu

:create_sample_in_data
echo Creating sample files in data/ directory...
cargo run --release sample -o "data\sample_small.txt" -k small
cargo run --release sample -o "data\sample_medium.txt" -k medium  
cargo run --release sample -o "data\sample_large.txt" -k large
echo.
echo [SUCCESS] Sample files created in data/:
echo - sample_small.txt (3 cities - small example)
echo - sample_medium.txt (5 cities - medium example)
echo - sample_large.txt (7 cities - large example)
goto :eof

:validate
echo.
echo Validate Test Case
echo ========================================
echo Available test cases:
if exist "data\*.txt" (
    dir /b "data\*.txt" 2>nul
) else (
    echo [INFO] No test cases found in data/ folder
    echo Creating sample files first...
    call :create_sample_in_data
    echo.
    echo Available test cases:
    dir /b "data\*.txt" 2>nul
)
echo.
set /p filename="Enter filename to validate (without data/ prefix): "
if not exist "data\%filename%" (
    echo [ERROR] File 'data\%filename%' not found!
    echo.
    pause
    goto menu
)
echo.
echo [START] Validating data\%filename%...
cargo run --release validate -f "data\%filename%"
echo.
pause
goto menu

:benchmark
echo.
echo TSP Benchmark Information
echo ========================================
echo [INFO] Benchmark functionality requires test case files.
echo [INFO] Use 'Create Sample Files' to generate test cases first.
echo [INFO] Then use 'Solve All Test Cases' to benchmark multiple files.
echo.
echo Available sample sizes:
echo - Small (3 cities): Fast solving, good for testing
echo - Medium (5 cities): Moderate complexity 
echo - Large (7 cities): Higher complexity, takes more time
echo.
echo For custom benchmarking:
echo 1. Create multiple test files with different sizes
echo 2. Use 'Solve All Test Cases' to process them
echo 3. Compare the results and timing manually
echo.
pause
goto menu

:help
echo.
echo TSP Solver Help
echo ========================================
echo.
echo Available Commands:
echo.
echo 1. LIST TEST CASES:
echo    - Shows all .txt files in data/ folder
echo    - Displays count of available test cases
echo.
echo 2. SOLVE SPECIFIC:
echo    - Solves one test case from data/ folder
echo    - Option for verbose output
echo    - Shows detailed output with visualization
echo.
echo 3. SOLVE ALL:
echo    - Processes all test cases in data/ folder
echo    - Runs each one sequentially with verbose output
echo    - Good for batch processing
echo.
echo 4. CREATE SAMPLES:
echo    - Creates example files in data/ folder
echo    - Three types: small (3 cities), medium (5 cities), large (7 cities)
echo.
echo 5. VALIDATE:
echo    - Checks if test case file is valid
echo    - Reports errors and warnings
echo    - Verifies graph structure and TSP requirements
echo.
echo 6. BENCHMARK:
echo    - Displays benchmark information
echo    - Note: Actual benchmarking requires sample files in data/ folder
echo    - Use 'Create Sample Files' first to generate test cases
echo.
echo File Format for data/ folder:
echo   # Comments start with #
echo   CITIES: City1, City2, City3, City4
echo   MATRIX:
echo   0 10 15 20
echo   5  0  9 10
echo   6 13  0 12
echo   8  8  9  0
echo.
echo Notes:
echo   - Matrix must be square (n x n)
echo   - Diagonal elements must be 0
echo   - All non-diagonal elements must be ^> 0
echo   - CITIES line is optional (will auto-generate names)
echo.
echo Sample Types:
echo   - small: 3 cities (fast solving)
echo   - medium: 5 cities (moderate complexity)
echo   - large: 7 cities (higher complexity, slower)
echo.
echo Manual Usage:
echo   cargo run --release solve -f data\test1.txt
echo   cargo run --release solve -f data\test1.txt --verbose
echo   cargo run --release validate -f data\problem.txt
echo   cargo run --release sample -o data\my_test.txt -k medium
echo.
pause
goto menu

:exit
echo.
echo [INFO] Thank you for using TSP Solver!
echo.
echo To create your own test cases:
echo 1. Create a .txt file in the data/ folder
echo 2. Follow the format shown in Help section
echo 3. Use 'Validate' to check your file
echo 4. Use 'Solve Specific' to run your test case
echo.
echo Goodbye!
pause
exit /b 0