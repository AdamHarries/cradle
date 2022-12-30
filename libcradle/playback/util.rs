pub mod gstreamer {
    use glib::{source::Priority, FlagsClass, MainContext, ObjectExt, PRIORITY_HIGH};
    use gst::prelude::*;

    pub fn disable_video(playbin: &mut gst::Element) -> () {
        // disable the video on `playbin`
        let flags: glib::Value = playbin.property_value("flags");
        let flags_class = FlagsClass::new(flags.type_())
            .expect("Could not construct flags class from playbin flags");
        let flags = flags_class
            .builder_with_value(flags)
            .expect("Failed to create flagbuilder")
            .unset_by_nick("video")
            .build()
            .expect("Failed to build flags");
        playbin.set_property_from_value("flags", &flags);
    }
}
