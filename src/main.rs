extern crate gndump;
use gndump::dump;
use gndump::prep::{self, Config};

fn main() {
    simple_logger::init().unwrap();
    let conf = Config::new();
    prep::set_dirs(&conf.mysql_dir, &conf.pg_dir).unwrap();
    match dump::mysql_to_csv(conf) {
        Ok(()) => (),
        Err(err) => panic!("CSV error: {}", err),
    };
}
