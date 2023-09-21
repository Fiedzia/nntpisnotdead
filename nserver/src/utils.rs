use std::path::Path;

use env_file_reader;

///This MUST be run BEFORE anything else, ideally as first thing program does
///as setting env vars after they've been read makes no sense
///also any access to env vars in multithreaded context is risky
pub fn read_env_file_if_present() -> std::io::Result<()> {
    let fname = ".env";
    if Path::new(fname).exists() {
        let env_map = env_file_reader::read_file(".env")?;
        for (k, v) in &env_map {
            std::env::set_var(k, v);
        } 
    }
    Ok(())
}
