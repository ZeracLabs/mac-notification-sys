//! Custom structs and enums for mac-notification-sys.

use crate::utilities::check_sound;
use objc_foundation::{INSDictionary, INSString, NSDictionary, NSString};
use objc_id::Id;
use std::ops::Deref;

/// Possible actions accessible through the main button of the notification
pub enum MainButton<'a> {
    /// Display a single action with the given name
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::SingleAction("Action name");
    /// ```
    SingleAction(&'a str),
    /// Display a dropdown with the given title, with a list of actions with given names
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::DropdownActions("Dropdown name", &["Action 1", "Action 2"]);
    /// ```
    DropdownActions(&'a str, &'a [&'a str]),
    /// Display a text input field with the given placeholder
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::Response("Enter some text...");
    /// ```
    Response(&'a str),
}

/// Options to further customize the notification
pub struct NotificationOptions<'a> {
    pub(crate) main_button: Option<MainButton<'a>>,
    pub(crate) close_button: Option<&'a str>,
    pub(crate) app_icon: Option<&'a str>,
    pub(crate) content_image: Option<&'a str>,
    pub(crate) group_id: Option<&'a str>,
    pub(crate) delivery_date: Option<(f64, bool)>,
    pub(crate) sound: Option<&'a str>,
}

impl<'a> NotificationOptions<'a> {
    /// Create a NotificationOptions to further customize the notification
    pub fn new() -> NotificationOptions<'a> {
        NotificationOptions {
            main_button: None,
            close_button: None,
            app_icon: None,
            content_image: None,
            group_id: None,
            delivery_date: None,
            sound: None,
        }
    }

    /// Allow actions through a main button
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().main_button(MainButton::SingleAction("Main button"));
    /// ```
    pub fn main_button(&mut self, main_button: MainButton<'a>) -> &mut Self {
        self.main_button = Some(main_button);
        self
    }

    /// Display a close button with the given name
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().close_button("Close");
    /// ```
    pub fn close_button(&mut self, close_button: &'a str) -> &mut Self {
        self.close_button = Some(close_button);
        self
    }

    /// Display an icon on the left side of the notification
    ///
    /// NOTE: The icon of the app associated to the bundle will be displayed next to the notification title
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().app_icon("/path/to/icon.icns");
    /// ```
    pub fn app_icon(&mut self, app_icon: &'a str) -> &mut Self {
        self.app_icon = Some(app_icon);
        self
    }

    /// Display an image on the right side of the notification
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().content_image("/path/to/image.png");
    /// ```
    pub fn content_image(&mut self, content_image: &'a str) -> &mut Self {
        self.content_image = Some(content_image);
        self
    }

    /// Set an identifier to group multiple notifications together
    /// Notifications in group will dismiss each other so only the latest one is displayed
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().group_id("my_group_id");
    /// ```
    pub fn group_id(&mut self, group_id: &'a str) -> &mut Self {
        self.group_id = Some(group_id);
        self
    }

    /// Schedule the notification to be delivered at a later time
    ///
    /// The delivery_date parameter is the time at which to schedule the notification
    ///
    /// The synchronous parameter defines whether or not the notification
    /// should wait synchronously until it has been delivered (if no action is set)
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// # use chrono::offset::*;
    /// let stamp = Utc::now().timestamp() as f64 + 5.;
    ///
    /// // Synchronous is true, this will wait until the user
    /// // interacts with the notification before returning
    /// let _ = NotificationOptions::new().delivery_date(stamp, true);
    /// ```
    pub fn delivery_date(&mut self, delivery_date: f64, synchronous: bool) -> &mut Self {
        self.delivery_date = Some((delivery_date, synchronous));
        self
    }

    /// Play a system sound when the notification is delivered
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions::new().sound("Blow");
    /// ```
    pub fn sound(&mut self, sound: &'a str) -> &mut Self {
        self.sound = Some(sound);
        self
    }

    /// Convert the NotificationOptions into an Objective C NSDictionary
    pub(crate) fn to_dictionary(&self) -> Id<NSDictionary<NSString, NSString>> {
        // TODO: If possible, find a way to simplify this so I don't have to manually convert struct to NSDictionary
        let keys = &[
            &*NSString::from_str("mainButtonLabel"),
            &*NSString::from_str("actions"),
            &*NSString::from_str("closeButtonLabel"),
            &*NSString::from_str("appIcon"),
            &*NSString::from_str("contentImage"),
            &*NSString::from_str("groupID"),
            &*NSString::from_str("response"),
            &*NSString::from_str("deliveryDate"),
            &*NSString::from_str("synchronous"),
            &*NSString::from_str("sound"),
        ];
        let (main_button_label, actions, is_response): (&str, &[&str], bool) =
            match &self.main_button {
                Some(main_button) => match main_button {
                    MainButton::SingleAction(main_button_label) => (main_button_label, &[], false),
                    MainButton::DropdownActions(main_button_label, actions) => {
                        (main_button_label, actions, false)
                    }
                    MainButton::Response(response) => (response, &[], true),
                },
                None => ("", &[], false),
            };

        let vals = vec![
            NSString::from_str(main_button_label),
            // TODO: Find a way to support NSArray as a NSDictionary Value rather than JUST NSString so I don't have to convert array to string and back
            NSString::from_str(&actions.join(",")),
            NSString::from_str(self.close_button.unwrap_or("")),
            NSString::from_str(self.app_icon.unwrap_or("")),
            NSString::from_str(self.content_image.unwrap_or("")),
            NSString::from_str(self.group_id.unwrap_or_default()),
            // TODO: Same as above, if NSDictionary could support multiple types, this could be a boolean
            NSString::from_str(if is_response { "yes" } else { "" }),
            NSString::from_str(&match self.delivery_date {
                Some((delivery_date, _)) => delivery_date.to_string(),
                _ => String::new(),
            }),
            // TODO: Same as above, if NSDictionary could support multiple types, this could be a boolean
            NSString::from_str(match self.delivery_date {
                Some((_, true)) => "yes",
                _ => "",
            }),
            NSString::from_str(match self.sound {
                Some(sound) if check_sound(sound) => sound,
                _ => "_mute",
            }),
        ];
        NSDictionary::from_keys_and_objects(keys, vals)
    }
}

/// Response from the Notification
#[derive(Debug)]
pub enum NotificationResponse {
    /// No interaction has occured
    None,
    /// User clicked on an action button with the given name
    ActionButton(String),
    /// User clicked on the close button with the given name
    CloseButton(String),
    /// User clicked the notification directly
    Click,
    /// User submitted text to the input text field
    Reply(String),
}

impl NotificationResponse {
    /// Create a NotificationResponse from the given Objective C NSDictionary
    pub(crate) fn from_dictionary(dictionary: Id<NSDictionary<NSString, NSString>>) -> Self {
        let dictionary = dictionary.deref();

        let activation_type =
            match dictionary.object_for(NSString::from_str("activationType").deref()) {
                Some(str) => Some(str.deref().as_str().to_owned()),
                None => None,
            };

        match activation_type.as_deref() {
            Some("actionClicked") => NotificationResponse::ActionButton(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("closeClicked") => NotificationResponse::CloseButton(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("replied") => NotificationResponse::Reply(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("contentsClicked") => NotificationResponse::Click,
            _ => NotificationResponse::None,
        }
    }
}
