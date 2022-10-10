mod ejf;
use ejf::{EjfConfig, build_ejf};

fn main() {
    build_ejf(EjfConfig {
        path: "./fonts/Roboto/Roboto-Light.ttf".to_string(),
        size: 25,
        skip_control_characters: true
    });
}
