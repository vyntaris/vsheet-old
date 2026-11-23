use super::Dialog;

/// Help ▸ About VSheet: the app, its contributors and the AI models that helped. The credits
/// are compiled in (`crate::credits`, docs/contributors.md).
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            ui.heading("VSheet");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.add_space(4.0);
            ui.label("The spreadsheet of Vyntaris Office, based on GridCraft by the ArtCraft Team.");
            ui.hyperlink_to("github.com/vyntaris/vsheet", "https://github.com/vyntaris/vsheet");
            ui.hyperlink_to("GridCraft", "https://github.com/storytold/gridcraft");
        }
    }
}
