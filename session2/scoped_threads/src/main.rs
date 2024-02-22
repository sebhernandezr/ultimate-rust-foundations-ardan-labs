use std::thread;

const N_THREADS: usize = 8;

fn main() {
    let to_add = (0..5000).collect::<Vec<u32>>();
    let chunks = to_add.chunks(N_THREADS);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(thread_handle)
        }

        thread_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });

    println!("Sum: {}", sum);
}
