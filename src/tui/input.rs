//! Input handling for the visualizer
//! 
//! Processes keyboard events and user interactions.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Input handler for keyboard events
pub struct InputHandler;

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self
    }

    /// Process a key event and return the action
    pub fn handle_key_event(&self, key: KeyEvent) -> InputAction {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => InputAction::Quit,
            KeyCode::Char('m') => InputAction::NextMode,
            KeyCode::Char('p') => InputAction::NextPalette,
            KeyCode::Char('+') | KeyCode::Char('=') => InputAction::IncreaseSensitivity,
            KeyCode::Char('-') => InputAction::DecreaseSensitivity,
            KeyCode::Left => InputAction::SeekBackward,
            KeyCode::Right => InputAction::SeekForward,
            KeyCode::Char(' ') => InputAction::PauseToggle,
            KeyCode::Char('r') => InputAction::ResetView,
            _ => InputAction::None,
        }
    }

    /// Check if a key combination should quit
    pub fn should_quit(&self, key: KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
    }

    /// Get help text for available controls
    pub fn get_help_text() -> &'static str {
        "Controls:\n\
         Q/Esc     - Quit\n\
         M         - Next visualization mode\n\
         P         - Next color palette\n\
         +/=       - Increase sensitivity\n\
         -         - Decrease sensitivity\n\
         ←/→       - Seek backward/forward\n\
         Space     - Pause/resume\n\
         R         - Reset view"
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions that can be triggered by input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputAction {
    /// Quit the application
    Quit,
    /// Cycle to next visualization mode
    NextMode,
    /// Cycle to next color palette
    NextPalette,
    /// Increase sensitivity/amplitude scaling
    IncreaseSensitivity,
    /// Decrease sensitivity/amplitude scaling
    DecreaseSensitivity,
    /// Seek backward in audio
    SeekBackward,
    /// Seek forward in audio
    SeekForward,
    /// Toggle pause/resume
    PauseToggle,
    /// Reset view to default
    ResetView,
    /// No action
    None,
}

impl InputAction {
    /// Check if this is a meaningful action
    pub fn is_action(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Get display name for the action
    pub fn name(&self) -> &'static str {
        match self {
            Self::Quit => "Quit",
            Self::NextMode => "Next Mode",
            Self::NextPalette => "Next Palette",
            Self::IncreaseSensitivity => "Increase Sensitivity",
            Self::DecreaseSensitivity => "Decrease Sensitivity",
            Self::SeekBackward => "Seek Backward",
            Self::SeekForward => "Seek Forward",
            Self::PauseToggle => "Pause/Resume",
            Self::ResetView => "Reset View",
            Self::None => "None",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn test_quit_keys() {
        let handler = InputHandler::new();
        
        assert!(handler.should_quit(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)));
        assert!(handler.should_quit(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
        assert!(!handler.should_quit(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE)));
    }

    #[test]
    fn test_key_actions() {
        let handler = InputHandler::new();
        
        assert_eq!(handler.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE)), InputAction::NextMode);
        assert_eq!(handler.handle_key_event(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE)), InputAction::NextPalette);
        assert_eq!(handler.handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)), InputAction::SeekBackward);
        assert_eq!(handler.handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)), InputAction::SeekForward);
    }

    #[test]
    fn test_action_names() {
        assert_eq!(InputAction::Quit.name(), "Quit");
        assert_eq!(InputAction::NextMode.name(), "Next Mode");
        assert_eq!(InputAction::None.name(), "None");
    }

    #[test]
    fn test_is_action() {
        assert!(!InputAction::None.is_action());
        assert!(InputAction::Quit.is_action());
        assert!(InputAction::NextMode.is_action());
    }

    #[test]
    fn test_help_text() {
        let help = InputHandler::get_help_text();
        
        assert!(help.contains("Quit"));
        assert!(help.contains("Mode"));
        assert!(help.contains("Palette"));
    }
}
