pub struct InvalidUserErr(String);

impl InvalidUserErr {
    pub fn new(message: Option<String>) -> InvalidUserErr
    {
        match message {
            Some(msg) => InvalidUserErr(msg),
            None => InvalidUserErr("Err: User is invalid".into()),
        }
    }
}
