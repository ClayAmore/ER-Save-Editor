use std::sync::RwLock;

use eframe::egui::Context;

use crate::vm::vm::ViewModel;

#[allow(unused)]
#[derive(Default, Clone)]
pub enum NotificationType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

pub static NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(Vec::new());

type NotificationCallback = fn(ctx: &Context, vm: &mut ViewModel) -> ();
type NotificationCallbacks = Vec<(String, NotificationCallback)>;

#[allow(unused)]
pub(crate) enum NotificationButtons<S: Into<String>> {
    None,
    Button((S, NotificationCallback)),
    Group(Vec<(S, NotificationCallback)>),
}

#[derive(Default, Clone)]
pub(crate) struct Notification {
    pub notification_type: NotificationType,
    pub text: String,
    pub buttons: NotificationCallbacks,
}

impl Notification {
    pub(crate) fn new(
        notification_type: NotificationType,
        text: impl Into<String>,
        buttons: NotificationButtons<impl Into<String>>,
    ) -> Self {
        Notification {
            notification_type,
            text: text.into(),
            buttons: match buttons {
                NotificationButtons::None => Vec::new(),
                NotificationButtons::Button(button) => vec![(button.0.into(), button.1)],
                NotificationButtons::Group(buttons) => buttons
                    .into_iter()
                    .map(|(button_text, callback)| (button_text.into(), callback))
                    .collect(),
            },
        }
    }
}
