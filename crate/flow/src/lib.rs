mod dir;
mod executor;
mod flow;
mod recording;
mod resource;
mod store;
mod util;
mod worker;

pub use dir::*;
pub use executor::*;
pub use flow::*;
pub use recording::*;
pub use resource::*;
pub use store::*;
pub use util::*;
pub use worker::*;

#[cfg(test)]
mod tests {

    use tracing::Level;

    use super::*;

    pub fn init() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        let app_dir = dirs::data_dir()
            .map(|dir| dir.join("nicee"))
            .unwrap()
            .to_string_lossy()
            .to_string();
        init_root_dir(&app_dir);
    }
}
