pub mod device;
pub mod util;
use std::path::PathBuf;
use std::thread;

use glib;
use glib::{source::Priority, FlagsClass, MainContext, MainLoop, ObjectExt, PRIORITY_HIGH};
use gst::prelude::*;
use gst::*;

use std::sync::Arc;
use std::sync::Mutex;

enum Messages {
    NoMessage,
    Done,
}

struct Playback {
    message_bus: glib::Receiver<Messages>,
    playbin: gst::Element,
    mainloop: MainLoop,
    thread: thread::JoinHandle<()>,
}

impl Playback {
    fn start() -> Arc<Mutex<Playback>> {
        // Start initialising the various glib components
        let ctx = glib::MainContext::default();
        let _guard = ctx.acquire();
        let mainloop = glib::MainLoop::new(Some(&ctx), false);

        // Initialise gstreamer
        gst::init().expect("Unable to initialise GStreamer");
        let mut playbin: gst::Element = gst::ElementFactory::make("playbin")
            .build()
            .expect("Unable to create `playbin` element");

        util::gstreamer::disable_video(&mut playbin);

        // Disabling this until we have a nice interface for choosing audio devices.
        // let new_sink = device::get_audio_sink().expect("Could not create sink from device");
        // playbin.set_property("audio-sink", &new_sink);

        // create a communiation channel so that the playback unit can communicate with the middle end
        let (main_tx, main_rx): (glib::Sender<Messages>, glib::Receiver<Messages>) =
            MainContext::channel(PRIORITY_HIGH);

        // hook up responses to gstreamer events
        playbin
            .bus()
            .expect("Failed to get GStreamer message bus")
            .add_watch(glib::clone!(@strong main_tx => move |_bus, msg| {
                match msg.view() {
                    gst::MessageView::Eos(_) =>
                        main_tx.send(Messages::Done).expect("Unable to send message to main()"),
                    gst::MessageView::Error(e) =>
                        glib::g_debug!("song", "{}", e.error()),
                        _ => (),
                }
                glib::Continue(true)
            }))
            .expect("Failed to connect to GStreamer message bus");

        // Handle signals when a song is almost done
        // This needs to send things to the middle end to find new songs
        playbin.connect(
            "about-to-finish",
            false,
            glib::clone!(@strong main_tx => move |_args| {
                println!("About to finish!");
                None
            }),
        );

        // Spawn a thread to run the mainloop
        let thread = thread::spawn(glib::clone!(@strong mainloop => move || {
            mainloop.run();
        }));

        // Bundle up the various things that we've created and return it to the user
        Arc::new(Mutex::new(Playback {
            message_bus: main_rx,
            playbin: playbin,
            mainloop: mainloop,
            thread: thread,
        }))
    }

    pub fn play_song(mut self, track: PathBuf) {
        self.playbin
            .set_state(gst::State::Ready)
            .expect("Could not set ready state");
        let path = track.as_path();
        let path_string = glib::filename_to_uri(path, None)
            .expect("Error converting path to uri")
            .to_string();
        self.playbin.set_property("uri", path_string);
        self.playbin
            .set_state(gst::State::Playing)
            .expect("Failed to start playback");
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        // Kill the mainloop
        self.mainloop.quit();
        // Stop the playbin from running
        self.playbin
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
        // We don't need to kill the thread. It should end itself as the handle will be dropped, and the thread will finish running and end.
        // let newthread = self.thread.join().unwrap();
    }
}
