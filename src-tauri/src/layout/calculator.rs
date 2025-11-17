// Layout calculation logic
// Calculates split-screen panel dimensions based on provider count

use super::{LayoutConfiguration, LayoutType, PanelDimension};
use crate::types::ProviderId;
use log::info;

/// Calculates the layout configuration for the given list of selected providers
///
/// Layout rules per data-model.md:
/// - 1 provider: Full screen (x:0, y:0, w:1.0, h:1.0)
/// - 2 providers: Vertical split (left: w:0.5, right: x:0.5, w:0.5)
/// - 3 providers: Horizontal triple split (3 equal columns, w:0.333 each)
///
/// # Panics
/// Panics if provider_count is not between 1 and 3
pub fn calculate_layout(providers: &[ProviderId]) -> LayoutConfiguration {
    let provider_count = providers.len() as u8;

    // Validation
    assert!(
        (1..=3).contains(&provider_count),
        "Provider count must be between 1 and 3"
    );

    info!("Calculating layout for {} providers", provider_count);

    let (layout_type, panel_dimensions) = match provider_count {
        1 => {
            // Full screen layout
            let panels = vec![PanelDimension::new(providers[0], 0.0, 0.0, 1.0, 1.0)];
            (LayoutType::Full, panels)
        }
        2 => {
            // Vertical split: left and right panels
            let panels = vec![
                PanelDimension::new(providers[0], 0.0, 0.0, 0.5, 1.0), // Left panel
                PanelDimension::new(providers[1], 0.5, 0.0, 0.5, 1.0), // Right panel
            ];
            (LayoutType::VerticalSplit, panels)
        }
        3 => {
            // Horizontal triple split: 3 equal columns side by side
            let panel_width = 1.0 / 3.0; // Each panel gets 1/3 of the width
            let panels = vec![
                PanelDimension::new(providers[0], 0.0, 0.0, panel_width, 1.0), // Left column
                PanelDimension::new(providers[1], panel_width, 0.0, panel_width, 1.0), // Middle column
                PanelDimension::new(providers[2], 2.0 * panel_width, 0.0, panel_width, 1.0), // Right column
            ];
            (LayoutType::Grid, panels)
        }
        _ => unreachable!("Validation should prevent this"),
    };

    LayoutConfiguration::new(provider_count, layout_type, panel_dimensions)
}
