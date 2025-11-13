// Unit tests for layout calculation logic
// Tests the calculate_layout() function with different provider counts

use chenchen_lib::layout::{LayoutType, LayoutConfiguration};
use chenchen_lib::layout::calculator::calculate_layout;
use chenchen_lib::types::ProviderId;

#[test]
fn test_calculate_layout_with_1_provider() {
    // Arrange
    let providers = vec![ProviderId::ChatGPT];

    // Act
    let layout = calculate_layout(&providers);

    // Assert
    assert_eq!(layout.provider_count, 1);
    assert_eq!(layout.layout_type, LayoutType::Full);
    assert_eq!(layout.panel_dimensions.len(), 1);

    let panel = &layout.panel_dimensions[0];
    assert_eq!(panel.provider_id, ProviderId::ChatGPT);
    assert_eq!(panel.x, 0.0);
    assert_eq!(panel.y, 0.0);
    assert_eq!(panel.width, 1.0);
    assert_eq!(panel.height, 1.0);
}

#[test]
fn test_calculate_layout_with_2_providers() {
    // Arrange
    let providers = vec![ProviderId::ChatGPT, ProviderId::Gemini];

    // Act
    let layout = calculate_layout(&providers);

    // Assert
    assert_eq!(layout.provider_count, 2);
    assert_eq!(layout.layout_type, LayoutType::VerticalSplit);
    assert_eq!(layout.panel_dimensions.len(), 2);

    // Left panel (ChatGPT)
    let left = &layout.panel_dimensions[0];
    assert_eq!(left.provider_id, ProviderId::ChatGPT);
    assert_eq!(left.x, 0.0);
    assert_eq!(left.y, 0.0);
    assert_eq!(left.width, 0.5);
    assert_eq!(left.height, 1.0);

    // Right panel (Gemini)
    let right = &layout.panel_dimensions[1];
    assert_eq!(right.provider_id, ProviderId::Gemini);
    assert_eq!(right.x, 0.5);
    assert_eq!(right.y, 0.0);
    assert_eq!(right.width, 0.5);
    assert_eq!(right.height, 1.0);
}

#[test]
fn test_calculate_layout_with_3_providers() {
    // Arrange
    let providers = vec![ProviderId::ChatGPT, ProviderId::Gemini, ProviderId::Claude];

    // Act
    let layout = calculate_layout(&providers);

    // Assert
    assert_eq!(layout.provider_count, 3);
    assert_eq!(layout.layout_type, LayoutType::Grid);
    assert_eq!(layout.panel_dimensions.len(), 3);

    // Top-left panel (ChatGPT)
    let top_left = &layout.panel_dimensions[0];
    assert_eq!(top_left.provider_id, ProviderId::ChatGPT);
    assert_eq!(top_left.x, 0.0);
    assert_eq!(top_left.y, 0.0);
    assert_eq!(top_left.width, 0.5);
    assert_eq!(top_left.height, 0.5);

    // Top-right panel (Gemini)
    let top_right = &layout.panel_dimensions[1];
    assert_eq!(top_right.provider_id, ProviderId::Gemini);
    assert_eq!(top_right.x, 0.5);
    assert_eq!(top_right.y, 0.0);
    assert_eq!(top_right.width, 0.5);
    assert_eq!(top_right.height, 0.5);

    // Bottom panel (Claude) - full width
    let bottom = &layout.panel_dimensions[2];
    assert_eq!(bottom.provider_id, ProviderId::Claude);
    assert_eq!(bottom.x, 0.0);
    assert_eq!(bottom.y, 0.5);
    assert_eq!(bottom.width, 1.0);
    assert_eq!(bottom.height, 0.5);
}

#[test]
#[should_panic(expected = "Provider count must be between 1 and 3")]
fn test_calculate_layout_with_0_providers_panics() {
    let providers: Vec<ProviderId> = vec![];
    calculate_layout(&providers);
}

#[test]
#[should_panic(expected = "Provider count must be between 1 and 3")]
fn test_calculate_layout_with_4_providers_panics() {
    let providers = vec![
        ProviderId::ChatGPT,
        ProviderId::Gemini,
        ProviderId::Claude,
        ProviderId::ChatGPT, // Duplicate to make 4
    ];
    calculate_layout(&providers);
}
