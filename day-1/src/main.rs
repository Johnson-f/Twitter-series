// Import the Rng trait from the rand crate
// This trait provides methods for generating random numbers
use rand::Rng;

/// Generates a 24-bit UUID as a hexadecimal string
/// 
/// Purpose: Create a unique identifier using 24 bits (3 bytes) of random data
/// Returns: A 6-character hexadecimal string (e.g., "a3f2b1")
fn generate_24bit_uuid() -> String {
    // Create a thread-local random number generator
    // Purpose: Get access to a fast, thread-safe random number generator
    // Why: Each thread gets its own RNG instance for better performance
    let mut rng = rand::thread_rng();
    
    // Generate an array of 3 random bytes (24 bits total)
    // Purpose: Store the raw random data that will form our UUID
    // Why: 3 bytes = 24 bits, which gives us 16,777,216 possible unique IDs
    let bytes: [u8; 3] = [
        rng.r#gen::<u8>(),  // First byte (bits 0-7)
        rng.r#gen::<u8>(),  // Second byte (bits 8-15)
        rng.r#gen::<u8>(),  // Third byte (bits 16-23)
    ];
    
    // Convert the 3 bytes to a hexadecimal string
    // Purpose: Make the UUID human-readable and easy to copy/share
    // Why: {:02x} formats each byte as 2-digit lowercase hex with leading zeros
    // Example: bytes [163, 242, 177] becomes "a3f2b1"
    format!("{:02x}{:02x}{:02x}", bytes[0], bytes[1], bytes[2])
}

fn main() {
    // Print a header message to inform the user what's happening
    // Purpose: Provide context and instructions for stopping the program
    println!("Generating 24-bit UUIDs infinitely (Ctrl+C to stop):\n");
    
    // Start an infinite loop
    // Purpose: Generate UUIDs continuously until the user stops the program
    // Why: The requirement is to generate UUIDs "infinitely"
    loop {
        // Generate a new 24-bit UUID
        // Purpose: Create a fresh random identifier on each iteration
        let uuid = generate_24bit_uuid();
        
        // Print the UUID to the console
        // Purpose: Display the generated UUID so the user can see/use it
        println!("{}", uuid);
        
        // Pause execution for 100 milliseconds
        // Purpose: Slow down the output to make it readable for humans
        // Why: Without this, UUIDs would print too fast to read (thousands per second)
        // Note: Remove this line if you need maximum generation speed
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
