extern crate gndump;
use gndump::models::DataSource;

fn main() {
    let conn = gndump::establish_connection();
    let data = DataSource::all(&conn);
    for datum in data {
        let title = match datum.title {
            Some(t) => t,
            None => String::new(),
        };
        println!("{}", title);
    }
}
