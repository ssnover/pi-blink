use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let my_led = Pin::new(533);
    my_led
        .with_exported(|| {
            my_led.set_direction(Direction::Out).unwrap();
            loop {
                my_led.set_value(1).unwrap();
                std::thread::sleep(Duration::from_secs(1));
                my_led.set_value(0).unwrap();
                std::thread::sleep(Duration::from_secs(1));
            }
        })
        .unwrap();
}
