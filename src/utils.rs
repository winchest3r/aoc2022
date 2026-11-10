/// Split pattern for distinct data with empty space between.
pub const SPLIT_PATTERN: &'static str = cfg_select! {
    windows => "\r\n\r\n",
    _ => "\n\n",
};
