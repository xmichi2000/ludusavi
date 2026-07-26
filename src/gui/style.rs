use iced::{
    Background, Border, Color, Shadow, Vector,
    widget::{button, checkbox, container, pick_list, scrollable, text_editor, text_input},
};

use crate::{resource::config, scan::ScanChange};

macro_rules! rgb8 {
    ($r:expr, $g:expr, $b:expr) => {
        Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

trait ColorExt {
    fn alpha(self, alpha: f32) -> Color;
}

impl ColorExt for Color {
    fn alpha(mut self, alpha: f32) -> Self {
        self.a = alpha;
        self
    }
}

pub struct Theme {
    source: config::Theme,
    background: Color,
    field: Color,
    text: Color,
    text_inverted: Color,
    text_button: Color,
    text_skipped: Color,
    text_selection: Color,
    positive: Color,
    negative: Color,
    disabled: Color,
    navigation: Color,
    added: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::from(config::Theme::Light)
    }
}

impl From<config::Theme> for Theme {
    fn from(source: config::Theme) -> Self {
        match source {
            // One accent hue throughout, layered surfaces, and muted status colours.
            // See docs/design-system.md.
            config::Theme::Light => Self {
                source,
                background: rgb8!(250, 250, 252),
                field: rgb8!(232, 233, 238),
                text: rgb8!(24, 25, 30),
                text_inverted: Color::WHITE,
                text_button: Color::WHITE,
                text_skipped: rgb8!(24, 25, 30),
                text_selection: rgb8!(206, 199, 255),
                positive: rgb8!(101, 84, 235),
                negative: rgb8!(206, 62, 62),
                disabled: rgb8!(160, 162, 173),
                navigation: rgb8!(101, 84, 235),
                added: rgb8!(38, 166, 106),
            },
            config::Theme::Dark => Self {
                source,
                // Slightly blue-black rather than flat grey, with a lifted surface.
                background: rgb8!(20, 21, 26),
                field: rgb8!(38, 40, 48),
                text: rgb8!(238, 239, 244),
                text_inverted: rgb8!(20, 21, 26),
                text_button: Color::WHITE,
                text_skipped: rgb8!(238, 239, 244),
                text_selection: rgb8!(70, 60, 140),
                positive: rgb8!(124, 108, 246),
                negative: rgb8!(224, 92, 92),
                disabled: rgb8!(110, 114, 128),
                navigation: rgb8!(124, 108, 246),
                added: rgb8!(60, 190, 128),
            },
        }
    }
}

impl iced::theme::Base for Theme {
    fn default(_preference: iced::theme::Mode) -> Self {
        <Theme as Default>::default()
    }

    fn mode(&self) -> iced::theme::Mode {
        match self.source {
            config::Theme::Light => iced::theme::Mode::Light,
            config::Theme::Dark => iced::theme::Mode::Dark,
        }
    }

    fn base(&self) -> iced::theme::Style {
        iced::theme::Style {
            background_color: self.background,
            text_color: self.text,
        }
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        None
    }

    fn name(&self) -> &str {
        match self.source {
            config::Theme::Light => "light",
            config::Theme::Dark => "dark",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Text {
    #[default]
    Default,
    Failure,
}
impl iced::widget::text::Catalog for Theme {
    type Class<'a> = Text;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, item: &Self::Class<'_>) -> iced::widget::text::Style {
        match item {
            Text::Default => iced::widget::text::Style { color: None },
            Text::Failure => iced::widget::text::Style {
                color: Some(self.negative),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Menu;
impl iced::widget::overlay::menu::Catalog for Theme {
    type Class<'a> = Menu;

    fn default<'a>() -> <Self as iced::overlay::menu::Catalog>::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &<Self as iced::overlay::menu::Catalog>::Class<'_>) -> iced::overlay::menu::Style {
        iced::overlay::menu::Style {
            background: self.field.into(),
            border: Border {
                color: self.text.alpha(0.5),
                width: 1.0,
                radius: 5.0.into(),
            },
            text_color: self.text,
            selected_background: self.positive.into(),
            selected_text_color: Color::WHITE,
            shadow: Shadow::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Button {
    #[default]
    Primary,
    /// Everything that isn't the one primary action of a view.
    /// See docs/design-system.md.
    Secondary,
    Negative,
    GameActionPrimary,
    GameListEntryTitle,
    GameListEntryTitleFailed,
    GameListEntryTitleDisabled,
    GameListEntryTitleUnscanned,
    NavButtonActive,
    NavButtonInactive,
    Badge,
    Bare,
}
impl button::Catalog for Theme {
    type Class<'a> = Button;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let active = button::Style {
            background: match class {
                Button::Primary | Button::GameActionPrimary => Some(self.positive.into()),
                // A list item is text, not a filled control. See docs/design-system.md.
                Button::GameListEntryTitle
                | Button::GameListEntryTitleFailed
                | Button::GameListEntryTitleDisabled
                | Button::GameListEntryTitleUnscanned => None,
                Button::Negative => Some(self.negative.into()),
                Button::NavButtonActive => Some(self.navigation.alpha(0.9).into()),
                Button::NavButtonInactive => None,
                Button::Secondary => None,
                Button::Badge => None,
                Button::Bare => None,
            },
            border: Border {
                color: match class {
                    Button::NavButtonActive | Button::NavButtonInactive => self.navigation,
                    Button::Secondary => self.text.alpha(0.35),
                    _ => Color::TRANSPARENT,
                },
                width: match class {
                    Button::NavButtonActive | Button::NavButtonInactive | Button::Secondary => 1.0,
                    _ => 0.0,
                },
                radius: match class {
                    Button::GameActionPrimary
                    | Button::GameListEntryTitle
                    | Button::GameListEntryTitleFailed
                    | Button::GameListEntryTitleDisabled
                    | Button::GameListEntryTitleUnscanned
                    | Button::NavButtonActive
                    | Button::NavButtonInactive => 10.0.into(),
                    _ => 4.0.into(),
                },
            },
            text_color: match class {
                Button::GameListEntryTitleDisabled => self.text.alpha(0.45),
                Button::GameListEntryTitleUnscanned => self.text.alpha(0.6),
                Button::GameListEntryTitle => self.text,
                Button::GameListEntryTitleFailed => self.negative,
                Button::NavButtonInactive | Button::Bare | Button::Secondary => self.text,
                _ => self.text_button.alpha(0.8),
            },
            shadow: Shadow::default(),
            snap: true,
        };

        match status {
            button::Status::Active => active,
            button::Status::Hovered => button::Style {
                background: match class {
                    Button::NavButtonActive => Some(self.navigation.alpha(0.95).into()),
                    Button::NavButtonInactive => Some(self.navigation.alpha(0.16).into()),
                    Button::GameListEntryTitle
                    | Button::GameListEntryTitleFailed
                    | Button::GameListEntryTitleDisabled
                    | Button::GameListEntryTitleUnscanned => Some(self.text.alpha(0.07).into()),
                    _ => active.background,
                },
                border: Border {
                    color: match class {
                        Button::NavButtonActive | Button::NavButtonInactive => self.navigation,
                        _ => active.border.color,
                    },
                    width: match class {
                        Button::NavButtonActive | Button::NavButtonInactive => 1.0,
                        _ => active.border.width,
                    },
                    radius: match class {
                        Button::NavButtonActive | Button::NavButtonInactive => 10.0.into(),
                        _ => active.border.radius,
                    },
                },
                text_color: match class {
                    Button::GameListEntryTitleDisabled => self.text.alpha(0.6),
                    Button::GameListEntryTitle
                    | Button::GameListEntryTitleUnscanned
                    | Button::NavButtonInactive
                    | Button::Secondary => self.text,
                    Button::GameListEntryTitleFailed => self.negative,
                    Button::Bare => self.text.alpha(0.9),
                    _ => self.text_button,
                },
                shadow: Shadow::default(),
                snap: true,
            },
            button::Status::Pressed => button::Style {
                shadow: Shadow {
                    offset: Vector::default(),
                    ..active.shadow
                },
                ..active
            },
            button::Status::Disabled => button::Style {
                shadow: Shadow {
                    offset: Vector::default(),
                    ..active.shadow
                },
                background: active.background.map(|background| match background {
                    Background::Color(color) => Background::Color(Color {
                        a: color.a * 0.5,
                        ..color
                    }),
                    Background::Gradient(gradient) => Background::Gradient(gradient.scale_alpha(0.5)),
                }),
                text_color: Color {
                    a: active.text_color.a * 0.5,
                    ..active.text_color
                },
                ..active
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Container {
    #[default]
    Wrapper,
    Primary,
    ModalForeground,
    ModalBackground,
    GameListEntry,
    Badge,
    BadgeActivated,
    BadgeFaded,
    ChangeBadge {
        change: ScanChange,
        faded: bool,
    },
    DisabledBackup,
    Notification,
    Tooltip,
}
impl container::Catalog for Theme {
    type Class<'a> = Container;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        container::Style {
            background: Some(match class {
                Container::Wrapper => Color::TRANSPARENT.into(),
                Container::GameListEntry => self.field.alpha(0.55).into(),
                Container::ModalBackground => self.field.alpha(0.75).into(),
                Container::Notification => self.field.alpha(0.5).into(),
                Container::Tooltip => self.field.into(),
                Container::DisabledBackup => self.disabled.into(),
                Container::BadgeActivated => self.negative.into(),
                Container::Badge => self.text.alpha(0.1).into(),
                _ => self.background.into(),
            }),
            border: Border {
                color: match class {
                    Container::Wrapper => Color::TRANSPARENT,
                    Container::GameListEntry => Color::TRANSPARENT,
                    Container::Notification => self.field,
                    Container::ChangeBadge { change, faded } => {
                        if *faded {
                            self.disabled
                        } else {
                            match change {
                                ScanChange::New => self.added,
                                ScanChange::Different => self.positive,
                                ScanChange::Removed => self.negative,
                                ScanChange::Same | ScanChange::Unknown => self.disabled,
                            }
                        }
                    }
                    Container::BadgeActivated => self.negative,
                    Container::Badge => Color::TRANSPARENT,
                    Container::ModalForeground | Container::BadgeFaded => self.disabled,
                    _ => self.text,
                },
                width: match class {
                    Container::ModalForeground
                    | Container::Badge
                    | Container::BadgeActivated
                    | Container::BadgeFaded
                    | Container::ChangeBadge { .. }
                    | Container::Notification => 1.0,
                    _ => 0.0,
                },
                radius: match class {
                    Container::ModalForeground
                    | Container::GameListEntry
                    | Container::Badge
                    | Container::BadgeActivated
                    | Container::BadgeFaded
                    | Container::ChangeBadge { .. }
                    | Container::DisabledBackup => 10.0.into(),
                    Container::Notification | Container::Tooltip => 20.0.into(),
                    _ => 0.0.into(),
                },
            },
            text_color: match class {
                Container::Wrapper => None,
                Container::DisabledBackup => Some(self.text_inverted),
                Container::ChangeBadge { change, faded } => {
                    if *faded {
                        Some(self.disabled)
                    } else {
                        match change {
                            ScanChange::New => Some(self.added),
                            ScanChange::Different => Some(self.positive),
                            ScanChange::Removed => Some(self.negative),
                            ScanChange::Same | ScanChange::Unknown => Some(self.disabled),
                        }
                    }
                }
                Container::BadgeActivated => Some(self.text_button),
                Container::BadgeFaded => Some(self.disabled),
                _ => Some(self.text),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.0,
            },
            snap: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Scrollable;
impl scrollable::Catalog for Theme {
    type Class<'a> = Scrollable;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        let active = scrollable::Style {
            auto_scroll: scrollable::AutoScroll {
                background: self.background.into(),
                border: Border::default(),
                shadow: Shadow::default(),
                icon: self.text,
            },
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: Some(Color::TRANSPARENT.into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                scroller: scrollable::Scroller {
                    background: self.text.alpha(0.7).into(),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                },
            },
            horizontal_rail: scrollable::Rail {
                background: Some(Color::TRANSPARENT.into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                scroller: scrollable::Scroller {
                    background: self.text.alpha(0.7).into(),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                },
            },
            gap: None,
        };

        match status {
            scrollable::Status::Active { .. } => active,
            scrollable::Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                ..
            } => {
                if !is_horizontal_scrollbar_hovered && !is_vertical_scrollbar_hovered {
                    return active;
                }

                scrollable::Style {
                    vertical_rail: scrollable::Rail {
                        background: Some(self.text.alpha(0.4).into()),
                        border: Border {
                            color: self.text.alpha(0.8),
                            ..active.vertical_rail.border
                        },
                        ..active.vertical_rail
                    },
                    horizontal_rail: scrollable::Rail {
                        background: Some(self.text.alpha(0.4).into()),
                        border: Border {
                            color: self.text.alpha(0.8),
                            ..active.horizontal_rail.border
                        },
                        ..active.horizontal_rail
                    },
                    ..active
                }
            }
            scrollable::Status::Dragged { .. } => self.style(
                _class,
                scrollable::Status::Hovered {
                    is_horizontal_scrollbar_hovered: true,
                    is_vertical_scrollbar_hovered: true,
                    is_horizontal_scrollbar_disabled: false,
                    is_vertical_scrollbar_disabled: false,
                },
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PickList {
    #[default]
    Primary,
    Backup,
    Popup,
}
impl pick_list::Catalog for Theme {
    type Class<'a> = PickList;

    fn default<'a>() -> <Self as pick_list::Catalog>::Class<'a> {
        Default::default()
    }

    fn style(&self, class: &<Self as pick_list::Catalog>::Class<'_>, status: pick_list::Status) -> pick_list::Style {
        let active = pick_list::Style {
            border: Border {
                color: self.text.alpha(0.7),
                width: 1.0,
                radius: match class {
                    PickList::Primary => 5.0.into(),
                    PickList::Backup | PickList::Popup => 10.0.into(),
                },
            },
            background: self.field.alpha(0.6).into(),
            text_color: self.text,
            placeholder_color: iced::Color::BLACK,
            handle_color: self.text,
        };

        match status {
            pick_list::Status::Active => active,
            pick_list::Status::Hovered => pick_list::Style {
                background: self.field.into(),
                ..active
            },
            pick_list::Status::Opened { .. } => active,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Checkbox;
impl checkbox::Catalog for Theme {
    type Class<'a> = Checkbox;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        let active = checkbox::Style {
            background: self.field.alpha(0.6).into(),
            icon_color: self.text,
            border: Border {
                color: self.text.alpha(0.6),
                width: 1.0,
                radius: 5.0.into(),
            },
            text_color: Some(self.text),
        };

        match status {
            checkbox::Status::Active { .. } => active,
            checkbox::Status::Hovered { .. } => checkbox::Style {
                background: self.field.into(),
                ..active
            },
            checkbox::Status::Disabled { .. } => checkbox::Style {
                background: match active.background {
                    Background::Color(color) => Background::Color(Color {
                        a: color.a * 0.5,
                        ..color
                    }),
                    Background::Gradient(gradient) => Background::Gradient(gradient.scale_alpha(0.5)),
                },
                ..active
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TextInput;
impl text_input::Catalog for Theme {
    type Class<'a> = TextInput;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let active = text_input::Style {
            background: Color::TRANSPARENT.into(),
            border: Border {
                color: self.text.alpha(0.8),
                width: 1.0,
                radius: 5.0.into(),
            },
            icon: self.negative,
            placeholder: self.text.alpha(0.5),
            value: self.text,
            selection: self.text_selection,
        };

        match status {
            text_input::Status::Active => active,
            text_input::Status::Hovered | text_input::Status::Focused { .. } => text_input::Style {
                border: Border {
                    color: self.text,
                    ..active.border
                },
                ..active
            },
            text_input::Status::Disabled => text_input::Style {
                background: self.disabled.into(),
                value: self.text.alpha(0.5),
                ..active
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProgressBar;
impl iced::widget::progress_bar::Catalog for Theme {
    type Class<'a> = ProgressBar;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &Self::Class<'_>) -> iced::widget::progress_bar::Style {
        iced::widget::progress_bar::Style {
            background: self.disabled.into(),
            bar: self.added.into(),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TextEditor;
impl text_editor::Catalog for Theme {
    type Class<'a> = TextEditor;

    fn default<'a>() -> Self::Class<'a> {
        Default::default()
    }

    fn style(&self, _class: &Self::Class<'_>, status: text_editor::Status) -> text_editor::Style {
        let active = text_editor::Style {
            background: self.field.alpha(0.3).into(),
            border: Border {
                radius: 2.0.into(),
                width: 1.0,
                color: self.field,
            },
            placeholder: self.text_skipped,
            value: self.text,
            selection: self.text_selection,
        };

        match status {
            text_editor::Status::Active => active,
            text_editor::Status::Hovered => text_editor::Style {
                border: Border {
                    color: self.text,
                    ..active.border
                },
                ..active
            },
            text_editor::Status::Focused { .. } => text_editor::Style {
                border: Border {
                    color: self.text,
                    ..active.border
                },
                ..active
            },
            text_editor::Status::Disabled => text_editor::Style {
                background: Background::Color(self.disabled),
                value: active.placeholder,
                ..active
            },
        }
    }
}
