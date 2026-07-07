mod app;
mod domain;
mod infra;
mod interface;

fn main() {
    let reading = app::snapshot::take();
    interface::display::show(&reading);
}
