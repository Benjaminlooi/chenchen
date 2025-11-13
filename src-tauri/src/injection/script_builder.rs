// JavaScript code generation for element location and prompt submission
// Generates scripts that try selectors in order until elements are found

/// Generates a JavaScript injection script to submit a prompt to an LLM provider
///
/// The generated script:
/// 1. Tries each input selector in order until an element is found
/// 2. Sets the prompt value in the input element
/// 3. Tries each submit selector in order until a button is found
/// 4. Clicks the submit button
/// 5. Returns a result object with success status
///
/// # Arguments
/// * `input_selectors` - CSS selectors for the prompt input element (tried in order)
/// * `submit_selectors` - CSS selectors for the submit button (tried in order)
/// * `prompt` - The text to inject into the input element
///
/// # Returns
/// A JavaScript code string that can be executed via webview.eval()
pub fn generate_injection_script(
    input_selectors: &[String],
    submit_selectors: &[String],
    prompt: &str,
) -> String {
    // Escape the prompt text for safe JavaScript string embedding
    let escaped_prompt = escape_for_javascript(prompt);

    format!(
        r#"
(function() {{
    try {{
        // Try each input selector until we find an element
        let inputElement = null;
        const inputSelectors = {input_selectors};

        for (let i = 0; i < inputSelectors.length; i++) {{
            const selector = inputSelectors[i];
            inputElement = document.querySelector(selector);
            if (inputElement) {{
                console.log('Found input element with selector:', selector);
                break;
            }}
        }}

        if (!inputElement) {{
            return {{
                success: false,
                error_message: 'Input element not found. Tried selectors: ' + inputSelectors.join(', '),
                element_found: false,
                submit_triggered: false
            }};
        }}

        // Set the prompt value
        // Handle both input/textarea elements and contenteditable divs
        if (inputElement.tagName === 'TEXTAREA' || inputElement.tagName === 'INPUT') {{
            inputElement.value = {escaped_prompt};
            // Trigger input event for frameworks that listen to it
            inputElement.dispatchEvent(new Event('input', {{ bubbles: true }}));
            inputElement.dispatchEvent(new Event('change', {{ bubbles: true }}));
        }} else if (inputElement.isContentEditable || inputElement.getAttribute('contenteditable') === 'true') {{
            inputElement.textContent = {escaped_prompt};
            // Trigger input event for contenteditable elements
            inputElement.dispatchEvent(new Event('input', {{ bubbles: true }}));
        }} else {{
            // Fallback: try setting value
            inputElement.value = {escaped_prompt};
            inputElement.dispatchEvent(new Event('input', {{ bubbles: true }}));
        }}

        console.log('Set prompt value in input element');

        // Small delay to allow any reactive frameworks to process the input
        setTimeout(function() {{
            // Try each submit selector until we find a button
            let submitButton = null;
            const submitSelectors = {submit_selectors};

            for (let i = 0; i < submitSelectors.length; i++) {{
                const selector = submitSelectors[i];
                submitButton = document.querySelector(selector);
                if (submitButton) {{
                    console.log('Found submit button with selector:', selector);
                    break;
                }}
            }}

            if (!submitButton) {{
                return {{
                    success: false,
                    error_message: 'Submit button not found. Tried selectors: ' + submitSelectors.join(', '),
                    element_found: true,
                    submit_triggered: false
                }};
            }}

            // Click the submit button
            submitButton.click();
            console.log('Clicked submit button');

            return {{
                success: true,
                error_message: null,
                element_found: true,
                submit_triggered: true
            }};
        }}, 100);

        // Return success for the input setting part
        return {{
            success: true,
            error_message: null,
            element_found: true,
            submit_triggered: false
        }};

    }} catch (error) {{
        console.error('Injection script error:', error);
        return {{
            success: false,
            error_message: 'JavaScript error: ' + error.message,
            element_found: false,
            submit_triggered: false
        }};
    }}
}})();
"#,
        input_selectors = format_selector_array(input_selectors),
        submit_selectors = format_selector_array(submit_selectors),
        escaped_prompt = escaped_prompt,
    )
}

/// Escapes a string for safe embedding in JavaScript code
/// Handles quotes, backslashes, newlines, and other special characters
fn escape_for_javascript(text: &str) -> String {
    let escaped = text
        .replace('\\', r"\\")   // Backslash must be first
        .replace('"', r#"\""#)   // Escape double quotes
        .replace('\'', r"\'")    // Escape single quotes
        .replace('\n', r"\n")    // Escape newlines
        .replace('\r', r"\r")    // Escape carriage returns
        .replace('\t', r"\t")    // Escape tabs
        .replace('\u{2028}', r"\u2028") // Line separator
        .replace('\u{2029}', r"\u2029"); // Paragraph separator

    // Wrap in double quotes
    format!(r#""{}""#, escaped)
}

/// Formats an array of selectors as a JavaScript array literal
fn format_selector_array(selectors: &[String]) -> String {
    let quoted_selectors: Vec<String> = selectors
        .iter()
        .map(|s| format!(r#""{}""#, s.replace('"', r#"\""#)))
        .collect();

    format!("[{}]", quoted_selectors.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_for_javascript() {
        assert_eq!(
            escape_for_javascript("simple"),
            r#""simple""#
        );

        assert_eq!(
            escape_for_javascript(r#"with "quotes""#),
            r#""with \"quotes\"""#
        );

        assert_eq!(
            escape_for_javascript("with\nnewline"),
            r#""with\nnewline""#
        );
    }

    #[test]
    fn test_format_selector_array() {
        let selectors = vec![
            "input".to_string(),
            "textarea".to_string(),
        ];

        let result = format_selector_array(&selectors);
        assert_eq!(result, r#"["input", "textarea"]"#);
    }

    #[test]
    fn test_generate_script_basic() {
        let script = generate_injection_script(
            &vec!["input".to_string()],
            &vec!["button".to_string()],
            "Hello",
        );

        assert!(script.contains("input"));
        assert!(script.contains("button"));
        assert!(script.contains("Hello"));
        assert!(script.contains("querySelector"));
        assert!(script.contains("click"));
    }
}
