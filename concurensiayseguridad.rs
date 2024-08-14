use std::thread;  

fn main() {  
    let handle = thread::spawn(|| {  
        for i in 1..5 {  
            println!("Desde el hilo: {}", i);  
        }  
    });  

    handle.join().unwrap(); // Esperar a que el hilo termine  
}
