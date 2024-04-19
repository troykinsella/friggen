use env_logger::{Builder, Env};

pub fn init() {
    let env = Env::default().filter("FRIGGEN_LOG");
    Builder::from_env(env).init();
}
