use diy_graphics_engine::app;

fn main() {
    match app::run() {
        Ok(_) => {
            println!("Exited successfully!");
        },
        Err(_error) => {
            println!("Something went wrong:\n\n\t{}\n", _error);
        }
    }
}
