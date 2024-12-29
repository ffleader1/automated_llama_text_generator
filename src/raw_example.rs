use std::ops::Add;

const EXAMPLE1_PREVIOUS_ANSWER: & str = r#"
Here's the updated code with the `handle_telnet` function implemented:

```rust
fn handle_telnet(
    mut stream: TcpStream,
    peer_addr: &str,
    connections: &Arc<Mutex<HashMap<String, ClientConnection>>>
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }

                let input_str = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}: {}", peer_addr, input_str);

                let response = format!("Echo: {}", input_str);
                stream.write_all(response.as_bytes())?;

                // Handle matrix input
                if input_str.starts_with("matrix") {
                    let input_str = input_str.trim_start_matches("matrix");
                    let input_str = input_str.trim();

                    match serde_json::from_str::<MatrixInput>(&input_str) {
                        Ok(matrices) => {
                            let result = Self::process_matrices_old(matrices);
                            let response_body = serde_json::to_string(&result).unwrap();
                            stream.write_all(response_body.as_bytes())?;
                        }
                        Err(e) => {
                            let response = format!("Error: {}", e);
                            stream.write_all(response.as_bytes())?;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", peer_addr, e);
                return Err(e);
            }
        }
    }
}
```

To test this using `nc` (Netcat), you can use the following commands:

```bash
nc 127.0.0.1 49152
```

Then, you can send messages to the server by typing them in and pressing Enter. The server will echo back the message.

To send a matrix to the server, you can use the following format:

```
matrix {"matrix_a":[[1,2],[3,4]],"matrix_b":[[5,6],[7,8]]}
```

This will send a matrix multiplication request to the server, which will respond with the result.

Note that the matrix input must be in JSON format, and it must start with the string "matrix".
"#;

const EXAMPLE1_PROMPT: &str = r#"I got this err: Connection error: Resource temporarily unavailable (os error 35)"#;

const EXAMPLE1_SAMPLE_RESULT: &str = r#"
# Experience
- Need experience about debugging Telnet in Rust
- Rating: Medium - Hard

# Knowledge
- Rust Telnet knowledge is not common
- The information provided is only the log
- The model has to debug telnet in conjunction with curl http
- Rating: Hard

# Ambiguity
- Prompt is clear about the error and the expectation
- it does not provide any hint to the solution or at least what could be wrong
- Rating: Medium

# Complexity
- Unclear where this problem is, thus the solution is hard to determine
- Coule be related to the provided Telnet implementation, but maybe changing code in other part could also fix it
- Rating: Hard

# Overall
 Difficulty Hard
"#;

const EXAMPLE2_PREVIOUS_ANSWER: &str = "(None)";

const EXAMPLE2_PROMPT: &str = r#"
The following code measure cpu usage, ram, cache size and cache access time, cross platform
Place look at the code, without take into account any speculative observation or edge case
The code barely have any comment, so it's your duty to add comment to such code.
Also make clear by comment and text answer any OS-lacking feature

```
use sysinfo::System;
use std::env;
use std::time::Instant;

const L1_SIZE: usize = 32 * 1024;
const L2_SIZE: usize = 256 * 1024;

fn measure_basic(metric: &str) -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    match metric {
        "cpu" => format!("CPU Usage: {:.1}%", sys.global_cpu_usage()),
        "ram" => {
            let used = sys.used_memory() as f64 / 1024.0 / 1024.0;
            let total = sys.total_memory() as f64 / 1024.0 / 1024.0;
            format!("RAM: {:.2} GB used / {:.2} GB total", used, total)
        }
        _ => "Unknown metric".to_string()
    }
}

fn get_cache_size() -> (Option<u64>, Option<u64>, Option<u64>) {
    #[cfg(target_os = "windows")]
    {
        let mut l1 = None;
        let mut l2 = None;
        let mut l3 = None;

        if let Ok(output) = std::process::Command::new("wmic")
            .args(["cpu", "get", "L2CacheSize,L3CacheSize"])
            .output() {
            let output = String::from_utf8_lossy(&output.stdout);
            for line in output.lines() {
                if let Ok(size) = line.trim().split_whitespace().next()
                    .and_then(|s| s.parse::<u64>()) {
                    l2 = Some(size * 1024);
                }
                if let Ok(size) = line.trim().split_whitespace().nth(1)
                    .and_then(|s| s.parse::<u64>()) {
                    l3 = Some(size * 1024);
                }
            }
        }
        (l1, l2, l3)
    }


    #[cfg(target_os = "linux")]
    {
        let mut l1 = None;
        let mut l2 = None;
        let mut l3 = None;

        return match std::process::Command::new("lscpu").output() {
            Ok(output) => {
                let output = String::from_utf8_lossy(&output.stdout);
                for line in output.lines() {
                    if line.contains("L1d cache:") {
                        l1 = parse_size(line
                            .split(':')
                            .nth(1));
                    } else if line.contains("L2 cache:") {
                        l2 = parse_size(line
                            .split(':')
                            .nth(1));
                    } else if line.contains("L3 cache:") {
                        l3 = parse_size(line
                            .split(':')
                            .nth(1));
                    }
                }
                (l1, l2, l3)
            }
            Err(err) => {
                (0,0,0)
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut l1 = None;
        let mut l2 = None;
        let mut l3 = None;

        for (cache, size) in [
            ("l1dcachesize", &mut l1),
            ("l2cachesize", &mut l2),
            ("l3cachesize", &mut l3)
        ].iter_mut() {
            if let Ok(output) = std::process::Command::new("sysctl")
                .arg(format!("hw.{}", cache))
                .output() {
                let output = String::from_utf8_lossy(&output.stdout);
                **size = output.split(':').nth(1)
                    .and_then(|s| s.trim().parse().ok());
            }
        }
        (l1, l2, l3)
    }
}

fn measure_cache(size: usize) -> u128 {
    let mut data = vec![0u8; size];
    let iterations = 100_000;

    for i in (0..size).step_by(64) {
        data[i] = 1;
    }

    let start = Instant::now();
    let mut sum = 0u8;

    for _ in 0..iterations {
        for i in (0..size).step_by(64) {
            sum = sum.wrapping_add(data[i]);
            data[i] = sum;
        }
    }

    start.elapsed().as_nanos() / iterations as u128
}

fn parse_size(input_opt: Option<&str>) -> Option<u64> {
    if input_opt == None {
        return Some(0)
    }
    let input = input_opt.unwrap();
    let input = input.trim().to_uppercase();
    let mut chars = input.chars();

    let mut num_str = String::new();
    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            num_str.push(c);
        } else {
            break;
        }
    }

    // Parse the number
    let number: f64 = num_str.parse().ok()?;

    // Get the unit part
    let unit: String = chars.collect();
    let unit = unit.trim();

    let bytes = match unit {
        "KIB" | "KB" | "K" => number * 1024.0,
        "MIB" | "MB" | "M" => number * 1024.0 * 1024.0,
        "GIB" | "GB" | "G" => number * 1024.0 * 1024.0 * 1024.0,
        "" => number,

        _ => return None
    };

    Some(bytes as u64)
}

fn format_size(size: u64) -> String {
    if size >= 1_048_576 {
        format!("{:.1} MB", size as f64 / 1_048_576.0)
    } else if size >= 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{} B", size)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <metric>", args[0]);
        println!("Available metrics: cpu, ram, l1, l2, cache_size");
        return;
    }

    match args[1].to_lowercase().as_str() {
        "cpu" | "ram" => println!("{}", measure_basic(&args[1])),
        "l1" => {
            let time = measure_cache(L1_SIZE);
            println!("L1 Cache access time: {} nanoseconds", time);
        }
        "l2" => {
            let time = measure_cache(L2_SIZE);
            println!("L2 Cache access time: {} nanoseconds", time);
        }
        "cache_size" => {
            let (l1, l2, l3) = get_cache_size();
            println!("Cache Sizes:");
            if let Some(size) = l1 {
                println!("L1: {}", format_size(size));
            }
            if let Some(size) = l2 {
                println!("L2: {}", format_size(size));
            }
            if let Some(size) = l3 {
                println!("L3: {}", format_size(size));
            }
        }
        _ => println!("Unknown metric. Use: cpu, ram, l1, l2, or cache_size")
    }
}
```
"#;

const EXAMPLE2_SAMPLE_RESULT: &str = r#"
# Experience
- Need basic Rust experience to understand Rust code
- Rating: Easy - Medium

# Knowledge
- Rust basic knowledge to understand the code
- Rating: Easy - Medium

# Ambiguity
- Prompt is clear about the code, the task and the expectation
- Rating: Easy

# Complexity
- Just adding comments to the code, no need to code any line
- Also need to find an OS-lacking feature
- Rating: Easy - Medium

# Overall
 Difficulty Easy

"#;

pub fn generate_sample() -> String {
    let mut spm = format!("\n---\
    Example 1:\n\
    Previous Answer:{}\n\
    Current Prompt:{}\n\
    Expected Markdown {}\n---\n", EXAMPLE1_PREVIOUS_ANSWER, EXAMPLE1_PROMPT, EXAMPLE1_SAMPLE_RESULT);

    spm.add(
        &format!("\n---\
    Example 2:\n\
    Previous Answer:{}\n\
    Current Prompt:{}\n\
    Expected Markdown {}\n---\n", EXAMPLE2_PREVIOUS_ANSWER, EXAMPLE2_PROMPT, EXAMPLE2_SAMPLE_RESULT)
    )
}