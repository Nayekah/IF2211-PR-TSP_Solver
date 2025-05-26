@echo off
setlocal

echo ========================================
echo      TSP Solver - Cleanup Script
echo ========================================
echo.

echo [INFO] This will clean up build artifacts and lock files
echo [INFO] Files to be removed:
echo   - Cargo.lock
echo   - target/ folder (build artifacts)
echo   - Any temporary files
echo.

set /p confirm="Are you sure you want to continue? (y/N): "
if /i not "%confirm%"=="y" (
    echo [INFO] Cleanup cancelled.
    pause
    exit /b 0
)

echo.
echo [START] Cleaning up project...

REM Remove Cargo.lock if exists
if exist "Cargo.lock" (
    echo [CLEAN] Removing Cargo.lock...
    del "Cargo.lock"
    if %errorlevel% equ 0 (
        echo [SUCCESS] Cargo.lock removed
    ) else (
        echo [ERROR] Failed to remove Cargo.lock
    )
) else (
    echo [INFO] Cargo.lock not found
)

REM Remove target folder if exists
if exist "target" (
    echo [CLEAN] Removing target/ folder...
    rmdir /s /q "target"
    if %errorlevel% equ 0 (
        echo [SUCCESS] target/ folder removed
    ) else (
        echo [ERROR] Failed to remove target/ folder
    )
) else (
    echo [INFO] target/ folder not found
)

REM Remove other temporary files
echo [CLEAN] Removing temporary files...

REM Remove backup files
if exist "*.bak" (
    del "*.bak" 2>nul
    echo [SUCCESS] Backup files removed
)

REM Remove temporary editor files
if exist "*.tmp" (
    del "*.tmp" 2>nul
    echo [SUCCESS] Temporary files removed
)

if exist "*~" (
    del "*~" 2>nul
    echo [SUCCESS] Editor backup files removed
)

REM Remove Windows specific temp files
if exist "Thumbs.db" (
    del "Thumbs.db" 2>nul
    echo [SUCCESS] Thumbs.db removed
)

if exist "desktop.ini" (
    del "desktop.ini" 2>nul
    echo [SUCCESS] desktop.ini removed
)

echo.
echo [SUCCESS] Cleanup completed!
echo.
echo [INFO] What was cleaned:
echo - Build artifacts (target/ folder)
echo - Lock file (Cargo.lock)  
echo - Temporary and backup files
echo.
echo [INFO] Next steps:
echo 1. Run 'cargo build --release' to rebuild
echo 2. Or use run.bat to build and run automatically
echo.
pause