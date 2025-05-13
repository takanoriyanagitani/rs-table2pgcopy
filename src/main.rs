use std::process::ExitCode;

use std::io;

use postgres::{Config, NoTls};

use rs_table2pgcopy::tablename2pgcopy2stdout;

fn env_val_by_key(key: &'static str) -> impl FnMut() -> Result<String, io::Error> {
    move || std::env::var(key).map_err(io::Error::other)
}

fn env2table_name() -> Result<String, io::Error> {
    env_val_by_key("ENV_TABLE_NAME")()
}

fn sub() -> Result<(), io::Error> {
    let user: String = env_val_by_key("PGUSER")()?;
    let host: String = env_val_by_key("PGHOST")()?;
    let pass: String = env_val_by_key("PGPASSWORD")()?;
    let dbnm: String = env_val_by_key("PGDATABASE")()?;

    let mut cfg = Config::new();
    cfg.user(&user);
    cfg.host(&host);
    cfg.password(&pass);
    cfg.dbname(&dbnm);

    let mut client = cfg.connect(NoTls).map_err(io::Error::other)?;
    let trusted_table_name: String = env2table_name()?;
    tablename2pgcopy2stdout(&trusted_table_name, &mut client)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
