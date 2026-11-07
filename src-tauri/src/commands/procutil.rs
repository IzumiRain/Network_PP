// Process-spawn helper: on Windows a GUI process spawning a console
// subsystem binary (ping, netstat, powershell…) opens a visible console
// window per call — CREATE_NO_WINDOW suppresses that. Cross-platform safe.

use std::process::{Command, Output};

#[allow(unused_variables)]
pub fn command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd
}

#[allow(dead_code)]
pub fn output(program: &str, args: &[&str]) -> Result<Output, String> {
    command(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))
}
