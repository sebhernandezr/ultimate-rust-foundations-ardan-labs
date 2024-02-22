use std::thread;

const N_THREADS: usize = 8;

fn main() {
    let to_add = (0..5000).collect::<Vec<u32>>();
    let chunks = to_add.chunks(N_THREADS);
    // opt.2 -> clone each chunk, so they have ownership.
    // let chunks = to_add.chunks(N_THREADS).map(Vec::from).collect::<Vec<_>>();

    let mut thread_handles = Vec::new();
    for chunk in chunks {
        // opt.1 -> get ownership of chunks.
        let chunk = chunk.to_owned();
        let thread_handle = thread::spawn(move || chunk.iter().sum::<u32>());
        thread_handles.push(thread_handle);
    }

    let sum = thread_handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum::<u32>();

    println!("Sum: {}", sum);
}
