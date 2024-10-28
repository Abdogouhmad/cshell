/// The `cmd!` macro is a convenient wrapper around `std::process::Command`
/// that executes a system command with specified arguments. It provides
/// enhanced error handling and logging to give clear output about the command's
/// success or failure. The macro also allows setting custom success and failure messages.
///
/// # Parameters
///
/// - `$command:expr` - The command to execute (e.g., `"chsh"`).
/// - `$($args:expr),*` - A variadic list of arguments to pass to the command.
/// - `path_shell: $path_shell:expr` - Specifies a mandatory argument (like a file path or shell path)
///   that will be appended as the last argument of the command.
/// - `success: $success_msg:expr` - The success message to log if the command executes successfully.
/// - `fail: $fail_msg:expr` - The failure message to log if the command fails (e.g., due to permission issues).
///
/// # Usage
///
///     let shell_name = "zsh";  // Example shell name, could be "bash", "nu", etc.
///
///     let shell_path = match shell_name {
///         "bash" => "/bin/bash",
///         "zsh" => "/usr/bin/zsh",
///         "nu" => "/usr/bin/nu",
///         _ => {
///             error!("Error: Unsupported shell '{}'", shell_name);
///             exit(1);
///         }
///     };
///
///     info!("Switching to shell: {}", shell_name);
///
///     // Attempt to change the shell using `chsh`
///     cmd!(
///         "chsh", "-s"; path_shell: shell_path;
///         success: format!("Shell successfully changed to {}. Please log out and log back in to apply the changes.", shell_name),
///         fail: "Failed to change shell: Permission denied or invalid shell."
///     );
/// ```
///
/// # Explanation of Execution Flow
///
/// 1. **Command Construction**: The macro constructs a `Command` object with
///    the specified `$command` and `$args`, appending `path_shell` as the last argument.
/// 2. **Error Handling**:
///     - If the command executes successfully, it logs the `success` message.
///     - If the command fails (non-zero status), it logs the `fail` message and exits with status `1`.
///     - If an error occurs when attempting to execute the command (e.g., command not found),
///       it logs an error message with the error details and exits with status `1`.
///
/// This macro simplifies repetitive error handling when running shell commands and improves code readability.
#[macro_export]
macro_rules! cmd {
    ($command:expr, $($args:expr),*; path_shell: $path_shell:expr; success: $success_msg:expr, fail: $fail_msg:expr) => {{
        let status = Command::new($command)
            $(.arg($args))*
            .arg($path_shell)
            .status();

        match status {
            Ok(status) if status.success() => {
                info!("{}", $success_msg);
            }
            Ok(_) => {
                error!("{}", $fail_msg);
                exit(1);
            }
            Err(e) => {
                error!("Error executing command `{}`: {}", $command, e);
                exit(1);
            }
        }
    }};
}
