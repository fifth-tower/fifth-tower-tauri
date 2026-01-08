pub mod script;

mod key_value;
pub use key_value::*;

pub mod record;

#[cfg(test)]
mod tests {

    use tracing::Level;

    use super::*;

    pub fn init() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}
