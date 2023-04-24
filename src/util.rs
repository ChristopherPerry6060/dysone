/// Returns a handle to the secondary monitor.
///
/// This is determined by comparing the primary monitory's name with an
/// iterator of all available monitors. This function will return the first
/// monitor it finds that is not the primary.
pub fn secondary_monitor_handle(
    el: &glium::glutin::event_loop::EventLoop<()>,
) -> Option<glium::glutin::monitor::MonitorHandle> {
    let primary = el.primary_monitor();
    // This wont panic on single monitor machines as None will just default
    // to the system's preffernce.
    let primary_name = primary.as_ref()?.name();
    el.available_monitors().find(|x| x.name() != primary_name)
}
