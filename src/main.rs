use std::env;
use std::process::{ Command, ExitStatus };

fn main() {
    // Access command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 3 {
        println!("Usage: {} <package/executable> <installer_args>", args[0]);
        return;
    }

    // Get the package and install parameters
    let installer_path = &args[1];
    let installer_args = &args[2];

    // Use pattern matching to determine the installer type
    let status = match installer_path {
        p if p.ends_with(".exe") => install_exe(p, installer_args),
        p if p.ends_with(".msi") => install_msi(p, installer_args),
        _ => {
            println!("Path must be an executable or MSI file");
            return;
        }
    };

    // Check the exit status of the installer
    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Installer completed successfully.");
            } else {
                println!("Installer failed with exit code: {:?}", exit_status.code());
                println!(
                    "Error codes can be looked up at https://learn.microsoft.com/en-us/windows/win32/msi/error-codes"
                );
            }
        }
        Err(e) => {
            println!("Error running installer: {:?}", e);
        }
    }
}

// Install MSI function
fn install_msi(installer_path: &str, installer_args: &str) -> Result<ExitStatus, std::io::Error> {
    println!("Installing MSI {} with parameters {}", installer_path, installer_args);
    Command::new("msiexec")
        .args(&["/i", installer_path, installer_args])
        .spawn()
        .and_then(|mut child| child.wait())
}

// Install EXE function
fn install_exe(installer_path: &str, installer_args: &str) -> Result<ExitStatus, std::io::Error> {
    println!("Installing EXE {} with parameters {}", installer_path, installer_args);
    Command::new(installer_path)
        .args(installer_args.split_whitespace()) // Split installer_args by spaces
        .spawn()
        .and_then(|mut child| child.wait())
}
