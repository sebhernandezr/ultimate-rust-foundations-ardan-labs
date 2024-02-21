use std::io::stdin;

#[derive(Debug, PartialEq, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    let users = get_users();

    // match users.iter().find(|user| user.username == username) {
    //     Some(user) => {
    //         if user.password == password {
    //             Some(LoginAction::Granted(user.role.clone()))
    //         } else {
    //             Some(LoginAction::Denied)
    //         }
    //     }
    //     None => None,
    // }

    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("Lorem"), "Hello Lorem");
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
        assert_eq!(login("not-admin", "not-password"), None);
    }
}
