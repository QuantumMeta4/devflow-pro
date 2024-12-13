fn main() {
    println!("Hello, world!");

    // A simple function to test analysis
    let result = calculate_fibonacci(10);
    println!("Fibonacci(10) = {}", result);
}

fn calculate_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

// A complex function to test Windsurf analysis
fn calculate_fibonacci_with_memoization(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut memo = vec![0; (n + 1) as usize];
    memo[1] = 1;

    for i in 2..=n as usize {
        memo[i] = memo[i - 1] + memo[i - 2];
    }

    memo[n as usize]
}

// A function with potential security issues
fn process_user_input(input: &str) -> String {
    // WARNING: Unsafe - direct command execution
    let output = std::process::Command::new("echo")
        .arg(input)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

// A function with high cognitive complexity
fn complex_nested_logic(x: i32, y: i32) -> bool {
    if x > 0 {
        if y > 0 {
            if x > y {
                true
            } else if x == y {
                if x % 2 == 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            if x > -y {
                true
            } else {
                false
            }
        }
    } else {
        false
    }
}
