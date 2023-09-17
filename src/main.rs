mod server;
mod window;

fn main() {
    let url = "http://127.0.0.1:8080";
    let _ = window::run_window(url);
}
