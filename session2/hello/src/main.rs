fn hello_thread(n: u32) {
    println!("Hello from hello_thread {n}")
}

fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    }
    n
}

fn main() {
    println!("Hello from main_thread");

    let mut thread_handles = Vec::new();

    for i in 0..10 {
        let hello_thread_handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(hello_thread_handle)
    }

    // for handle in thread_handles {
    //     handle.join().unwrap();
    // }

    thread_handles.into_iter().for_each(|h| {
        println!("{}", h.join().unwrap());
    });
}
