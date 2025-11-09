#Summary#
A cli application that watches for modifications in a given file and executes arbitrary commands as when a change is detected.
A change in this context means that the modification timestamp in the file metadata changes. Currently no file content is taken into account.

Usage: fp [filepath] <command> [arg1] [arg2] ...
-------

#Installation#

1. Make sure you have cargo installed. If not, follow instructions here: https://doc.rust-lang.org/cargo/getting-started/installation.html
2. Clone repo, cd into the root of the repo
3. To build the binary, run cargo build --release
4. In case you want the binary to be directly adde to path so that you can call the tool with the fp command, run cargo install --path . instead of the build command.
