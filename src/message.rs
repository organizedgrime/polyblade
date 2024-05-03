use std::time::Instant;

use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    CloseAlert,
    Conway(ConwayMessage),
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ConwayMessage {
    // 1
    Seed,
    Dual,
    // 2
    Join,
    Ambo,
    // 3
    Kis,
    Needle,
    Zip,
    Truncate,
    // 4
    Ortho,
    Expand,
    // 5
    Gyro,
    Snub,
    // 6
    Meta,
    Bevel,
}
