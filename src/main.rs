use std::env;

fn main() {
    // Access command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 3 {
        println!("Usage: {} <package/executable> <install_parameters>", args[0]);
        return;
    }

    // Get the package and install parameters
    let package = &args[1];
    let install_parameters = &args[2];

    // Use pattern matching to determine the installer type
    match package {
        p if p.ends_with(".exe") => install_exe(p, install_parameters),
        p if p.ends_with(".msi") => install_msi(p, install_parameters),
        _ => println!("Package must be an executable or MSI file"),
    }
}

// Install MSI function
fn install_msi(package: &str, install_parameters: &str) {
    println!("Installing MSI {} with parameters {}", package, install_parameters);
}

// Install EXE function
fn install_exe(package: &str, install_parameters: &str) {
    println!("Installing EXE {} with parameters {}", package, install_parameters);
}
