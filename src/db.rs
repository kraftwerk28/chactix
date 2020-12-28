use postgres::{Client, Config, NoTls};
use std::time::Duration;

macro_rules! rtenv {
    ($name:literal) => {
        ::std::env::var($name)
            .expect(format!("Env variable {} is not set", $name).as_str())
            .as_str()
    };
    ($name:literal as $t:ty) => {
        ::std::env::var($name)
            .expect(format!("Env variable {} is not set", $name).as_str())
            .parse::<$t>()
            .unwrap()
    };
    ($name:literal or $fb:literal) => {
        ::std::env::var($name).unwrap_or($fb.to_string()).as_str()
    };
    ($name:literal or $fb:literal as $t:ty) => {
        ::std::env::var($name)
            .unwrap_or($fb.to_string())
            .parse::<$t>()
            .unwrap()
    };
}

pub async fn connect() -> Client {
    Config::new()
        .user(rtenv!("DBUSER"))
        .password(rtenv!("DBPASS"))
        .dbname(rtenv!("DBNAME"))
        .host(rtenv!("DBHOST" or "localhost"))
        .port(rtenv!("DBPORT" or "5432" as u16))
        .connect_timeout(Duration::from_secs(2))
        .connect(NoTls)
        .unwrap()
}
