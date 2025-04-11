# CELA (Conda Environment Lister & Analyzer)

![CELA Demo](assets/cela-demo.gif)

## Features

- **Fast Environment Discovery** - Quickly list all available Conda environments
- **Package Analysis** - View detailed package information for any environment
- **Clean Output Format** - Neatly formatted output for better readability
- **Error Resilience** - Robust error handling with helpful messages

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (1.85.1)
- Conda installed and available in your PATH

### From Source

```bash
# Clone the repository
git clone https://github.com/krascsi/cela.git
cd cela

# Build in release mode
cargo build --release

# Make the binary executable (Linux/macOS)
chmod +x target/release/cela

# Optional: Add to your PATH
cp target/release/cela ~/.local/bin/
```

## Usage

### List All Conda Environments

```bash
cela list-envs
```

Example output:
```
Available Conda environments:
-------------------------------
1. miniconda3 (/home/krascsi/miniconda3)
2. test (/home/krascsi/miniconda3/envs/test)

Total environments: 2
```

### List Packages in a Specific Environment

```bash
cela list-packages <environment-name>
```

Example output:
```
Listing packages for environment: test

Packages in 'test' environment:
-------------------------------
Name                 Version         Channel
----                 -------         -------
_libgcc_mutex        0.1             pkgs/main
_openmp_mutex        5.1             pkgs/main
blas                 1.0             pkgs/main
bzip2                1.0.8           pkgs/main
ca-certificates      2025.2.25       pkgs/main
expat                2.6.4           pkgs/main
intel-openmp         2023.1.0        pkgs/main
ld_impl_linux-64     2.40            pkgs/main
libffi               3.4.4           pkgs/main
libgcc-ng            11.2.0          pkgs/main
libgomp              11.2.0          pkgs/main
libmpdec             4.0.0           pkgs/main
libstdcxx-ng         11.2.0          pkgs/main
libuuid              1.41.5          pkgs/main
mkl                  2023.1.0        pkgs/main
mkl-service          2.4.0           pkgs/main
mkl_fft              1.3.11          pkgs/main
mkl_random           1.2.8           pkgs/main
ncurses              6.4             pkgs/main
numpy                2.2.4           pkgs/main
numpy-base           2.2.4           pkgs/main
openssl              3.0.16          pkgs/main
pip                  25.0            pkgs/main
python               3.13.2          pkgs/main
python_abi           3.13            pkgs/main
readline             8.2             pkgs/main
setuptools           72.1.0          pkgs/main
sqlite               3.45.3          pkgs/main
tbb                  2021.8.0        pkgs/main
tk                   8.6.14          pkgs/main
tzdata               2025a           pkgs/main
wheel                0.45.1          pkgs/main
xz                   5.6.4           pkgs/main
zlib                 1.2.13          pkgs/main

Total packages: 34
```

## Motivation

I developed CELA to address the need for a fast, efficient way to inspect Conda environments while learning Rust. Inspired by the work at [Prefix.dev](https://github.com/prefix-dev), which focuses on building tools for the Conda ecosystem, I wanted to create something practical that demonstrates my interest in developer tooling and the Conda ecosystem.

The project showcases:
- Rust's performance and safety guarantees
- Effective CLI design principles
- JSON parsing and structured data handling
- Proper error management

## Possible Future Enhancements

- Improving code quality
- Environment comparison functionality
- Package dependency visualization
- Export options to various formats (CSV, JSON)
- Integration with environment.yml files
- Search and filtering capabilities

## License

This project is licensed under the MIT License.

## Author

Created by [Bence Krascsenits] as a demonstration project for Prefix.dev.
