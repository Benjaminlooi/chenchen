// Layout management library
// Handles split-screen layout calculation based on provider count

pub mod calculator;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::types::ProviderId;

/// Layout type based on number of selected providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum LayoutType {
    /// Single provider - full window
    Full,
    /// Two providers - side-by-side vertical split
    VerticalSplit,
    /// Three providers - grid layout (2 top, 1 bottom)
    Grid,
}

/// Panel dimensions for a provider in the split-screen layout
/// All values are percentages from 0.0 to 1.0
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PanelDimension {
    pub provider_id: ProviderId,
    pub x: f32,      // X position (0.0 - 1.0 as percentage of window width)
    pub y: f32,      // Y position (0.0 - 1.0 as percentage of window height)
    pub width: f32,  // Width (0.0 - 1.0 as percentage)
    pub height: f32, // Height (0.0 - 1.0 as percentage)
}

impl PanelDimension {
    pub fn new(provider_id: ProviderId, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            provider_id,
            x,
            y,
            width,
            height,
        }
    }
}

/// Complete layout configuration for all selected providers
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutConfiguration {
    pub provider_count: u8,
    pub layout_type: LayoutType,
    pub panel_dimensions: Vec<PanelDimension>,
}

impl LayoutConfiguration {
    pub fn new(
        provider_count: u8,
        layout_type: LayoutType,
        panel_dimensions: Vec<PanelDimension>,
    ) -> Self {
        Self {
            provider_count,
            layout_type,
            panel_dimensions,
        }
    }
}
