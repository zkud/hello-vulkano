use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::{app_info_from_cargo_toml, Version};

fn main() {
    let (mut device, mut queue) = init_device();

    let buffer = init_buffer(&device);
}

fn init_buffer(device: &Arc<Device>) -> Arc<CpuAccessibleBuffer<i32>> {
    let data: i32 = 123;
    CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
        .expect("failed to create buffer")
}

fn init_device() -> (Arc<Device>, Arc<Queue>) {
    let app_infos = app_info_from_cargo_toml!();
    let instance = Instance::new(
        Some(&app_infos),
        Version::V1_1,
        &InstanceExtensions::none(),
        None,
    )
    .unwrap();
    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");
    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(
            physical_device,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    (device, queue)
}
