pub enum UserRegisteringCurrentlyEditing {
    Name,
    Email,
    Password,
}

pub enum UserLoggingInCurrentlyEditing {
    Email,
    Password,
}

pub enum UserModifyingCurrentlyEditing {
    Name,
    Email,
    Password,
}

pub enum EventCreatingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
    Invitees,
}

pub enum EventModifyingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
}

pub enum CurrentScreen {
    Main,
    ApiEditing,
    UserRegistering(UserRegisteringCurrentlyEditing),
    UserLoggingIn(UserLoggingInCurrentlyEditing),
    UserLoggingOut,
    UserDeleting,
    UserListing,
    UserFetching,
    UserModifying(UserModifyingCurrentlyEditing),
    EventDeleting,
    EventCreating(EventCreatingCurrentlyEditing),
    EventListing,
    EventModifying(EventModifyingCurrentlyEditing),
}

pub struct App<'a> {
    pub api_link: &'a str,
    pub config: Config,
    pub current_screen: CurrentScreen,
}
