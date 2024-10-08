
<body>
    <div>
        <h1>Prime Number Finder with NVIDIA GPU</h1>
  <h3>Overview</h3>
  <strong>This project is a high-performance prime number finder that harnesses the computational power of NVIDIA GPUs through the OpenCL framework. By leveraging the parallel processing capabilities of GPUs, the application accelerates the search for prime numbers up to a specified limit, achieving faster results compared to traditional CPU-based methods.</strong>
  
  <h3>Requirements</h3>
        <ul>
            <li><strong>Rust:</strong> The programming language used for this project. You can install it from the official <a href="https://www.rust-lang.org/learn/get-started">Rust website</a>.</li>
            <li><strong>NVIDIA GPU:</strong> Ensure you have an NVIDIA GPU with CUDA support.</li>
            <li><strong>NVIDIA CUDA Toolkit:</strong> Required for OpenCL development. Download and install it from the <a href="https://developer.nvidia.com/cuda-toolkit">NVIDIA website</a>.</li>
            <li><strong>OpenCL:</strong> The framework used for parallel computing. It should be included in the CUDA Toolkit.</li>
        </ul>
        
  <h3>Quick Start</h3>
  <pre><code> git clone https://github.com/idevanshu/primeNumber-rust.git</code></pre>
  <pre><code>cargo run build </code></pre>
