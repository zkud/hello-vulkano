use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::{Version, app_info_from_cargo_toml};

fn main() {
    let app_infos = app_info_from_cargo_toml!();
    let instance = Instance::new(Some(&app_infos), Version::V1_1, &InstanceExtensions::none(), None).unwrap();
}
