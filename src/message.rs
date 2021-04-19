use rand::distributions::{Standard, Uniform};
use rand::{thread_rng, Rng};

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    pub phone_number: String,
    pub content: String,
}

impl Message {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let phone_number = format!(
            "{}-{}-{}",
            (&mut rng)
                .sample_iter::<char, _>(Uniform::new_inclusive('0', '9'))
                .take(3)
                .collect::<String>(),
            (&mut rng)
                .sample_iter::<char, _>(Uniform::new_inclusive('0', '9'))
                .take(3)
                .collect::<String>(),
            (&mut rng)
                .sample_iter::<char, _>(Uniform::new_inclusive('0', '9'))
                .take(4)
                .collect::<String>(),
        );
        let content = (&mut rng)
            .sample_iter::<char, _>(Standard)
            .take(100)
            .collect();
        Message {
            phone_number,
            content,
        }
    }
}

#[test]
fn test_random_message() {
    let message1 = Message::random();
    let message2 = Message::random();
    assert_ne!(message1, message2);
    assert_eq!(message1.phone_number.len(), 12);
    assert_eq!(message1.content.chars().count(), 100);
}
