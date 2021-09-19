pub mod git_lib {
    use std::str::Split;

    // Struct for the input to the git hook, which is
    // a list of 3-token strings.
    pub struct GitPostReceiveInput {
        // The old head of the branch
        old_ref: String,
        // The new head of the branch
        new_ref: String,
        // The branch being modified
        branch_name: String
    }

    impl GitPostReceiveInput {
        pub fn from_command_line_string(command_line_input: String) -> GitPostReceiveInput {
            return from_vec(
                command_line_input.split(" ").collect::<Vec<&str>>()
            );
        }

        pub fn from_vec(command_line_vec: Vec<&str>) -> GitPostReceiveInput {
            // Command line input should be of the format:
            // "<old_ref> <new_ref> <branch_name>"
            assert_eq!(command_line_vec.len(), 3);

            return GitPostReceiveInput {
                old_ref: command_line_split[0],
                new_ref: command_line_split[1],
                branch_name: command_line_split[2]
            }
        }
    }
}