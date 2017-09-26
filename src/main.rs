use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::result::Result;
use std::fmt::Debug;

pub type TError = String;
pub type TResult<T> = Result<T, TError>;

#[derive(Debug, PartialEq)]
pub struct Type1 {
    payload: u32,
}

#[derive(Debug, PartialEq)]
pub struct Type2 {
    payload: f64,
}

#[derive(Debug, PartialEq)]
pub struct Type3 {
    payload: bool,
}

#[derive(Debug, PartialEq)]
pub enum Message<T> {
    Quit,
    Skip(TError),
    Work(TResult<T>),
}

fn try_send<T>(sender: &SyncSender<Message<T>>, msg: Message<T>) {
    sender.send(msg).expect("The channel is broken\n");
}

fn try_receive<T>(receiver: &Receiver<Message<T>>) -> Message<T> {
    receiver.recv().expect("The channel is broken\n")
}

fn worker<I, O, F>(receiver: Receiver<Message<I>>, sender: SyncSender<Message<O>>, mut process: F)
where
    F: FnMut(I) -> TResult<O>,
    O: Debug,
{
    loop {
        let result: Message<O>;
        match try_receive(&receiver) {
            Message::Quit => {
                break;
            }
            Message::Skip(err) => {
                result = Message::Skip(err);
            }
            Message::Work(msg) => {
                match msg {
                    Ok(data) => result = Message::Work(process(data)),
                    Err(err) => result = Message::Skip(err),
                }
            }
        };
        //        println!("{:#?}", &result);
        try_send(&sender, result);
    }
    println!("Message::Quit");
    try_send(&sender, Message::Quit);
}

pub fn worker1(arg: Type1) -> TResult<Type2> {
    if arg.payload < 5 {
        Ok(Type2 { payload: 2.0 * arg.payload as f64 })
    } else {
        Err(format!("Payload {} more than 5", arg.payload))
    }
}

pub fn worker2(arg: Type2) -> TResult<Type3> {
    if arg.payload < 10.0 {
        Ok(Type3 { payload: true })
    } else {
        Err(format!("Payload {} more than 10", arg.payload))
    }
}




fn main() {
    println!("Begin");
    let (sender1, receiver1) = sync_channel(100);
    let (sender2, receiver2) = sync_channel(100);
    let (sender3, receiver3) = sync_channel(100);

    thread::spawn(move || { worker(receiver1, sender2, worker1); });
    thread::spawn(move || { worker(receiver2, sender3, worker2); });

    for i in vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9] {
        let data = Type1 { payload: i };
        try_send(&sender1, Message::Work(Ok(data)));
        println!("Result: {:?}", try_receive(&receiver3));
    }



    // let mut zip = archive::open(archive_name)?;
    // for i in 0..zip.len() {
    //     let mut file = zip.by_index(i)?;
    //     let header = archive::load_header(&mut file);
    //     sender1.send(header).expect("The channel is broken\n");
    //     let msg = receiver3.recv().expect("The channel is broken\n");
    //     match msg {
    //         Ok(fb) => println!("{}", tools::fmt_book(&fb)),
    //         Err(Fb2Error::Done) => break,
    //         Err(err) => println!("!!! {} -> {}", file.name(), err.description()),
    //     }
    // }

    try_send(&sender1, Message::Quit);
    assert!(try_receive(&receiver3) == Message::Quit);

    println!("Done");
}


/*
use pipe;
use tools;
use archive;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;

use tools::as_utf8;
use tools::create_fb2;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::thread;


fn is_done<T>(msg: &Result<T, Fb2Error>) -> bool {
    match *msg {
        Err(Fb2Error::Done) => true,
        _ => false,
    }
}

fn worker<I, O, F>(
    receiver: Receiver<Result<I, Fb2Error>>,
    sender: SyncSender<Result<O, Fb2Error>>,
    mut processor: F,
) where
    F: FnMut(Result<I, Fb2Error>) -> Result<O, Fb2Error>,
{
    let mut have_tasks = true;
    while have_tasks {
        let input = receiver.recv().expect("The channel is broken\n");
        have_tasks = !is_done(&input);
        let output = processor(input);
        sender.send(output).expect("The channel is broken\n");
    }
}

pub fn do_parse(archive_name: &str) -> Fb2Result<()> {
    let (sender1, receiver1) = sync_channel(100);
    let (sender2, receiver2) = sync_channel(100);
    let (sender3, receiver3) = sync_channel(100);

    thread::spawn(move || { worker(receiver1, sender2, pipe::converter); });
    thread::spawn(move || { worker(receiver2, sender3, pipe::maker); });

    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let header = archive::load_header(&mut file);
        sender1.send(header).expect("The channel is broken\n");
        let msg = receiver3.recv().expect("The channel is broken\n");
        match msg {
            Ok(fb) => println!("{}", tools::fmt_book(&fb)),
            Err(Fb2Error::Done) => break,
            Err(err) => println!("!!! {} -> {}", file.name(), err.description()),
        }
    }
    sender1.send(Err(Fb2Error::Done)).expect(
        "The channel is broken\n",
    );
    Ok(())
}

*/