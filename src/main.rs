use lucian::gate::hub::line_header::LineType;
use lucian::log::Log;
use lucian::server::Server;

const APP_PORT:usize = 3389;
const LOG_PORT:usize = 6000;

fn main() {
    Log::init();
    let mut app = Server::new(APP_PORT,LineType::Operator);
    let mut http = Server::new(LOG_PORT,LineType::Http);
    std::thread::spawn(move ||{
        http.start();
    });
    app.start();
}

