use crate::structs::{
    Alert, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged, FindStatus,
    LanguageChanged, MeasureWidth, PluginStarted, PluginStopped, ReplaceStatus, ScrollTo, Style,
    ThemeChanged, Update, UpdateCmds,
};
use serde_json::{self, from_value, Value};

#[derive(Debug)]
pub enum RpcOperations {
    Update(Update),
    ScrollTo(ScrollTo),
    DefStyle(Style),
    AvailablePlugins(AvailablePlugins),
    UpdateCmds(UpdateCmds),
    PluginStarted(PluginStarted),
    PluginStopped(PluginStopped),
    ConfigChanged(ConfigChanged),
    ThemeChanged(ThemeChanged),
    Alert(Alert),
    AvailableThemes(AvailableThemes),
    FindStatus(FindStatus),
    ReplaceStatus(ReplaceStatus),
    AvailableLanguages(AvailableLanguages),
    LanguageChanged(LanguageChanged),
    MeasureWidth((u64, MeasureWidth)),
    None,
}

pub fn handle_notification(method: String, params: Value) -> RpcOperations {
    match method.as_str() {
        "update" => RpcOperations::Update(from_value::<Update>(params).unwrap()),
        "scroll_to" => RpcOperations::ScrollTo(from_value::<ScrollTo>(params).unwrap()),
        "def_style" => RpcOperations::DefStyle(from_value::<Style>(params).unwrap()),
        "available_plugins" => {
            RpcOperations::AvailablePlugins(from_value::<AvailablePlugins>(params).unwrap())
        }
        "plugin_started" => {
            RpcOperations::PluginStarted(from_value::<PluginStarted>(params).unwrap())
        }
        "plugin_stopped" => {
            RpcOperations::PluginStopped(from_value::<PluginStopped>(params).unwrap())
        }
        "update_cmds" => RpcOperations::UpdateCmds(from_value::<UpdateCmds>(params).unwrap()),
        "config_changed" => {
            RpcOperations::ConfigChanged(from_value::<ConfigChanged>(params).unwrap())
        }
        "theme_changed" => RpcOperations::ThemeChanged(from_value::<ThemeChanged>(params).unwrap()),
        "alert" => RpcOperations::Alert(from_value::<Alert>(params).unwrap()),
        "available_themes" => {
            RpcOperations::AvailableThemes(from_value::<AvailableThemes>(params).unwrap())
        }
        "find_status" => RpcOperations::FindStatus(from_value::<FindStatus>(params).unwrap()),
        "replace_status" => {
            RpcOperations::ReplaceStatus(from_value::<ReplaceStatus>(params).unwrap())
        }
        "available_languages" => {
            RpcOperations::AvailableLanguages(from_value::<AvailableLanguages>(params).unwrap())
        }
        "language_changed" => {
            RpcOperations::LanguageChanged(from_value::<LanguageChanged>(params).unwrap())
        }
        _ => {
            RpcOperations::None
            // unreachable!("Unknown method {}", method)
        }
    }
}
