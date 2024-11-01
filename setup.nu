#!/usr/bin/env nu

# Color constants using the `ansi` command
let green = (ansi green)
let yellow = (ansi yellow)
let nc = (ansi reset)

# Global constants
let repo_url = "https://github.com/Abdogouhmad/cshell"
let bin_path = "/usr/bin/cshell"

# Print messages with color
def printcl [color message] {
    print $"($color)($message)($nc)"
}

# Install the cshell CLI
def install_cshell [] {
    printcl $yellow "Installing The CLI into $bin_path..."

    # Download the binary
    if !(sudo wget -q --show-progress -O $bin_path "$repo_url/releases/latest/download/cshell") {
        printcl $yellow "Error: Failed to download the binary."
        return 1
    }

    # Make the binary executable
    if !(sudo chmod +x $bin_path) {
        printcl $yellow "Error: Failed to make cshell executable."
        return 1
    }

    printcl $green "Installed Successfully! Run the CLI with the command: cshell help"
}

# Uninstall the cshell CLI
def uninstall_cshell [] {
    printcl $yellow "Uninstalling The CLI from $bin_path..."

    # Remove the binary
    if !(sudo rm -i $bin_path) {
        printcl $yellow "Error: Failed to remove the binary at $bin_path."
        return 1
    }

    printcl $green "Uninstalled Successfully!"
}

# Main function to choose an action
def main [] {
    printcl $yellow "Choose an action:"
    printcl $yellow "1: Install"
    printcl $yellow "2: Uninstall"
    printcl $yellow "q: exit"

    let choice = (input "Enter your choice: ")

    match $choice {
        "1" => install_cshell
        "2" => uninstall_cshell
        "q" => exit
    }
}

# Run the main function
main
