// Unit tests for JavaScript injection script generation
// T074-T075: Verify script generation includes selector logic and submit triggers

use chenchen_lib::injection::script_builder;

#[test]
fn test_generate_injection_script_includes_input_selector_logic() {
    // T074: Script should include logic to iterate through input selectors
    let input_selectors = vec![
        "textarea[data-id='root']".to_string(),
        "textarea[placeholder*='Message']".to_string(),
    ];
    let submit_selectors = vec!["button[data-testid='send-button']".to_string()];
    let prompt = "Test prompt";

    let script =
        script_builder::generate_injection_script(&input_selectors, &submit_selectors, prompt);

    // Verify script contains input selectors
    assert!(
        script.contains("textarea[data-id='root']"),
        "Script should contain first input selector"
    );
    assert!(
        script.contains("textarea[placeholder*='Message']"),
        "Script should contain second input selector"
    );

    // Verify script contains selector iteration logic
    assert!(
        script.contains("querySelector") || script.contains("querySelectorAll"),
        "Script should use querySelector methods"
    );

    // Verify script contains the prompt text
    assert!(
        script.contains("Test prompt"),
        "Script should contain the prompt text"
    );

    // Verify script has input value setting
    assert!(
        script.contains("value") || script.contains("textContent") || script.contains("innerText"),
        "Script should set element value/content"
    );
}

#[test]
fn test_generate_injection_script_includes_submit_button_trigger() {
    // T075: Script should include submit button click trigger
    let input_selectors = vec!["textarea".to_string()];
    let submit_selectors = vec![
        "button[data-testid='send-button']".to_string(),
        "button[aria-label='Send']".to_string(),
    ];
    let prompt = "Test";

    let script =
        script_builder::generate_injection_script(&input_selectors, &submit_selectors, prompt);

    // Verify script contains submit selectors
    assert!(
        script.contains("button[data-testid='send-button']"),
        "Script should contain first submit selector"
    );
    assert!(
        script.contains("button[aria-label='Send']"),
        "Script should contain second submit selector"
    );

    // Verify script contains click trigger
    assert!(
        script.contains("click"),
        "Script should call click() on submit button"
    );
}

#[test]
fn test_generate_injection_script_handles_element_not_found() {
    // T083: Script should report when elements aren't found
    let input_selectors = vec!["input".to_string()];
    let submit_selectors = vec!["button".to_string()];
    let prompt = "Test";

    let script =
        script_builder::generate_injection_script(&input_selectors, &submit_selectors, prompt);

    // Verify script has error handling
    assert!(
        script.contains("null")
            || script.contains("!")
            || script.contains("error")
            || script.contains("Error"),
        "Script should check for null/missing elements"
    );

    // Verify script returns result information
    assert!(
        script.contains("return") || script.contains("result"),
        "Script should return execution result"
    );
}

#[test]
fn test_generate_injection_script_tries_selectors_in_order() {
    // T080: Script should try each selector until one works
    let input_selectors = vec![
        "selector1".to_string(),
        "selector2".to_string(),
        "selector3".to_string(),
    ];
    let submit_selectors = vec!["button".to_string()];
    let prompt = "Test";

    let script =
        script_builder::generate_injection_script(&input_selectors, &submit_selectors, prompt);

    // Verify all selectors appear in order
    let selector1_pos = script.find("selector1");
    let selector2_pos = script.find("selector2");
    let selector3_pos = script.find("selector3");

    assert!(selector1_pos.is_some(), "Should include selector1");
    assert!(selector2_pos.is_some(), "Should include selector2");
    assert!(selector3_pos.is_some(), "Should include selector3");

    // Verify they appear in order
    assert!(
        selector1_pos < selector2_pos,
        "selector1 should appear before selector2"
    );
    assert!(
        selector2_pos < selector3_pos,
        "selector2 should appear before selector3"
    );
}

#[test]
fn test_generate_injection_script_escapes_special_characters() {
    // Verify script properly escapes quotes and special characters in prompt
    let input_selectors = vec!["input".to_string()];
    let submit_selectors = vec!["button".to_string()];
    let prompt = r#"Test with "quotes" and 'apostrophes' and \backslashes"#;

    let script =
        script_builder::generate_injection_script(&input_selectors, &submit_selectors, prompt);

    // Script should be valid JavaScript (no syntax errors from unescaped quotes)
    // We can't run it, but we can check it doesn't have obvious issues
    assert!(
        !script.contains(r#"value = "Test with "quotes""#),
        "Should escape quotes properly"
    );
}
