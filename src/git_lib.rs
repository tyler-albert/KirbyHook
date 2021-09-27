pub mod git_lib {
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
        pub fn from_vec(command_line_vec: Vec<&str>) -> GitPostReceiveInput {
            assert_eq!(command_line_vec.len(), 3);

            return GitPostReceiveInput {
                old_ref: command_line_vec[0].to_string(),
                new_ref: command_line_vec[1].to_string(),
                branch_name: command_line_vec[2].to_string()
            }
        }

        pub fn from_command_line_string(command_line_input: String) -> GitPostReceiveInput {
            // Command line input should be of the format:
            // "<old_ref> <new_ref> <branch_name>"
            return GitPostReceiveInput::from_vec(
                command_line_input.split(" ").collect::<Vec<&str>>()
            );
        }
    }

    use std::fmt;
    impl fmt::Display for GitPostReceiveInput {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "branch: {}\n\told_ref:<{}>\n\tnew_ref:<{}>",
                    self.branch_name,
                    self.old_ref,
                    self.new_ref)
        }
    }

    // Struct for
}