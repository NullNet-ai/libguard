# nullnet-platform
A collection of shared libraries used across various components of the NullNet architecture

## Steps to add a new member to the platform
1. Modify the `Cargo.toml` file `[memebers]` section to include the new member (in the form `"client_libraries/member_name"` or `"server_libraries/member_name"`)
2. Run the command `cargo new <new_member_path> --lib` to generate the skeleton for the new member
