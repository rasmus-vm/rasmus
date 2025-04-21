use std::env;

pub struct UserInput {
    pub source_file_path: String,
}

impl UserInput {
    pub fn parse_args() -> UserInput {
        let mut args = env::args();

        UserInput {
            source_file_path: args
                .nth(1)
                .unwrap_or("./rasmus/tests/files/factorial-main.wasm".into()),
        }
    }
}
