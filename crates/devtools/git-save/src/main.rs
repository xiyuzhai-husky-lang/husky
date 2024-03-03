use husky_cli_utils::ask::ask_user_for_permission;
use husky_control_flow_utils::require;
use std::process::Command;

fn main() {
    macro_rules! exec_command_if_permited {
        ($command: expr) => {
            require!(ask_user_for_permission($command));
            exec_command($command)
        };
    }
    exec_command("git status");
    exec_command_if_permited!("git add -A");
    exec_command_if_permited!("git commit -m save");
    exec_command_if_permited!("git push");
}

fn exec_command(command: &str) {
    let splits: Vec<_> = command.split(' ').collect();
    let name = splits[0];
    let args = &splits[1..];
    let output = Command::new(name)
        .arg("-c")
        .arg("color.status=always")
        .args(args)
        .output()
        .expect("failed to execute process");
    print!("{}", std::str::from_utf8(&output.stdout).unwrap());
    eprint!("{}", std::str::from_utf8(&output.stderr).unwrap());
}
