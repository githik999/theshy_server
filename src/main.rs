use lucian::server::Server;
use omg_cool::{log::Log, config::Config,header::LineType};

fn main() {
    Log::create_log_dir();
    Config::set_panic_hook();
    let (app_addr,http_addr,write_log) = Config::get_listen_addr();
    if write_log {
        Config::turn_on();
    }
    let mut app = Server::new(app_addr,LineType::Operator);
    let mut http = Server::new(http_addr,LineType::Http);
    std::thread::spawn(move ||{
        http.start();
    });
    app.start();
}