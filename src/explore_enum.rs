pub fn call() {
    enum Message {
        Write(String),
        Send(Option<bool>),
        Read(bool),
    }

    fn dispatch(action: Message) {
        match action {
            Message::Write(msg) => println!("Message writed {}", msg),
            Message::Read(already_read) => println!("Already read? {}", already_read),
            Message::Send(sent) => println!("{:#?}", sent),
        }
    }

    println!("Message sent");

    dispatch(Message::Write(String::from("Hello World")));
    dispatch(Message::Send(Some(true)));
    dispatch(Message::Read(true));

    println!("Message will have sent");
    dispatch(Message::Write(String::from("Hello World")));
    dispatch(Message::Send(None));
    dispatch(Message::Read(false));
}
