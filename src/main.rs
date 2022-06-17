use lucian::app::App;
use lucian::hub::line::LineType;

const LISTEN_ADDR:&str = "0.0.0.0:3389";

fn main() {
    let mut app = App::new(LISTEN_ADDR,LineType::Operator);
    app.start();
}
