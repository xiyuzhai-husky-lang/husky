use ::ui::{settings::*, ui::*, *};

#[derive(SettingsUi)]
struct VsCodeSettings {
    editor: EditorSettings,
    workbench: WorkbenchSettings,
    terminal: TerminalSettings,
}

#[derive(SettingSectionUi)]
struct EditorSettings {
    font: FontSettings,
    cursor: CursorSettings,
    find: FindSettings,
}

#[derive(SettingSubsectionUi)]
struct FontSettings {
    size: u32,
    family: String,
    line_height: f32,
}

#[derive(SettingSubsectionUi)]
struct CursorSettings {
    style: String,
    blink: bool,
    width: u32,
}

#[derive(SettingSubsectionUi)]
struct FindSettings {
    case_sensitive: bool,
    whole_word: bool,
    regex: bool,
}

#[derive(SettingSectionUi)]
struct WorkbenchSettings {
    appearance: AppearanceSettings,
    sidebar: SidebarSettings,
}

#[derive(SettingSubsectionUi)]
struct AppearanceSettings {
    color_theme: String,
    icon_theme: String,
}

#[derive(SettingSubsectionUi)]
struct SidebarSettings {
    position: String,
    visible: bool,
}

#[derive(SettingSectionUi)]
struct TerminalSettings {
    integrated: IntegratedTerminalSettings,
}

#[derive(SettingSubsectionUi)]
struct IntegratedTerminalSettings {
    shell_windows: String,
    shell_linux: String,
    shell_osx: String,
}

#[test]
fn vscode_settings_ui_test() {
    let mut settings = VsCodeSettings {
        editor: EditorSettings {
            font: FontSettings {
                size: 14,
                family: "Consolas".to_string(),
                line_height: 1.5,
            },
            cursor: CursorSettings {
                style: "line".to_string(),
                blink: true,
                width: 2,
            },
            find: FindSettings {
                case_sensitive: false,
                whole_word: false,
                regex: false,
            },
        },
        workbench: WorkbenchSettings {
            appearance: AppearanceSettings {
                color_theme: "Monokai".to_string(),
                icon_theme: "vs-seti".to_string(),
            },
            sidebar: SidebarSettings {
                position: "left".to_string(),
                visible: true,
            },
        },
        terminal: TerminalSettings {
            integrated: IntegratedTerminalSettings {
                shell_windows: "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"
                    .to_string(),
                shell_linux: "/bin/bash".to_string(),
                shell_osx: "/bin/zsh".to_string(),
            },
        },
    };

    let mut sections = Vec::new();
    settings.for_each_section(&mut |name, _section| {
        sections.push(name.to_string());
    });
    assert_eq!(sections, vec!["Editor", "Workbench", "Terminal"]);

    // Test editor section
    let mut editor_subsections = Vec::new();
    settings
        .editor
        .for_each_subsection(&mut |name, _subsection| {
            editor_subsections.push(name.to_string());
        });
    assert_eq!(editor_subsections, vec!["Font", "Cursor", "Find"]);

    // Test font subsection
    let mut font_items = Vec::new();
    settings.editor.font.for_each_item(&mut |name, _item| {
        font_items.push(name.to_string());
    });
    assert_eq!(font_items, vec!["Size", "Family", "Line height"]);

    // Additional tests can be added for other sections and subsections
}
