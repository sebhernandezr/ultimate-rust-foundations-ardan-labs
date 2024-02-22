const N_THREADS: usize = 8;

fn main() {
    let to_add = (0..5000).collect::<Vec<u32>>();
    let mut thread_handles = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    let mut sum: u32 = 0;
    for thread_handle in thread_handles {
        sum += thread_handle.join().unwrap()
    }
    println!("Sum: {}", sum);
}
