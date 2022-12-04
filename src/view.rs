pub fn toggle_fullscreen(frame: &mut eframe::Frame) {
    if frame.info().window_info.fullscreen {
        frame.set_fullscreen(false);
    } else {
        frame.set_fullscreen(true);
    }
}
