#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable parse color from string: {0}")]
    ParseColor(String),
    #[error("Unable parse color component from string")]
    ParseColorComponent(#[from] std::num::ParseIntError),
    #[error("Unable parse orientation from string: {0}")]
    ParseOrientation(String),
    #[error("Unable parse render order from string: {0}")]
    ParseRenderOrder(String),
    #[error("Unable parse stagger axis from string: {0}")]
    ParseStaggerAxis(String),
    #[error("Unable parse stagger index from string: {0}")]
    ParseStaggerIndex(String),
    #[error("Unable parse map type from string: {0}")]
    ParseMapType(String),
    #[error("Unable parse compression from string: {0}")]
    ParseCompression(String),
    #[error("Unable parse draw order from string: {0}")]
    ParseDrawOrder(String),
    #[error("Unable parse encoding from string: {0}")]
    ParseEncoding(String),
    #[error("Unable parse layer type from string: {0}")]
    ParseLayerType(String),
    #[error("Unable parse data from string: {0}")]
    ParseDataSource(String),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
