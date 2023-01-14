use rustyserver::types::Server;
use std::thread;

#[cfg(test)]
mod test_server {
    use super::*;
    const ADDRESS: &str = "localhost:3000";

    #[test]
    fn it_runs_server() {
        let handle = thread::spawn(|| {
            Server::new(ADDRESS).listen_once();
        });

        reqwest::blocking::get(format!("http://{}/ok", ADDRESS)).unwrap();

        handle.join().unwrap();
    }
}
