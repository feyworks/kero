use strum::FromRepr;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FromRepr)]
pub enum CursorIcon {
    #[default]
    Default,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

impl Into<winit::window::CursorIcon> for CursorIcon {
    fn into(self) -> winit::window::CursorIcon {
        type Icon = winit::window::CursorIcon;
        match self {
            CursorIcon::Default => Icon::Default,
            CursorIcon::ContextMenu => Icon::ContextMenu,
            CursorIcon::Help => Icon::Help,
            CursorIcon::Pointer => Icon::Pointer,
            CursorIcon::Progress => Icon::Progress,
            CursorIcon::Wait => Icon::Wait,
            CursorIcon::Cell => Icon::Cell,
            CursorIcon::Crosshair => Icon::Crosshair,
            CursorIcon::Text => Icon::Text,
            CursorIcon::VerticalText => Icon::VerticalText,
            CursorIcon::Alias => Icon::Alias,
            CursorIcon::Copy => Icon::Copy,
            CursorIcon::Move => Icon::Move,
            CursorIcon::NoDrop => Icon::NoDrop,
            CursorIcon::NotAllowed => Icon::NotAllowed,
            CursorIcon::Grab => Icon::Grab,
            CursorIcon::Grabbing => Icon::Grabbing,
            CursorIcon::EResize => Icon::EResize,
            CursorIcon::NResize => Icon::NResize,
            CursorIcon::NeResize => Icon::NeResize,
            CursorIcon::NwResize => Icon::NwResize,
            CursorIcon::SResize => Icon::SResize,
            CursorIcon::SeResize => Icon::SeResize,
            CursorIcon::SwResize => Icon::SwResize,
            CursorIcon::WResize => Icon::WResize,
            CursorIcon::EwResize => Icon::EwResize,
            CursorIcon::NsResize => Icon::NsResize,
            CursorIcon::NeswResize => Icon::NeswResize,
            CursorIcon::NwseResize => Icon::NwseResize,
            CursorIcon::ColResize => Icon::ColResize,
            CursorIcon::RowResize => Icon::RowResize,
            CursorIcon::AllScroll => Icon::AllScroll,
            CursorIcon::ZoomIn => Icon::ZoomIn,
            CursorIcon::ZoomOut => Icon::ZoomOut,
        }
    }
}

#[cfg(feature = "lua")]
impl mlua::FromLua for CursorIcon {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let s = mlua::BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "default" => Self::Default,
            "context-menu" => Self::ContextMenu,
            "help" => Self::Help,
            "pointer" => Self::Pointer,
            "progress" => Self::Progress,
            "wait" => Self::Wait,
            "cell" => Self::Cell,
            "crosshair" => Self::Crosshair,
            "text" => Self::Text,
            "vertical_text" => Self::VerticalText,
            "alias" => Self::Alias,
            "copy" => Self::Copy,
            "move" => Self::Move,
            "no_drop" => Self::NoDrop,
            "not_allowed" => Self::NotAllowed,
            "grab" => Self::Grab,
            "grabbing" => Self::Grabbing,
            "e_resize" => Self::EResize,
            "n_resize" => Self::NResize,
            "ne_resize" => Self::NeResize,
            "nw_resize" => Self::NwResize,
            "s_resize" => Self::SResize,
            "se_resize" => Self::SeResize,
            "sw_resize" => Self::SwResize,
            "w_resize" => Self::WResize,
            "ew_resize" => Self::EwResize,
            "ns_resize" => Self::NsResize,
            "nesw_resize" => Self::NeswResize,
            "nwse_resize" => Self::NwseResize,
            "col_resize" => Self::ColResize,
            "row_resize" => Self::RowResize,
            "all_scroll" => Self::AllScroll,
            "zoom_in" => Self::ZoomIn,
            "zoom_out" => Self::ZoomOut,
            s => {
                return Err(mlua::prelude::LuaError::runtime(format!(
                    "invalid cursor type {s:?}"
                )));
            }
        })
    }
}

#[cfg(feature = "lua")]
impl mlua::IntoLua for CursorIcon {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Self::Default => "default",
            Self::ContextMenu => "context_menu",
            Self::Help => "help",
            Self::Pointer => "pointer",
            Self::Progress => "progress",
            Self::Wait => "wait",
            Self::Cell => "cell",
            Self::Crosshair => "crosshair",
            Self::Text => "text",
            Self::VerticalText => "vertical_text",
            Self::Alias => "alias",
            Self::Copy => "copy",
            Self::Move => "move",
            Self::NoDrop => "no_drop",
            Self::NotAllowed => "not_allowed",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
            Self::EResize => "e_resize",
            Self::NResize => "n_resize",
            Self::NeResize => "ne_resize",
            Self::NwResize => "nw_resize",
            Self::SResize => "s_resize",
            Self::SeResize => "se_resize",
            Self::SwResize => "sw_resize",
            Self::WResize => "w_resize",
            Self::EwResize => "ew_resize",
            Self::NsResize => "ns_resize",
            Self::NeswResize => "nesw_resize",
            Self::NwseResize => "nwse_resize",
            Self::ColResize => "col_resize",
            Self::RowResize => "row_resize",
            Self::AllScroll => "all_scroll",
            Self::ZoomIn => "zoom_in",
            Self::ZoomOut => "zoom_out",
        }
        .into_lua(lua)
    }
}
