use std::fs;
use std::process::Command;

fn main() {
    // Remove the node_modules directory before running npm install
    let node_modules_path = "wallet-adapter/node_modules";
    if fs::metadata(node_modules_path).is_ok() {
        fs::remove_dir_all(node_modules_path).expect("Failed to remove node_modules directory");
    }

    // Run npm install
    run_command("npm", &["install"], "npm install");

    // Run npm run build
    run_command("npm", &["run", "build"], "npm run build");

    println!("cargo:warning=npm run build completed successfully in wallet-adapter folder");
}

fn run_command(cmd: &str, args: &[&str], description: &str) {
    let output = Command::new(cmd)
        .current_dir("wallet-adapter")
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute {}", description));

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Failed to run {}: {}", description, stderr);
    }
}
