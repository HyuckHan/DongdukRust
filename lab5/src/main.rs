mod reader;
mod email_reader;

use crate::email_reader::EmailReader;
use crate::reader::Reader;
use crate::reader::Summary;

fn main() {
}

fn test_basic_and_empty_emails() {
    let basic_email = "Subject: Party Time
From: Jake
To: David
Just finished my work. Time for a break!";
    let empty_email = "";

    let basic_reader = EmailReader::parse(basic_email.to_string()).unwrap();
    assert_eq!(basic_reader.msg_len(), 40);
    assert_eq!(
        basic_reader.summarize(),
        "Jake: Just finished my work. Time for a break!"
    );
    assert_eq!(basic_reader.get_info(), "Party Time\nFrom: Jake, To: David");

    let empty_result = EmailReader::parse(empty_email.to_string());
    assert!(empty_result.is_err());
    assert_eq!(
        empty_result.as_ref().unwrap_err().kind(),
        std::io::ErrorKind::InvalidData
    );
}

fn test_malformed_emails() {
    let missing_fields = "Subject: Test Email
To: Recipient";
    let empty_subject = "Subject: \nFrom: sender\nTo: recipient\n";
    let empty_sender = "Subject: Test\nFrom: \nTo: recipient\n";
    let empty_recipient = "Subject: Test\nFrom: sender\nTo: ";

    for email in [missing_fields, empty_subject, empty_sender, empty_recipient].iter() {
        let result = EmailReader::parse(email.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
        assert_eq!(
            result.as_ref().unwrap_err().to_string(),
            "File is not in the correct format"
        );
    }
}

fn generate_long_email() -> String {
    let subject = "Long Message Subject";
    let sender = "LongSenderName@example.com";
    let recipient = "LongRecipientName@example.org";

    // Generate a very long message, exceeding typical lengths
    let mut message = String::new();
    for _ in 0..1000 {
        message.push_str("This is a sentence in the very long message. ");
    }

    let email_str = format!(
        "Subject: {}\nFrom: {}\nTo: {}\n{}",
        subject, sender, recipient, message
    );

    email_str
}

fn test_summaries() {
    let short_email = "Subject: Meeting Reschedule
From: Executive Team
To: Department Heads
Please note the change in meeting schedule...";
    let long_email = generate_long_email();

    let short_reader = EmailReader::parse(short_email.to_string()).unwrap();
    let summary = short_reader.summarize();
    assert!(summary.starts_with("Executive Team: "));
    assert!(short_reader.get_info().contains("Meeting Reschedule"));

    let long_reader = EmailReader::parse(long_email).unwrap();
    let summary = long_reader.summarize();
    assert_eq!(summary.len(), 280);
    assert!(summary.contains("LongSenderName@example.com"));
    assert!(summary.contains("This is a sentence in the very long message. "));
}

fn test_summarize_extremely_long_message() {
    let subject = "Long Message Subject";
    let sender = "LongSenderName@example.com";
    let recipient = "LongRecipientName@example.org";

    // Generate a very long message, exceeding typical lengths
    let mut message = String::new();
    for _ in 0..1000 {
        message.push_str("This is a sentence in the very long message. ");
    }

    let email_str = format!(
        "Subject: {}\nFrom: {}\nTo: {}\n{}",
        subject, sender, recipient, message
    );

    let reader = EmailReader::parse(email_str.to_string()).unwrap();
    let summary = reader.summarize();
    assert_eq!(summary.len(), 280);

    assert!(summary.contains(sender));
    assert!(summary.contains("This is a sentence in the very long message. "));
}
