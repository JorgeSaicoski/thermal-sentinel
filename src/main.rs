use sysinfo::Components;


fn main() {
    println!("Hello, world!");
    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{}", component.label());
        let value: Option<f32> = component.temperature();
        println!("Temperature: {}", value.unwrap_or(0.0));
    }

}
