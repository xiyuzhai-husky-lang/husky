# Use the official NVIDIA CUDA runtime image for Ubuntu 22.04
FROM nvidia/cuda:12.6.0-runtime-ubuntu22.04

# Prevent interactive prompts during installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# Install Python, git, git-lfs and required packages
RUN apt-get update && \
    apt-get install -y python3 python3-venv python3-pip git git-lfs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Setup git-lfs and clone model
RUN git lfs install && \
    mkdir -p /root/.llms/models && \
    git clone https://huggingface.co/Qwen/Qwen2-7B-Instruct /root/.llms/models/Qwen2-7B-Instruct

# Install Rust and required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and verify installation
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /root/.cargo/bin/cargo --version

# Set PATH for subsequent container runs
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app

# Copy application files
COPY internal .

# Create and activate virtual environment, install dependencies
RUN python3 -m venv .venv && \
    . .venv/bin/activate && \
    pip install --upgrade pip && \
    pip install torch torchvision torchaudio && \
    pip install "sglang[all]" --find-links https://flashinfer.ai/whl/cu124/torch2.4/flashinfer/

# For the build command later
RUN cd sglang-docker-ipc && /root/.cargo/bin/cargo build --release

# Run the script
CMD ["/bin/bash", "-c", "source .venv/bin/activate && ./sglang-docker-ipc/target/release/sglang-docker-ipc"]