use clap::Parser;

use glib::{source::Priority, FlagsClass, MainContext, ObjectExt, PRIORITY_HIGH};
use gstreamer::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "PlaySong")]
#[command(author = "Adam B-H. <harries.adam@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Plays a single song to completion", long_about = None)]
struct Cli {
    track_path: PathBuf,
}

enum Messages {
    Done,
}

fn main() -> () {
    let cli = Cli::parse();
    println!("Parsed arguments: {:?}", cli);
    let ctx = glib::MainContext::default();
    let _guard = ctx.acquire();
    let mainloop = glib::MainLoop::new(Some(&ctx), false);

    let mut playbin: gstreamer::Element;
    // let mut main_tx: glib::Sender<()>;

    // soi backend::new
    gstreamer::init().expect("Unable to initialise GStreamer");
    let playbin: gstreamer::Element = gstreamer::ElementFactory::make("playbin")
        .build()
        .expect("Unable to create `playbin` element");

    // disable the video on `playbin`
    let flags: glib::Value = playbin.property_value("flags");
    let flags_class =
        FlagsClass::new(flags.type_()).expect("Could not construct flags class from playbin flags");
    let flags = flags_class
        .builder_with_value(flags)
        .expect("Failed to create flagbuilder")
        .unset_by_nick("video")
        .build()
        .expect("Failed to build flags");
    playbin.set_property_from_value("flags", &flags);

    // create a glib communication channel
    let (main_tx, main_rx): (glib::Sender<Messages>, glib::Receiver<Messages>) =
        MainContext::channel(PRIORITY_HIGH);

    // hook up responses to gstreamer events
    playbin
        .bus()
        .expect("Failed to get GStreamer message bus")
        .add_watch(glib::clone!(@strong main_tx => move |_bus, msg| {
            match msg.view() {
                gstreamer::MessageView::Eos(_) =>
                    main_tx.send(Messages::Done).expect("Unable to send message to main()"),
                gstreamer::MessageView::Error(e) =>
                    glib::g_debug!("song", "{}", e.error()),
                    _ => (),
            }
            glib::Continue(true)
        }))
        .expect("Failed to connect to GStreamer message bus");

    // handle signals when a song is almost done
    playbin.connect(
        "about-to-finish",
        false,
        glib::clone!(@strong main_tx => move |_args| {
            println!("About to finish!");
            None
        }),
    );
    //.expect("Failed to connect `about-to-finish` signal");

    // send data every 100ms
    // ignore this for now.

    // play a song!
    playbin
        .set_state(gstreamer::State::Ready)
        .expect("Could not set ready state");
    let path = cli.track_path.as_path();
    let path_string = glib::filename_to_uri(path, None)
        .expect("Error converting path to uri")
        .to_string();
    playbin.set_property("uri", path_string);
    playbin
        .set_state(gstreamer::State::Playing)
        .expect("Failed to start playback");
    // main_tx
    //     .send(Messages::Done)
    //     .expect("Unable to send message to main()");

    // handle messages from backend

    main_rx.attach(
        None,
        glib::clone!(@strong mainloop => move |msg| {
            match msg {
                Messages::Done => {
                    println!("Done playing.");
                    mainloop.quit();
                }
            };
            glib::Continue(true)
        }),
    );

    mainloop.run();
    playbin.set_state(gstreamer::State::Null).expect("Unable to set the pipeline to the `Null` state");

    println!("Played a song!");
}
