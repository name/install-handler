# Rust Windows Installer Handler

This Rust application is designed to simplify the installation of both MSI and executable (EXE) packages on Windows. It accepts the path to the installer package and any installer-specific arguments, allowing you to automate software installations programmatically.

## Usage

To use this installer handler, follow these steps:

1. Compile the Rust code:

    ```bash
    cargo build --release
   ```

2. Run the application with the following command:

    ```bash
    ./target/release/installer_handler <package/executable> <installer_args>
    ```

    Replace <package/executable> with the path to the installer file (either a .exe or .msi file) and <installer_args> with any additional arguments required for the installation.

    Example usage:

    ```bash
    ./target/release/installer_handler "C:\Path\to\installer.exe" "/silent /norestart"
    ```

## Features

- Supports both MSI (.msi) and executable (.exe) installer packages.
- Automatically determines the installer type based on the file extension.
- Provides clear error messages for invalid inputs and installation failures.
- Waits for the installer to complete and reports the exit status.

## Resources

[Learn more about MSI error codes](https://learn.microsoft.com/en-us/windows/win32/msi/error-codes)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
