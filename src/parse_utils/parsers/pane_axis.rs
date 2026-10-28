use iced::widget::pane_grid::Axis;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsePaneAxisError {
    #[error("Invalid axis: {0}")]
    InvalidAxis(String),
}

/// Parse the pane axis value from a string into an Iced PaneGrid Axis enum.
/// The pane axis value can be specified in the following format:
/// "horizontal" or "vertical"
pub fn parse_pane_axis(value: &str) -> Result<Axis, ParsePaneAxisError> {
    match value {
        "horizontal" => Ok(Axis::Horizontal),
        "vertical" => Ok(Axis::Vertical),
        _ => {
            return Err(ParsePaneAxisError::InvalidAxis(value.to_string()));
        }
    }
}
