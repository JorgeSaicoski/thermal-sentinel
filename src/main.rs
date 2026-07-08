mod app;
mod domain;
mod infra;
mod interface;

fn main() {
    let reading = app::snapshot::take();
    interface::display::display_reading(reading);
    let readings = app::snapshot::take_all();
    interface::display::display_readings(readings);
}
