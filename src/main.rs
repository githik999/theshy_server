use lucian::log::Log;
use lucian::server::Server;
use lucian::hub::line::LineType;

const APP_PORT:usize = 3389;
const LOG_PORT:usize = 6000;

fn main() {
    Log::init();
    let mut app = Server::new(APP_PORT,LineType::Operator);
    let mut log = Server::new(LOG_PORT,LineType::Log);
    std::thread::spawn(move ||{
        log.start();
    });
    app.start();
}

