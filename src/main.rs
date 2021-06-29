use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::{app_info_from_cargo_toml, Version};

fn main() {
    let app_infos = app_info_from_cargo_toml!();
    let instance = Instance::new(
        Some(&app_infos),
        Version::V1_1,
        &InstanceExtensions::none(),
        None,
    )
    .unwrap();
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();
}
