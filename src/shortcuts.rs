// SPDX-License-Identifier: GPL-3.0-only

//! Key bindings the terminal handles itself.
//!
//! Everything that is not specific to the terminal - how a binding is written,
//! how the user's map is layered over these defaults, how a combination is
//! displayed and captured - lives in [`cosmic::shortcuts`] and is shared with the
//! rest of the stack, so that the settings app can edit this map without knowing
//! anything about terminals.

use cosmic::shortcuts::{ShortcutAction, ShortcutsConfig as SharedConfig};
use cosmic::widget::menu::key_bind::KeyBind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Action, fl};

pub use cosmic::shortcuts::{
    Binding, BindingSource, ModifierName, binding_display, binding_from_key,
};

/// The stored map: what goes into the `shortcuts_custom` config key.
pub type Shortcuts = cosmic::shortcuts::Shortcuts<KeyBindAction>;

/// The user's map layered over [`fallback_shortcuts`].
pub type ShortcutsConfig = SharedConfig<KeyBindAction>;

/// Builds the runtime configuration from the user's stored map.
#[must_use]
pub fn shortcuts_config(custom: Shortcuts) -> ShortcutsConfig {
    ShortcutsConfig::new(fallback_shortcuts(), custom)
}

/// Resolves both maps into the bindings the terminal matches key presses against.
#[must_use]
pub fn key_binds(config: &ShortcutsConfig) -> HashMap<KeyBind, Action> {
    config.key_binds(|action| action.to_action())
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum KeyBindAction {
    Disable,
    ClearScrollback,
    Copy,
    CopyOrSigint,
    Find,
    PaneFocusDown,
    PaneFocusLeft,
    PaneFocusRight,
    PaneFocusUp,
    PaneSplitHorizontal,
    PaneSplitVertical,
    PaneToggleMaximized,
    Paste,
    PastePrimary,
    #[cfg_attr(not(feature = "password_manager"), allow(dead_code))]
    PasswordManager,
    SelectAll,
    Settings,
    TabActivate0,
    TabActivate1,
    TabActivate2,
    TabActivate3,
    TabActivate4,
    TabActivate5,
    TabActivate6,
    TabActivate7,
    TabActivate8,
    TabClose,
    TabNew,
    TabNext,
    TabPrev,
    ToggleFullscreen,
    WindowClose,
    WindowNew,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

impl ShortcutAction for KeyBindAction {
    fn is_disable(&self) -> bool {
        matches!(self, Self::Disable)
    }
}

impl KeyBindAction {
    fn to_action(self) -> Option<Action> {
        Some(match self {
            Self::Disable => return None,
            Self::ClearScrollback => Action::ClearScrollback,
            Self::Copy => Action::Copy,
            Self::CopyOrSigint => Action::CopyOrSigint,
            Self::Find => Action::Find,
            Self::PaneFocusDown => Action::PaneFocusDown,
            Self::PaneFocusLeft => Action::PaneFocusLeft,
            Self::PaneFocusRight => Action::PaneFocusRight,
            Self::PaneFocusUp => Action::PaneFocusUp,
            Self::PaneSplitHorizontal => Action::PaneSplitHorizontal,
            Self::PaneSplitVertical => Action::PaneSplitVertical,
            Self::PaneToggleMaximized => Action::PaneToggleMaximized,
            Self::Paste => Action::Paste,
            Self::PastePrimary => Action::PastePrimary,
            Self::SelectAll => Action::SelectAll,
            Self::Settings => Action::Settings,
            Self::TabActivate0 => Action::TabActivate0,
            Self::TabActivate1 => Action::TabActivate1,
            Self::TabActivate2 => Action::TabActivate2,
            Self::TabActivate3 => Action::TabActivate3,
            Self::TabActivate4 => Action::TabActivate4,
            Self::TabActivate5 => Action::TabActivate5,
            Self::TabActivate6 => Action::TabActivate6,
            Self::TabActivate7 => Action::TabActivate7,
            Self::TabActivate8 => Action::TabActivate8,
            Self::TabClose => Action::TabClose,
            Self::TabNew => Action::TabNew,
            Self::TabNext => Action::TabNext,
            Self::TabPrev => Action::TabPrev,
            Self::ToggleFullscreen => Action::ToggleFullscreen,
            Self::WindowClose => Action::WindowClose,
            Self::WindowNew => Action::WindowNew,
            Self::ZoomIn => Action::ZoomIn,
            Self::ZoomOut => Action::ZoomOut,
            Self::ZoomReset => Action::ZoomReset,
            Self::PasswordManager => {
                #[cfg(feature = "password_manager")]
                {
                    Action::PasswordManager
                }
                #[cfg(not(feature = "password_manager"))]
                {
                    return None;
                }
            }
        })
    }
}

pub fn action_label(action: KeyBindAction) -> String {
    match action {
        KeyBindAction::Disable => fl!("disable"),
        KeyBindAction::ClearScrollback => fl!("clear-scrollback"),
        KeyBindAction::Copy => fl!("copy"),
        KeyBindAction::CopyOrSigint => fl!("copy-or-sigint"),
        KeyBindAction::Find => fl!("find"),
        KeyBindAction::PaneFocusDown => fl!("focus-pane-down"),
        KeyBindAction::PaneFocusLeft => fl!("focus-pane-left"),
        KeyBindAction::PaneFocusRight => fl!("focus-pane-right"),
        KeyBindAction::PaneFocusUp => fl!("focus-pane-up"),
        KeyBindAction::PaneSplitHorizontal => fl!("split-horizontal"),
        KeyBindAction::PaneSplitVertical => fl!("split-vertical"),
        KeyBindAction::PaneToggleMaximized => fl!("pane-toggle-maximize"),
        KeyBindAction::Paste => fl!("paste"),
        KeyBindAction::PastePrimary => fl!("paste-primary"),
        KeyBindAction::PasswordManager => fl!("password-manager"),
        KeyBindAction::SelectAll => fl!("select-all"),
        KeyBindAction::Settings => fl!("settings"),
        KeyBindAction::TabActivate0 => fl!("tab-activate", number = 1),
        KeyBindAction::TabActivate1 => fl!("tab-activate", number = 2),
        KeyBindAction::TabActivate2 => fl!("tab-activate", number = 3),
        KeyBindAction::TabActivate3 => fl!("tab-activate", number = 4),
        KeyBindAction::TabActivate4 => fl!("tab-activate", number = 5),
        KeyBindAction::TabActivate5 => fl!("tab-activate", number = 6),
        KeyBindAction::TabActivate6 => fl!("tab-activate", number = 7),
        KeyBindAction::TabActivate7 => fl!("tab-activate", number = 8),
        KeyBindAction::TabActivate8 => fl!("tab-activate", number = 9),
        KeyBindAction::TabClose => fl!("close-tab"),
        KeyBindAction::TabNew => fl!("new-tab"),
        KeyBindAction::TabNext => fl!("next-tab"),
        KeyBindAction::TabPrev => fl!("previous-tab"),
        KeyBindAction::ToggleFullscreen => fl!("toggle-fullscreen"),
        KeyBindAction::WindowClose => fl!("close-window"),
        KeyBindAction::WindowNew => fl!("new-window"),
        KeyBindAction::ZoomIn => fl!("zoom-in"),
        KeyBindAction::ZoomOut => fl!("zoom-out"),
        KeyBindAction::ZoomReset => fl!("zoom-reset"),
    }
}

pub struct ShortcutGroup {
    pub title: String,
    pub actions: Vec<KeyBindAction>,
}

pub fn shortcut_groups() -> Vec<ShortcutGroup> {
    let mut groups = Vec::new();
    groups.push(ShortcutGroup {
        title: fl!("shortcut-group-clipboard"),
        actions: vec![
            KeyBindAction::SelectAll,
            KeyBindAction::Copy,
            KeyBindAction::CopyOrSigint,
            KeyBindAction::Paste,
            KeyBindAction::PastePrimary,
            KeyBindAction::Find,
        ],
    });
    groups.push(ShortcutGroup {
        title: fl!("shortcut-group-tabs"),
        actions: vec![
            KeyBindAction::TabNew,
            KeyBindAction::TabClose,
            KeyBindAction::TabNext,
            KeyBindAction::TabPrev,
            KeyBindAction::TabActivate0,
            KeyBindAction::TabActivate1,
            KeyBindAction::TabActivate2,
            KeyBindAction::TabActivate3,
            KeyBindAction::TabActivate4,
            KeyBindAction::TabActivate5,
            KeyBindAction::TabActivate6,
            KeyBindAction::TabActivate7,
            KeyBindAction::TabActivate8,
        ],
    });
    groups.push(ShortcutGroup {
        title: fl!("splits"),
        actions: vec![
            KeyBindAction::PaneSplitHorizontal,
            KeyBindAction::PaneSplitVertical,
            KeyBindAction::PaneToggleMaximized,
            KeyBindAction::PaneFocusLeft,
            KeyBindAction::PaneFocusRight,
            KeyBindAction::PaneFocusUp,
            KeyBindAction::PaneFocusDown,
        ],
    });
    groups.push(ShortcutGroup {
        title: fl!("shortcut-group-window"),
        actions: vec![
            KeyBindAction::WindowNew,
            KeyBindAction::WindowClose,
            KeyBindAction::ToggleFullscreen,
            KeyBindAction::Settings,
        ],
    });
    groups.push(ShortcutGroup {
        title: fl!("shortcut-group-zoom"),
        actions: vec![
            KeyBindAction::ZoomIn,
            KeyBindAction::ZoomOut,
            KeyBindAction::ZoomReset,
        ],
    });
    let mut other_actions = vec![KeyBindAction::ClearScrollback];
    #[cfg(feature = "password_manager")]
    other_actions.push(KeyBindAction::PasswordManager);
    groups.push(ShortcutGroup {
        title: fl!("shortcut-group-other"),
        actions: other_actions,
    });
    groups
}

pub fn fallback_shortcuts() -> Shortcuts {
    let mut shortcuts = Shortcuts::new();

    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            shortcuts.0.insert(
                Binding::new([$(ModifierName::$modifier),*], $key),
                KeyBindAction::$action,
            );
        }};
    }

    // Standard key bindings
    bind!([Ctrl, Shift], "A", SelectAll);
    bind!([Ctrl, Shift], "C", Copy);
    bind!([Ctrl], "C", CopyOrSigint);
    bind!([Ctrl, Shift], "F", Find);
    bind!([Ctrl, Shift], "N", WindowNew);
    bind!([Ctrl, Shift], "Q", WindowClose);
    bind!([Ctrl, Shift], "T", TabNew);
    bind!([Ctrl, Shift], "V", Paste);
    bind!([Shift], "Insert", PastePrimary);
    bind!([Ctrl, Shift], "W", TabClose);
    bind!([Ctrl], ",", Settings);
    bind!([], "F11", ToggleFullscreen);

    // Ctrl+Alt+D splits horizontally, Ctrl+Alt+R splits vertically, Ctrl+Shift+X maximizes split
    //TODO: Adjust bindings as desired by UX
    bind!([Ctrl, Alt], "D", PaneSplitHorizontal);
    bind!([Ctrl, Alt], "R", PaneSplitVertical);
    bind!([Ctrl, Shift], "X", PaneToggleMaximized);
    #[cfg(feature = "password_manager")]
    bind!([Ctrl, Alt], "P", PasswordManager);

    // Ctrl+Tab and Ctrl+Shift+Tab cycle through tabs
    // Ctrl+Tab is not a special key for terminals and is free to use
    bind!([Ctrl], "Tab", TabNext);
    bind!([Ctrl, Shift], "Tab", TabPrev);

    // Ctrl+Shift+# activates tabs by index
    bind!([Ctrl, Shift], "1", TabActivate0);
    bind!([Ctrl, Shift], "2", TabActivate1);
    bind!([Ctrl, Shift], "3", TabActivate2);
    bind!([Ctrl, Shift], "4", TabActivate3);
    bind!([Ctrl, Shift], "5", TabActivate4);
    bind!([Ctrl, Shift], "6", TabActivate5);
    bind!([Ctrl, Shift], "7", TabActivate6);
    bind!([Ctrl, Shift], "8", TabActivate7);
    bind!([Ctrl, Shift], "9", TabActivate8);

    // Ctrl+0, Ctrl+-, and Ctrl+= are not special keys for terminals and are free to use
    bind!([Ctrl], "0", ZoomReset);
    bind!([Ctrl], "-", ZoomOut);
    bind!([Ctrl], "=", ZoomIn);
    bind!([Ctrl], "+", ZoomIn);

    // Ctrl+Arrows and Ctrl+HJKL move between splits
    bind!([Ctrl, Shift], "ArrowLeft", PaneFocusLeft);
    bind!([Ctrl, Shift], "H", PaneFocusLeft);
    bind!([Ctrl, Shift], "ArrowDown", PaneFocusDown);
    bind!([Ctrl, Shift], "J", PaneFocusDown);
    bind!([Ctrl, Shift], "ArrowUp", PaneFocusUp);
    bind!([Ctrl, Shift], "K", PaneFocusUp);
    bind!([Ctrl, Shift], "ArrowRight", PaneFocusRight);
    bind!([Ctrl, Shift], "L", PaneFocusRight);

    // CTRL+Alt+L clears the scrollback.
    bind!([Ctrl, Alt], "L", ClearScrollback);

    shortcuts
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shape of the `shortcuts_custom` config key. The settings app writes this
    /// file too, so the spelling is a contract and not an implementation detail.
    const STORED: &str = r#"{
    (modifiers: [Ctrl, Shift], key: "T"): TabNew,
    (modifiers: [Ctrl], key: "q"): Disable,
}"#;

    #[test]
    fn the_stored_shape_round_trips() {
        let parsed: Shortcuts = ron::from_str(STORED).expect("stored shortcuts have to parse");

        assert_eq!(
            parsed.0.get(&Binding::new(
                [ModifierName::Ctrl, ModifierName::Shift],
                "T"
            )),
            Some(&KeyBindAction::TabNew)
        );

        let written = ron::to_string(&parsed).expect("shortcuts have to serialize");
        let reparsed: Shortcuts = ron::from_str(&written).expect("written shortcuts have to parse");
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn disable_drops_a_default_binding() {
        let custom: Shortcuts =
            ron::from_str(r#"{ (modifiers: [Ctrl, Shift], key: "T"): Disable }"#)
                .expect("stored shortcuts have to parse");
        let config = shortcuts_config(custom);

        assert!(
            !key_binds(&config)
                .values()
                .any(|action| matches!(action, Action::TabNew)),
            "Ctrl+Shift+T was disabled, so no bind may open a tab"
        );
    }

    #[test]
    fn every_default_binding_resolves() {
        for (binding, action) in fallback_shortcuts().iter() {
            assert!(
                binding.to_key_bind().is_some(),
                "default binding {binding:?} for {action:?} does not resolve to a key"
            );
        }
    }

    /// The declaration the package ships, which the settings app builds its page
    /// from. It repeats what the code above says, so it can drift; this is what
    /// makes the drift loud.
    #[test]
    fn the_declaration_matches_the_code() {
        #[derive(serde::Deserialize)]
        struct Declaration {
            groups: Vec<Group>,
        }
        #[derive(serde::Deserialize)]
        struct Group {
            actions: Vec<Declared>,
        }
        #[derive(serde::Deserialize)]
        struct Declared {
            /// Quoted in the file: the settings app reads it as text, since it has
            /// no enum to put it in.
            action: String,
            // Named so that RON has somewhere to put it: an ignored map with
            // string keys is not something it can skip past.
            #[allow(dead_code)]
            label: HashMap<String, String>,
            defaults: Vec<Binding>,
        }

        let path = "res/app-shortcuts/fun.wmde.term.ron";
        let text = std::fs::read_to_string(path).expect("the declaration has to ship");
        let declared: Declaration = ron::from_str(&text).expect("the declaration has to parse");

        let mut from_declaration = Shortcuts::new();
        let mut declared_actions = Vec::new();
        for group in &declared.groups {
            for entry in &group.actions {
                let action: KeyBindAction = ron::from_str(&entry.action)
                    .unwrap_or_else(|_| panic!("{} is not an action", entry.action));
                declared_actions.push(action);
                for binding in &entry.defaults {
                    from_declaration.0.insert(binding.clone(), action);
                }
            }
        }

        assert_eq!(
            from_declaration,
            fallback_shortcuts(),
            "{path} no longer lists the same default bindings as fallback_shortcuts()"
        );

        for group in shortcut_groups() {
            for action in group.actions {
                assert!(
                    declared_actions.contains(&action),
                    "{path} is missing {action:?}"
                );
            }
        }
    }

    #[test]
    fn every_default_key_is_spelled_the_way_capture_spells_it() {
        // A captured key comes back upper case for single latin letters. A default
        // spelled any other way could not be replaced by a user binding: the two
        // would sit side by side in the map and both fire.
        for (binding, action) in fallback_shortcuts().iter() {
            let key = cosmic::shortcuts::key_from_string(&binding.key)
                .unwrap_or_else(|| panic!("{action:?}: key {:?} does not resolve", binding.key));
            let spelled = cosmic::shortcuts::key_to_string(&key)
                .unwrap_or_else(|| panic!("{action:?}: key {:?} has no spelling", binding.key));
            assert_eq!(
                binding.key, spelled,
                "{action:?}: a captured key would be stored as {spelled:?}"
            );
        }
    }
}
