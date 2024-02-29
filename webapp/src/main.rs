use parser::{parse_line, Register};
use poem::{
    handler,
    listener::TcpListener,
    post,
    web::{Json},
    Route, Server,
};

#[handler]
fn parse(body: String) -> Json<Register> {
    let mut register = Register::new();
    for line in body.lines() {
        let data = parse_line(line);
        register.push_data(data);
    }
    Json(register)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/parse", post(parse));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
