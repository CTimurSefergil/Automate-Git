use names::Generator;
use std::process::{exit, Command};

fn update_commit_push() {
    // git add .
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add command");
    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1);
    }

    // git commit -a -m name_generator
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-a")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes.");
    }

    // git push
    let push_command = Command::new("git")
        .arg("push")
        .output()
        .expect("Failed to execute git push command");
    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to remote.");
        exit(1);
    }

    println!("Successfully added, committed, and pushed changes!");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
}
