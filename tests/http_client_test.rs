#[cfg(test)]
mod tests {
    use payup::http_client::{get_shared_client, get_shared_blocking_client};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_shared_client_is_same_instance() {
        // Get two references to the shared client
        let client1 = get_shared_client();
        let client2 = get_shared_client();
        
        // They should be the same Arc instance
        assert!(Arc::ptr_eq(&client1, &client2), 
                "Shared clients should be the same instance");
    }

    #[test]
    fn test_shared_blocking_client_is_same_instance() {
        // Get two references to the shared blocking client
        let client1 = get_shared_blocking_client();
        let client2 = get_shared_blocking_client();
        
        // They should be the same Arc instance
        assert!(Arc::ptr_eq(&client1, &client2), 
                "Shared blocking clients should be the same instance");
    }

    #[test]
    fn test_thread_safety() {
        // Test that multiple threads can access the shared client
        let mut handles = vec![];
        
        for i in 0..10 {
            let handle = thread::spawn(move || {
                let client = get_shared_client();
                println!("Thread {} got client", i);
                // Client should be accessible from multiple threads
                assert!(!client.is_null());
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_blocking_client_thread_safety() {
        // Test that multiple threads can access the shared blocking client
        let mut handles = vec![];
        
        for i in 0..10 {
            let handle = thread::spawn(move || {
                let client = get_shared_blocking_client();
                println!("Thread {} got blocking client", i);
                // Client should be accessible from multiple threads
                assert!(!client.is_null());
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_performance_comparison() {
        use std::time::Instant;
        
        // Measure time to get shared client (should be very fast)
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = get_shared_client();
        }
        let shared_duration = start.elapsed();
        
        // Measure time to create new clients
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = reqwest::Client::new();
        }
        let new_duration = start.elapsed();
        
        println!("Shared client access: {:?}", shared_duration);
        println!("New client creation: {:?}", new_duration);
        
        // Shared client should be significantly faster
        assert!(shared_duration < new_duration,
                "Shared client should be faster than creating new clients");
    }
}