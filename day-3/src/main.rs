// Tokio Runtime: Rust's Answer to Go's Goroutines
// This example demonstrates how Tokio provides similar concurrency patterns to Go's goroutines
// with async/await syntax and a powerful runtime for managing asynchronous tasks.

use tokio::time::{sleep, Duration};
use tokio::sync::{mpsc, oneshot};
use std::sync::Arc;
use tokio::sync::Mutex;

// MAIN FUNCTION: Entry point with Tokio runtime
// The #[tokio::main] macro transforms the async main function into a synchronous
// entry point that creates a Tokio runtime and executes the async code.
// This is similar to Go's runtime that manages goroutines automatically.
#[tokio::main]
async fn main() {
    println!("Tokio Runtime Demo: Goroutine-like Concurrency in Rust\n");
    
    // Example 1: Basic task spawning (like go func() in Go)
    basic_spawn_example().await;
    
    // Example 2: Channels for communication (like Go channels)
    channel_communication_example().await;
    
    // Example 3: Multiple producers, single consumer pattern
    mpsc_example().await;
    
    // Example 4: Select-like behavior with tokio::select!
    select_example().await;
    
    // Example 5: Shared state with Arc and Mutex
    shared_state_example().await;
    
    // Example 6: Task cancellation and graceful shutdown
    cancellation_example().await;
    
    println!("\nAll examples completed!");
}

// EXAMPLE 1: Basic Task Spawning
// tokio::spawn() is the equivalent of Go's "go" keyword
// It spawns a new asynchronous task that runs concurrently on the Tokio runtime
async fn basic_spawn_example() {
    println!("Example 1: Basic Task Spawning (like 'go func()')");
    
    // Spawn multiple tasks concurrently
    // Each tokio::spawn() creates a new task (similar to goroutine)
    let handle1 = tokio::spawn(async {
        for i in 1..=3 {
            println!("  Task 1: Count {}", i);
            // sleep() yields control back to the runtime, allowing other tasks to run
            sleep(Duration::from_millis(100)).await;
        }
        "Task 1 completed"
    });
    
    let handle2 = tokio::spawn(async {
        for i in 1..=3 {
            println!("  Task 2: Count {}", i);
            sleep(Duration::from_millis(150)).await;
        }
        "Task 2 completed"
    });
    
    // JoinHandle allows us to wait for task completion and get the result
    // Similar to sync.WaitGroup or waiting for goroutines to finish
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    
    println!("  {} | {}\n", result1, result2);
}

// EXAMPLE 2: Channel Communication
// Tokio channels work like Go channels for sending data between tasks
// oneshot channel: single value, one sender, one receiver (like a promise)
async fn channel_communication_example() {
    println!("Example 2: Channel Communication (like Go channels)");
    
    // oneshot channel: perfect for getting a single result from a task
    let (tx, rx) = oneshot::channel();
    
    // Spawn a task that performs some work and sends the result
    tokio::spawn(async move {
        println!("  Worker: Processing data...");
        sleep(Duration::from_millis(200)).await;
        
        // Send the result through the channel
        // tx is moved into this closure (ownership transfer)
        let result = "Processed data: 42";
        tx.send(result).unwrap();
    });
    
    // Wait to receive the result from the channel
    // This is non-blocking at the async level - the runtime can do other work
    match rx.await {
        Ok(value) => println!("  Main: Received: {}\n", value),
        Err(_) => println!("  Main: Sender dropped\n"),
    }
}

// EXAMPLE 3: Multiple Producer, Single Consumer (MPSC)
// mpsc (multi-producer, single-consumer) channels allow multiple tasks
// to send data to a single receiver - perfect for aggregating results
async fn mpsc_example() {
    println!("Example 3: MPSC - Multiple Producers, Single Consumer");
    
    // Create an mpsc channel with a buffer size of 10
    // Buffer allows senders to send without waiting for receiver (up to capacity)
    let (tx, mut rx) = mpsc::channel::<String>(10);
    
    // Spawn multiple producer tasks
    for i in 1..=3 {
        // Clone the sender for each task (mpsc allows multiple senders)
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            let message = format!("Message from producer {}", i);
            println!("  Producer {}: Sending message", i);
            
            // send() is async and returns Result
            tx_clone.send(message).await.unwrap();
        });
    }
    
    // Drop the original sender so the channel closes when all clones are dropped
    drop(tx);
    
    // Receive all messages
    // recv() returns None when all senders are dropped and channel is empty
    while let Some(message) = rx.recv().await {
        println!("  Consumer: Received: {}", message);
    }
    
    println!("  All producers finished\n");
}

// EXAMPLE 4: Select-like Behavior
// tokio::select! macro is similar to Go's select statement
// It waits on multiple async operations and proceeds with the first one that completes
async fn select_example() {
    println!("Example 4: Select-like Behavior (like Go's select)");
    
    let (tx1, mut rx1) = mpsc::channel::<String>(1);
    let (tx2, mut rx2) = mpsc::channel::<String>(1);
    
    // Spawn two tasks with different delays
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx1.send("Fast task completed".to_string()).await.unwrap();
    });
    
    tokio::spawn(async move {
        sleep(Duration::from_millis(300)).await;
        tx2.send("Slow task completed".to_string()).await.unwrap();
    });
    
    // select! will execute the first branch that becomes ready
    // This is non-deterministic if multiple branches are ready simultaneously
    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("  Received from fast channel: {}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("  Received from slow channel: {}", msg);
        }
    }
    
    // Wait a bit and receive from the other channel
    sleep(Duration::from_millis(250)).await;
    
    if let Some(msg) = rx2.recv().await {
        println!("  Received remaining message: {}\n", msg);
    }
}

// EXAMPLE 5: Shared State with Arc and Mutex
// Arc (Atomic Reference Counting) + Mutex allows safe shared state between tasks
// Similar to Go's sync.Mutex but with Rust's ownership guarantees
async fn shared_state_example() {
    println!("Example 5: Shared State (like sync.Mutex in Go)");
    
    // Arc allows multiple ownership across tasks (thread-safe reference counting)
    // Mutex provides mutual exclusion for safe concurrent access
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn 5 tasks that all increment the same counter
    for i in 1..=5 {
        // Clone Arc to share ownership with the spawned task
        let counter_clone = Arc::clone(&counter);
        
        let handle = tokio::spawn(async move {
            // Lock the mutex to get exclusive access
            // The lock is automatically released when the guard goes out of scope
            let mut num = counter_clone.lock().await;
            *num += 1;
            println!("  Task {}: Incremented counter to {}", i, *num);
            
            // Simulate some work while holding the lock
            sleep(Duration::from_millis(50)).await;
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Read the final value
    let final_value = counter.lock().await;
    println!("  Final counter value: {}\n", *final_value);
}

// EXAMPLE 6: Task Cancellation and Graceful Shutdown
// Demonstrates how to cancel tasks and handle graceful shutdown
// Similar to context cancellation in Go
async fn cancellation_example() {
    println!("Example 6: Task Cancellation (like context.Context in Go)");
    
    // Spawn a long-running task
    let handle = tokio::spawn(async {
        for i in 1..=10 {
            println!("  Long-running task: iteration {}", i);
            sleep(Duration::from_millis(100)).await;
        }
        "Completed all iterations"
    });
    
    // Let it run for a bit
    sleep(Duration::from_millis(350)).await;
    
    // Cancel the task by aborting it
    // This is similar to canceling a context in Go
    handle.abort();
    
    // Check if the task was cancelled
    match handle.await {
        Ok(result) => println!("  Task completed: {}", result),
        Err(e) if e.is_cancelled() => println!("  Task was cancelled (as expected)"),
        Err(e) => println!("  Task failed: {}", e),
    }
    
    println!();
}


// KEY CONCEPTS SUMMARY
// 
// 1. TOKIO RUNTIME:
//    - Manages async tasks (like Go's runtime manages goroutines)
//    - Work-stealing scheduler for efficient task execution
//    - Automatic thread pool management
//
// 2. TOKIO::SPAWN:
//    - Equivalent to Go's "go" keyword
//    - Spawns tasks that run concurrently
//    - Returns JoinHandle for waiting and getting results
//
// 3. ASYNC/AWAIT:
//    - Rust's syntax for asynchronous programming
//    - .await yields control back to runtime (like goroutine scheduling)
//    - Zero-cost abstraction - no runtime overhead
//
// 4. CHANNELS:
//    - oneshot: Single value, one-time communication
//    - mpsc: Multiple producers, single consumer
//    - broadcast: Multiple producers, multiple consumers (not shown)
//
// 5. TOKIO::SELECT!:
//    - Wait on multiple async operations
//    - Proceeds with first one that completes
//    - Similar to Go's select statement
//
// 6. SHARED STATE:
//    - Arc: Atomic reference counting for shared ownership
//    - Mutex: Mutual exclusion for safe concurrent access
//    - Rust's ownership prevents data races at compile time
//
// 7. ADVANTAGES OVER GO:
//    - Zero-cost abstractions (no runtime overhead)
//    - Memory safety without garbage collection
//    - Compile-time prevention of data races
//    - Fine-grained control over async behavior
//    - No hidden allocations or runtime surprises
//
