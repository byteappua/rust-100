use std::mem;
use bytes::BytesMut;
use object_pool::Pool;
use std::alloc::{GlobalAlloc, Layout, System};

// --- 1. Memory Layout Optimization ---

// Use #[repr(C)] to prevent compiler from reordering fields.
#[repr(C)]
struct Unoptimized {
    a: u8,   // 1 byte
    // padding: 7 bytes (to align b to 8)
    b: u64,  // 8 bytes
    c: u16,  // 2 bytes
    // padding: 6 bytes (to make total size multiple of alignment 8)
    // Total: 24 bytes
}

#[repr(C)]
struct Optimized {
    b: u64,  // 8 bytes
    c: u16,  // 2 bytes
    a: u8,   // 1 byte
    // padding: 5 bytes
    // Total: 16 bytes
}

#[allow(dead_code)]
fn demonstrate_layout() {
    println!("--- Memory Layout Optimization (using #[repr(C)]) ---");
    println!("Size of Unoptimized: {} bytes", mem::size_of::<Unoptimized>());
    println!("Size of Optimized:   {} bytes", mem::size_of::<Optimized>());
    println!("Alignment of Unoptimized: {}", mem::align_of::<Unoptimized>());
    println!("Alignment of Optimized:   {}", mem::align_of::<Optimized>());
    println!();
}

// --- 2. Zero Copy with bytes ---

fn demonstrate_zero_copy() {
    println!("--- Zero Copy with bytes ---");
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"Hello, ");
    buf.extend_from_slice(b"World!");

    // Convert to Bytes (immutable, cheap clone/slice)
    // The type of b1 is `bytes::Bytes`
    let b1 = buf.freeze();
    let b2 = b1.clone(); // Shallow clone, points to same memory

    println!("b1: {:?}", b1);
    println!("b2 (clone): {:?}", b2);
    // Cast pointers to demonstrate they point to the same buffer
    println!("b1 ptr: {:p}, b2 ptr: {:p}", b1.as_ptr(), b2.as_ptr());
    println!();
}

// --- 3. Object Pool ---

struct ReusableBuffer {
    data: Vec<u8>,
}

impl ReusableBuffer {
    fn new() -> Self {
        Self { data: Vec::with_capacity(1024) }
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

fn demonstrate_object_pool() {
    println!("--- Object Pool ---");
    let pool = Pool::new(10, || ReusableBuffer::new());

    {
        let mut obj = pool.pull(|| ReusableBuffer::new());
        println!("Pulled object from pool. Capacity: {}", obj.data.capacity());
        obj.data.push(1);
        println!("Buffer len: {}", obj.data.len());
        // Before returning to pool (dropping), we should ideally reset it.
        // The `object-pool` crate provides a `Reusable` wrapper, but here we just drop it.
        // If we want to reset, we do it before drop or when pulling.
        obj.clear();
    }

    {
        let obj = pool.pull(|| ReusableBuffer::new());
        // Since we cleared it, len should be 0.
        println!("Pulled object again. Buffer len: {}", obj.data.len());
        println!("Object capacity: {}", obj.data.capacity());
    }
    println!();
}

// --- 4. Custom Allocator ---

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

// Register the global allocator.
#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn demonstrate_allocator() {
    println!("--- Custom Allocator ---");
    println!("Custom allocator is registered (MyAllocator wrapping System).");
    let _v = vec![1, 2, 3];
    println!("Vector allocated successfully.");
    println!();
}

fn main() {
    println!("=== Day 87: Memory Optimization Demo ===\n");
    demonstrate_layout();
    demonstrate_zero_copy();
    demonstrate_object_pool();
    demonstrate_allocator();
}
