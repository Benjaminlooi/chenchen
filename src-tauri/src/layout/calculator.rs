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
/// - 3 providers: Grid (2 top at h:0.5, 1 bottom full width)
///
/// # Panics
/// Panics if provider_count is not between 1 and 3
pub fn calculate_layout(providers: &[ProviderId]) -> LayoutConfiguration {
    let provider_count = providers.len() as u8;

    // Validation
    assert!(
        provider_count >= 1 && provider_count <= 3,
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
            // Grid layout: 2 top, 1 bottom full width
            let panels = vec![
                PanelDimension::new(providers[0], 0.0, 0.0, 0.5, 0.5), // Top-left
                PanelDimension::new(providers[1], 0.5, 0.0, 0.5, 0.5), // Top-right
                PanelDimension::new(providers[2], 0.0, 0.5, 1.0, 0.5), // Bottom (full width)
            ];
            (LayoutType::Grid, panels)
        }
        _ => unreachable!("Validation should prevent this"),
    };

    LayoutConfiguration::new(provider_count, layout_type, panel_dimensions)
}
