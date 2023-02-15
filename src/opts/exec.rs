use containers_api::{
    impl_field, impl_opts_builder, impl_str_enum_field, impl_str_field, impl_vec_field,
};
use std::fmt;

impl_opts_builder!(json =>
    /// Modify how an exec session is run inside a container.
    ExecCreate
);

#[derive(Debug, Clone)]
/// One of the variants accepted by [`ExecCreateOptsBuilder::user`](ExecCreateOptsBuilder::user).
pub enum UserOpt {
    User(String),
    UserGroup(String, String),
    Uid(isize),
    UidGid(isize, isize),
}

impl fmt::Display for UserOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UserOpt::*;
        match self {
            User(user) => write!(f, "{user}"),
            Uid(uid) => write!(f, "{uid}"),
            UserGroup(user, group) => write!(f, "{user}:{group}"),
            UidGid(uid, gid) => write!(f, "{uid}:{gid}"),
        }
    }
}

impl ExecCreateOptsBuilder {
    impl_field!(
        /// Attach to stderr of the exec command
        attach_stderr: bool => "AttachStderr"
    );

    impl_field!(
        /// Attach to stdin of the exec command
        attach_stdin: bool => "AttachStdin"
    );

    impl_field!(
        /// Attach to stdout of the exec command
        attach_stdout: bool => "AttachStdout"
    );

    impl_vec_field!(
        /// Command to run, as a string or array of strings.
        command => "Cmd"
    );

    impl_str_field!(
        /// Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl- where is one of: a-z, @, ^, [, , or _.
        detach_keys => "DetachKeys"
    );

    /// A list of environment variables to use for the command execution.
    pub fn env<K, V>(mut self, vars: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.params.insert(
            "Env",
            vars.into_iter()
                .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref()))
                .collect(),
        );
        self
    }

    impl_field!(
        /// Runs the exec process with extended privileges
        privileged: bool => "Privileged"
    );

    impl_field!(
        /// Allocate a pseudo-TTY
        tty: bool => "Tty"
    );

    impl_str_enum_field!(
        /// The user, and optionally, group to run the exec process inside the container.
        user: UserOpt => "User"
    );

    impl_str_field!(
        /// The working directory for the exec process inside the container.
        working_dir => "WorkingDir"
    );
}

impl_opts_builder!(json =>
    /// Adjust how an exec instance is started inside of a running container.
    ExecStart
);

impl ExecStartOptsBuilder {
    impl_field!(
        /// Detach from the command.
        detach: bool => "Detach"
    );

    impl_field!(
        /// Height of the TTY session in characters. Tty must be set to true to use it.
        height: usize => "h"
    );

    impl_field!(
        /// Allocate a pseudo-TTY.
        tty: bool => "Tty"
    );

    impl_field!(
        /// Width of the TTY session in characters. Tty must be set to true to use it.
        width: usize => "w"
    );
}
