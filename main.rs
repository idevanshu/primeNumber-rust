extern crate ocl;

use ocl::{ProQue, Buffer};

const KERNEL_SRC: &str = r#"
    __kernel void find_primes(__global ulong* primes, ulong max_num) {
        int gid = get_global_id(0);
        ulong num = gid * 2 + 3; // Start from 3, then 5, 7, 9, ...

        if (num <= max_num) {
            int is_prime = 1;

            for (ulong i = 2; i * i <= num; ++i) {
                if (num % i == 0) {
                    is_prime = 0;
                    break;
                }
            }

            primes[gid] = is_prime ? num : 0;
        }
    }
"#;

fn main() {
    println!("Enter a number: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let max_num: u64 = input.trim().parse().expect("Please enter a valid number");

    // Create a ProQue with the source code of our kernel.
    let pro_que = ProQue::builder()
        .src(KERNEL_SRC)
        .dims((max_num / 2 - 1) as usize)  // Use usize instead of u64
        .build().expect("Failed to build ProQue");

    // Create a buffer to store the primes.
    let buffer = Buffer::<u64>::builder()
        .queue(pro_que.queue().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len((max_num / 2 - 1) as usize)  // Use usize instead of u64
        .build().expect("Failed to build buffer");

    // Create a kernel.
    let kernel = pro_que.kernel_builder("find_primes")
        .arg(&buffer)
        .arg(max_num)
        .build().expect("Failed to build kernel");

    // Run the kernel.
    unsafe { kernel.enq().expect("Failed to enqueue kernel"); }

    // Read the results back to the host.
    let mut primes = vec![0u64; buffer.len()];
    buffer.read(&mut primes).enq().expect("Failed to read buffer");

    // Print the primes, 10 per row.
    let mut count = 0;
    for &prime in primes.iter() {
        if prime != 0 {
            print!("{:<12}", prime);
            count += 1;
            if count % 10 == 0 {
                println!();
            }
        }
    }

    if count % 10 != 0 {
        println!();
    }
}

