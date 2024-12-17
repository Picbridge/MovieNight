use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub is_logged_in: bool,
    pub user_id: Option<String>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            is_logged_in: false,
            user_id: None,
        }
    }
}

// Make the type alias public
pub type SharedAuthContext = Rc<RefCell<AuthContext>>;
