// use clap::Parser;

use glib::{ffi::g_idle_add, source::Priority, FlagsClass, MainContext, ObjectExt, PRIORITY_HIGH};
use gstreamer::prelude::*;
use std::path::PathBuf;

enum Messages {
    Done,
}

fn print_device(device: &gstreamer::Device) -> () { 
    println!("Device added:");;
    println!("\tDisplay name: {:?}", device.display_name());
    println!("\tClass: {:?}", device.device_class());
}

fn main() {
    let ctx = glib::MainContext::default();
    let _guard = ctx.acquire();
    let mainloop = glib::MainLoop::new(Some(&ctx), false);
    gstreamer::init().expect("Unable to initialise gstreamer!");

    let devicemonitor = gstreamer::DeviceMonitor::new();
    devicemonitor.set_show_all_devices(false);
    devicemonitor.set_show_all(false);

    let (main_tx, main_rx): (glib::Sender<Messages>, glib::Receiver<Messages>) =
        MainContext::channel(PRIORITY_HIGH);

    devicemonitor
        .bus()
        .add_watch(glib::clone!(@strong main_tx => move |_bus, msg| {
            match msg.view() {
                gstreamer::MessageView::DeviceAdded(device_added) => {
                    print_device(&device_added.device());
                },
                _ => {},
            }
            glib::Continue(true)
        }))
        .expect("Failed to connect to devicemonitor message bus");

    // Build a callback for when the message bus is idle.
    // This needs to clean up the device monitor, and stop the main thread.
    glib::source::idle_add(
        glib::clone!(@strong mainloop, @strong devicemonitor => move || {
            devicemonitor.stop();
            mainloop.quit();
            glib::Continue(false)
        }),
    );

    devicemonitor
        .start()
        .expect("Failed to start devicemonitor!");

    

    mainloop.run();

    // playbin
    //     .bus()
    //     .expect("Failed to get GStreamer message bus")
    //     .add_watch(glib::clone!(@strong main_tx => move |_bus, msg| {
    //         match msg.view() {
    //             gstreamer::MessageView::Eos(_) =>
    //                 main_tx.send(Messages::Done).expect("Unable to send message to main()"),
    //             gstreamer::MessageView::Error(e) =>
    //                 glib::g_debug!("song", "{}", e.error()),
    //                 _ => (),
    //         }
    //         glib::Continue(true)
    //     }))
    //     .expect("Failed to connect to GStreamer message bus");
}
