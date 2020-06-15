use snafu::{
    Backtrace,
    Snafu,
};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Unable parse color from string: {}", s))]
    ParseColor {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse color component from string: {}", source))]
    ParseColorComponent {
        backtrace: Backtrace,
        source:    std::num::ParseIntError,
    },
    #[snafu(display("Unable parse orientation from string: {}", s))]
    ParseOrientation {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse render order from string: {}", s))]
    ParseRenderOrder {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse stagger axis from string: {}", s))]
    ParseStaggerAxis {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse stagger index from string: {}", s))]
    ParseStaggerIndex {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse map type from string: {}", s))]
    ParseMapType {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse compression from string: {}", s))]
    ParseCompression {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse draw order from string: {}", s))]
    ParseDrawOrder {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse encoding from string: {}", s))]
    ParseEncoding {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse layer type from string: {}", s))]
    ParseLayerType {
        backtrace: Backtrace,
        s:         String,
    },
    #[snafu(display("Unable parse data from string: {}", s))]
    ParseDataSource {
        backtrace: Backtrace,
        s:         String,
    },
}
