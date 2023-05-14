# EveryGarf Comic Downloader

A Rust program to download every Garfield comic to date.

# Stats

- Download size: 4.8GB
- Download time: ~50m, from one test
- Images: 16,400

# Installation

Install from source with `cargo` 

```bash
git clone https://github.com/darccyy/everygarf
cd everygarf
./install
```

# Usage

```bash
# Help information
everygarf --help

# Download to an existing folder
everygarf ~/Pictures/garfield

# Change some options
everygarf ~/Pictures/garfield -cq --attempts 20 --timeout 30
```

