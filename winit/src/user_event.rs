/// The events that the skyshark core event loop can handle.
///
/// This enum is used to send events to the skyshark core from other parts of the program.
#[derive(Debug)]
pub enum DemoWinitEvent {
    /// A request to shut down skyshark.
    Kill,
}
