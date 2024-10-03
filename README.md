# Product Graph

## Overview

This project is a Rust-based application that uses various dependencies and tools to build, run, and deploy. This README will guide you through setting up the project using Nix and the Makefile.

## Prerequisites

- [Nix](https://nixos.org/download.html) installed on your system.
- [Docker](https://docs.docker.com/get-docker/) installed if you plan to build Docker images.

## Setup

### Using Nix

1. **Clone the repository:**

   ```sh
   git clone https://github.com/your-repo/product-graph.git
   cd product-graph
   ```

2. **Enter the development shell:**

   Nix provides a development shell with all the necessary dependencies. Run the following command to enter the shell:

   ```sh
   nix develop
   ```

   This command uses the [`flake.nix`] configuration to set up the environment.

3. **Build the project:**

   Inside the Nix development shell, you can build the project using Cargo:

   ```sh
   cargo build
   ```

### Using the Makefile

The Makefile provides various tasks to build, run, test, and deploy the project.

1. **Build the project:**

   ```sh
   make build
   ```

2. **Run the project:**

   ```sh
   make run
   ```

3. **Test the project:**

   ```sh
   make test
   ```

4. **Build the project for release:**

   ```sh
   make release
   ```

5. **Deploy the project:**

   ```sh
   make deploy
   ```

### Building Docker Image

To build a Docker image for the project, use the following Makefile target:

```sh
make build.fly.image
```

This command will build the project using the [`x86_64-unknown-linux-musl`] target and create a Docker image named [`product-graph`].

## Additional Information

- The project uses [`flake.nix`] for Nix configuration, which includes dependencies and build instructions.
- The [`Makefile`] provides a convenient way to run common tasks.
- The [`Cargo.toml`] file lists all the Rust dependencies required for the project.

For more details on the project structure and dependencies, refer to the respective configuration files:

- [`Cargo.toml`]
- [`flake.nix`]
- [`Makefile`]

## License

This project is licensed under the MIT License. See the LICENSE file for details.