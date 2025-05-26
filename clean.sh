#!/bin/bash

echo "========================================"
echo "      TSP Solver - Cleanup Script"
echo "========================================"
echo

echo "[INFO] This will clean up build artifacts and lock files"
echo "[INFO] Files to be removed:"
echo "  - Cargo.lock"
echo "  - target/ folder (build artifacts)"
echo "  - Any temporary files"
echo

read -p "Are you sure you want to continue? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy] ]]; then
    echo "[INFO] Cleanup cancelled."
    exit 0
fi

echo
echo "[START] Cleaning up project..."

# Remove Cargo.lock if exists
if [ -f "Cargo.lock" ]; then
    echo "[CLEAN] Removing Cargo.lock..."
    if rm "Cargo.lock"; then
        echo "[SUCCESS] Cargo.lock removed"
    else
        echo "[ERROR] Failed to remove Cargo.lock"
    fi
else
    echo "[INFO] Cargo.lock not found"
fi

# Remove target folder if exists
if [ -d "target" ]; then
    echo "[CLEAN] Removing target/ folder..."
    if rm -rf "target"; then
        echo "[SUCCESS] target/ folder removed"
    else
        echo "[ERROR] Failed to remove target/ folder"
    fi
else
    echo "[INFO] target/ folder not found"
fi

# Remove other temporary files
echo "[CLEAN] Removing temporary files..."

# Remove backup files
if ls *.bak 1> /dev/null 2>&1; then
    rm *.bak 2>/dev/null
    echo "[SUCCESS] Backup files removed"
fi

# Remove temporary files
if ls *.tmp 1> /dev/null 2>&1; then
    rm *.tmp 2>/dev/null
    echo "[SUCCESS] Temporary files removed"
fi

# Remove editor backup files
if ls *~ 1> /dev/null 2>&1; then
    rm *~ 2>/dev/null
    echo "[SUCCESS] Editor backup files removed"
fi

# Remove vim swap files
if ls .*.swp 1> /dev/null 2>&1; then
    rm .*.swp 2>/dev/null
    echo "[SUCCESS] Vim swap files removed"
fi

# Remove emacs backup files
if ls \#*\# 1> /dev/null 2>&1; then
    rm \#*\# 2>/dev/null
    echo "[SUCCESS] Emacs backup files removed"
fi

# Remove OS specific files
if [ -f ".DS_Store" ]; then
    rm ".DS_Store" 2>/dev/null
    echo "[SUCCESS] .DS_Store removed"
fi

# Remove log files if any
if ls *.log 1> /dev/null 2>&1; then
    rm *.log 2>/dev/null
    echo "[SUCCESS] Log files removed"
fi

echo
echo "[SUCCESS] Cleanup completed!"
echo
echo "[INFO] What was cleaned:"
echo "- Build artifacts (target/ folder)"
echo "- Lock file (Cargo.lock)"
echo "- Temporary and backup files"
echo "- Editor swap/backup files"
echo
echo "[INFO] Next steps:"
echo "1. Run 'cargo build --release' to rebuild"
echo "2. Or use ./run.sh to build and run automatically"
echo