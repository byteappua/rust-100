#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::arch::asm;

// --- 1. SIMD (Single Instruction, Multiple Data) ---

// Scalar addition (standard loop)
fn add_scalar(a: &[f32], b: &[f32], result: &mut [f32]) {
    for i in 0..a.len() {
        result[i] = a[i] + b[i];
    }
}

// SIMD addition using AVX (Advanced Vector Extensions)
// Processes 8 floats (256 bits) at a time.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
unsafe fn add_avx(a: &[f32], b: &[f32], result: &mut [f32]) {
    let len = a.len();
    let mut i = 0;

    // Process 8 elements at a time
    while i + 8 <= len {
        // Load data into 256-bit registers (8 floats)
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));

        // Add vectors
        let vr = _mm256_add_ps(va, vb);

        // Store result
        _mm256_storeu_ps(result.as_mut_ptr().add(i), vr);

        i += 8;
    }

    // Handle remaining elements (scalar fallback)
    while i < len {
        result[i] = a[i] + b[i];
        i += 1;
    }
}

fn demonstrate_simd() {
    println!("--- SIMD Demo (AVX) ---");

    #[cfg(target_arch = "x86_64")]
    {
        // Check if AVX is supported on this machine
        if !is_x86_feature_detected!("avx") {
            println!("AVX not supported on this machine. Skipping SIMD demo.");
            return;
        }

        let len = 16;
        let a: Vec<f32> = (0..len).map(|x| x as f32).collect();
        let b: Vec<f32> = (0..len).map(|x| x as f32 * 2.0).collect();
        let mut res_scalar = vec![0.0; len];
        let mut res_simd = vec![0.0; len];

        // Scalar
        add_scalar(&a, &b, &mut res_scalar);
        println!("Scalar Result: {:?}", res_scalar);

        // SIMD
        unsafe {
            add_avx(&a, &b, &mut res_simd);
        }
        println!("SIMD Result:   {:?}", res_simd);

        assert_eq!(res_scalar, res_simd);
        println!("Results match!");
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        println!("SIMD demo skipped (not x86_64).");
    }

    println!();
}

// --- 2. Inline Assembly ---

fn demonstrate_asm() {
    println!("--- Inline Assembly Demo ---");

    #[cfg(target_arch = "x86_64")]
    unsafe {
        let x: u64 = 42;
        let y: u64;

        // Move value of x into a register, then move it to y.
        // Simple 'mov' instruction.
        asm!(
            "mov {0}, {1}",
            out(reg) y,
            in(reg) x,
        );

        println!("Input: {}, Output (via asm): {}", x, y);
    }

    #[cfg(not(target_arch = "x86_64"))]
    println!("Inline assembly demo skipped (not x86_64).");

    println!();
}

fn main() {
    println!("=== Day 88: SIMD & Low Level Optimization ===\n");
    demonstrate_simd();
    demonstrate_asm();
}
