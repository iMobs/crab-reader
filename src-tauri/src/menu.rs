use std::str::FromStr;

use strum::{Display, EnumString};
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

#[derive(Display, EnumString)]
#[strum(serialize_all = "snake_case")]
enum MenuId {
    Preferences,
    AddSubscription,
    NextStory,
    PrevStory,
    ToggleRead,
    ToggleFavorite,
    MarkAllRead,
    SaveOffline,
    ViewBrowser,
    ImportSubscriptions,
    ExportSubscriptions,
}

impl From<MenuId> for String {
    fn from(value: MenuId) -> Self {
        value.to_string()
    }
}

pub fn create_menu() -> Menu {
    let mut main_menu = Menu::new();

    let file_menu = if cfg!(target_os = "macos") {
        Menu::new()
            .add_native_item(MenuItem::About(
                "Crab Reader".to_string(),
                AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new(MenuId::Preferences, "Preferences")
                    .accelerator("CommandOrControl+,"),
            )
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit)
    } else {
        Menu::new()
            .add_item(
                CustomMenuItem::new(MenuId::Preferences, "Preferences")
                    .accelerator("CommandOrControl+,"),
            )
            .add_native_item(MenuItem::Quit)
    };
    main_menu = main_menu.add_submenu(Submenu::new("Crab Reader", file_menu));

    let edit_menu = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        // Paste and match style?
        .add_native_item(MenuItem::SelectAll);
    main_menu = main_menu.add_submenu(Submenu::new("Edit", edit_menu));

    let view_menu = Menu::new().add_native_item(MenuItem::EnterFullScreen);
    main_menu = main_menu.add_submenu(Submenu::new("View", view_menu));

    let window_menu = Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::CloseWindow);
    main_menu = main_menu.add_submenu(Submenu::new("Window", window_menu));

    let subscriptions_menu = Menu::new().add_item(
        CustomMenuItem::new(MenuId::AddSubscription, "Add Subscription")
            .accelerator("CommandOrControl+N"),
    );
    main_menu = main_menu.add_submenu(Submenu::new("Subscriptions", subscriptions_menu));

    let stories_menu = Menu::new()
        .add_item(
            CustomMenuItem::new(MenuId::NextStory, "Next Story").accelerator("CommandOrControl+J"),
        )
        .add_item(
            CustomMenuItem::new(MenuId::PrevStory, "Previous Story")
                .accelerator("CommandOrControl+K"),
        )
        .add_native_item(MenuItem::Separator)
        .add_item(
            CustomMenuItem::new(MenuId::ToggleRead, "Toggle Read")
                .accelerator("CommandOrControl+T"),
        )
        .add_item(
            CustomMenuItem::new(MenuId::ToggleFavorite, "Toggle Favorite")
                .accelerator("CommandOrControl+S"),
        )
        .add_item(CustomMenuItem::new(MenuId::MarkAllRead, "Mark All Read").accelerator("Alt+R"))
        .add_native_item(MenuItem::Separator)
        .add_item(
            CustomMenuItem::new(MenuId::SaveOffline, "Save Offline")
                .accelerator("CommandOrControl+O"),
        )
        .add_item(
            CustomMenuItem::new(MenuId::ViewBrowser, "View in Browser")
                .accelerator("CommandOrControl+B"),
        );
    main_menu = main_menu.add_submenu(Submenu::new("Stories", stories_menu));

    let import_menu = Menu::new()
        .add_item(CustomMenuItem::new(
            MenuId::ImportSubscriptions,
            "Import Subscriptions",
        ))
        .add_item(CustomMenuItem::new(
            MenuId::ExportSubscriptions,
            "Export Subscriptions",
        ));
    main_menu = main_menu.add_submenu(Submenu::new("Import and Export", import_menu));

    // TODO: Language menu

    main_menu
}

pub fn handle_event(event: WindowMenuEvent) {
    let menu_id = event.menu_item_id();
    if let Ok(menu_id) = MenuId::from_str(menu_id) {
        log::debug!("handled menu_id: {}", menu_id);

        match menu_id {
            MenuId::Preferences => {}
            MenuId::AddSubscription => {}
            MenuId::NextStory => {}
            MenuId::PrevStory => {}
            MenuId::ToggleRead => {}
            MenuId::ToggleFavorite => {}
            MenuId::MarkAllRead => {}
            MenuId::SaveOffline => {}
            MenuId::ViewBrowser => {}
            MenuId::ImportSubscriptions => {}
            MenuId::ExportSubscriptions => {}
        }
    } else {
        log::debug!("unhandled menu_id: {}", menu_id);
    }
}
