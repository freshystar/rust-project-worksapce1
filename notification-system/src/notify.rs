pub trait Notifier {
     fn send_notification(&self);
}

pub struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn send_notification(&self) {
        let mut notify = String::new();
        println!("You received a notification from Jady: {}", notify);
    }
}

pub struct SMSNotifier;

impl Notifier for SMSNotifier {
    fn send_notification(&self) {
        let mut text = String::new();
        println!("You received a message from Anne: {}", text);
    }
}

pub struct NotificationService<S: Notifier> {
    Notifier: S,
}

impl <S: Notifier> NotificationService<S> {
    fn run(&self) {
        self.Notifier.send_notification();
    }
}
