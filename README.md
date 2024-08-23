
<body>
    <div>
        <h1>Prime Number Finder with NVIDIA GPU</h1>
  <h3>Overview</h3>
  <strong>This project is a high-performance prime number finder that leverages the power of NVIDIA GPUs using the OpenCL framework. The application finds prime numbers up to a specified limit by utilizing parallel computing capabilities of NVIDIA GPUs.</strong>
  
  <h3>Requirements</h3>
        <ul>
            <li><strong>Rust:</strong> The programming language used for this project. You can install it from the official <a href="https://www.rust-lang.org/learn/get-started">Rust website</a>.</li>
            <li><strong>NVIDIA GPU:</strong> Ensure you have an NVIDIA GPU with CUDA support.</li>
            <li><strong>NVIDIA CUDA Toolkit:</strong> Required for OpenCL development. Download and install it from the <a href="https://developer.nvidia.com/cuda-toolkit">NVIDIA website</a>.</li>
            <li><strong>OpenCL:</strong> The framework used for parallel computing. It should be included in the CUDA Toolkit.</li>
        </ul>
        
  <h3>Quick Start</h3>
  <pre><code>cargo new primegpu</code></pre>
  <strong> Cargo.toml </strong>
  <pre><code> [package]
name = "primegpu"
version = "0.1.0"
edition = "2021"
[dependencies]
ocl = "0.19.7"
</code>
</pre>

<pre><code>cargo run build </code></pre>
