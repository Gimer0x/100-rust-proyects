use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::Arc;
use rayon::prelude::*;

// 1MB chunks - balances parallelism with overhead (file I/O, thread coordination)
// Smaller chunks = more parallelism but more overhead.
// Larger chunks = less overhead but less parallelism.
const CHUNK_SIZE: usize = 1024 * 1024;

fn main() {
    println!("Multi-threaded File Copier");
 
    let src = prompt("Enter source file path: ");
    let dest = prompt("Enter destination file path: ");
 
    match copy_file_in_parallel(&src, &dest) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => println!("Error copying file: {}", e),
    }
}

// std::io::Result<()> is just a type alias for the standard Result with an I/O error
// pub type Result<T> = std::result::Result<T, std::io::Error>;
fn copy_file_in_parallel(src_path: &str, dest_path: &str) -> std::io::Result<()>{
    let src_file = File::open(src_path)?;
    let src_metadata = src_file.metadata()?;
    let file_size = src_metadata.len() as usize;

    // Ceiling division: (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE
    // Ensures any remainder creates an extra chunk (e.g., 2.1MB file = 3 chunks)
    // Example: 2.1MB file with 1MB chunks = (2,097,152 + 1,048,576 - 1) / 1,048,576 = 3 chunks
    let chunk_count = (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE;
    println!("File size: {} bytes | Chunks: {}", file_size, chunk_count);

    // Arc (Atomically Reference Counted) allows safe sharing of immutable data across threads
    // Each thread needs the source path to open its own file handle
    // We can't share a &str across threads, so we own it with to_string() and wrap in Arc
    let src_arc = Arc::new(src_path.to_string());
    
    // Pre-allocate destination file to full size - critical for parallel writes
    // Without this, threads writing to different offsets could cause file corruption or
    // the OS might not allow seeking past the end of the file
    let dest_file = File::create(dest_path)?;
    dest_file.set_len(file_size as u64)?;

    // rayon's into_par_iter() converts the range into a parallel iterator
    // try_for_each() processes chunks concurrently and short-circuits on first error
    // The ? operator here propagates any I/O error from the parallel execution
    (0..chunk_count).into_par_iter().try_for_each(|i| {
        let offset = i * CHUNK_SIZE;
        // Last chunk may be smaller - use min() to avoid reading past file end
        // For a 2.1MB file with 1MB chunks: chunk 0 = 1MB, chunk 1 = 1MB, chunk 2 = 0.1MB
        // 0u8 = zero value of type u8 (unsigned 8-bit integer, i.e., a byte)
        // vec![value; size] creates a vector with 'size' elements, each initialized to 'value'
        // The initial 0 values will be overwritten by read_exact(), but we need to pre-allocate the buffer
        let mut buffer = vec![0u8; CHUNK_SIZE.min(file_size - offset)];
 
        // Each thread opens its own file handle - File is not Send/Sync
        // This avoids contention on a shared file handle and allows true parallelism
        // The &*src_arc dereferences the Arc to get &String, then coerces to &str
        let mut src = File::open(&*src_arc)?;
        src.seek(SeekFrom::Start(offset as u64))?;
        src.read_exact(&mut buffer)?;
 
        // OpenOptions allows reopening the file for writing in each thread
        // Multiple threads can safely write to different offsets of the same file
        // as long as they don't overlap (which our chunking ensures)
        let mut dest = OpenOptions::new()
            .write(true)
            .open(dest_path)?;
        dest.seek(SeekFrom::Start(offset as u64))?;
        dest.write_all(&buffer)?;
 
        // Explicit type annotation required for try_for_each's error handling
        // The compiler needs to know the error type to properly propagate it
        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
