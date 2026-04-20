# Update your package list and upgrade existing packages

sudo apt update && sudo apt upgrade -y

# Install essential build tools required for compiling Rust
# build-essential includes gcc, g++, make, and other necessary tools
# curl is needed for downloading rustup
# pkg-config is needed for some Rust crates
# libssl-dev is needed for OpenSSL support in Rust
sudo apt install -y build-essential curl pkg-config libssl-dev

# Download and install Rust using rustup
echo "Downloading and installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the rustup environment
source "$HOME/.cargo/env"

# Verify installation
echo "Verifying installation..."
rustc --version
cargo --version

echo "Rust installation complete!"
