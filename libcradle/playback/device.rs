use gst::prelude::*;

fn print_device(device: &gst::Device) -> () {
    println!("Device added:");
    println!("\tDisplay name: {:?}", device.display_name());
    println!("\tClass: {:?}", device.device_class());
}

pub fn get_audio_sink() -> Option<gst::Element> {
    let monitor = gst::DeviceMonitor::new();
    monitor.add_filter(Some("Audio/Sink"), None);
    monitor.set_show_all(false);
    monitor.start().unwrap();
    let sink_devices = monitor.devices();
    sink_devices.iter().for_each(|device| {
        print_device(device);
    });
    let sink = sink_devices
        .iter()
        .map(|device| device.create_element(None).unwrap())
        .skip(1)
        .next();

    monitor.stop();
    sink
}
