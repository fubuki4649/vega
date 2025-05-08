pub struct ShellReturn {
    pub err_code: u8,
    pub stdout: String,
    pub stderr: String
}

/// A macro that executes a shell command using `/bin/sh` and captures its output.
///
/// # Usage
///
/// ```rust
/// use mycrate::sh;
///
/// let result = sh!("echo Hello, World!");
/// assert_eq!(result.stdout, "Hello, World!\n");
/// ```
///
/// # Details
/// - This macro takes a string-like input (formatted using `format!` if necessary) and passes it
///   as a command to `/bin/sh -c`.
/// - The command is executed synchronously, and its output, error, and exit code are captured in a `ShellReturn` struct.
///
/// # Returns
/// This macro evaluates to a `ShellReturn` struct with the following fields:
/// - `err_code`: The exit code of the command (u8). Defaults to `1` if the exit code cannot be determined.
/// - `stdout`: The standard output of the executed command, as a `String`.
/// - `stderr`: The standard error output of the executed command, as a `String`.
///
/// # Panics
/// This macro will panic if:
/// - The command fails to execute (e.g., due to invalid syntax or the `sh` executable not being found).
///
/// # Example
/// ```rust
/// let result = sh!("ls -l");
/// println!("Exit code: {}", result.err_code);
/// println!("Standard Output: {}", result.stdout);
/// println!("Standard Error: {}", result.stderr);
/// ```
///
/// # Notes
/// - This macro uses `std::process::Command` internally to execute the shell command.
/// - The exit code is cast to a `u8`; if the exit code cannot be determined, it defaults to `1`.
///
/// # Requirements
/// - The system must have a functional `/bin/sh` shell available to execute commands.
///
/// ```rust,no_run
/// let shell_return = sh!("invalid_command_123");
/// println!("Error Code: {}", shell_return.err_code);
/// println!("Standard Error: {}", shell_return.stderr);
/// ```
#[macro_export]
macro_rules! sh {
    ($($arg:tt)*) => {{
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!($($arg)*))
            .output()
            .expect("failed to execute command");

        ShellReturn {
            err_code: output.status.code().unwrap_or(1) as u8,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }};
}