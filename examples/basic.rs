use morgan::Morgan;

fn main() {
    Morgan::init(Vec::new());
    log::error!("My error message");
    log::warn!("My warn message");
    log::info!("My info message");
    log::debug!("Will not be shown");
    log::trace!("Will not be shown");
}