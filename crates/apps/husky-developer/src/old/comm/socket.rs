use std::{
    io::{self, BufReader},
    net::TcpStream,
    thread,
};

use crossbeam_channel::{bounded, Receiver, Sender};

use super::{
    io_threads::IoThreads,
    json::{ReadFromJSON, WriteToJSON},
    ClientMessage, DeveloperResponse,
};

pub(crate) fn socket_transport(
    stream: TcpStream,
) -> (
    Sender<DeveloperResponse>,
    Receiver<ClientMessage>,
    IoThreads,
) {
    let (reader_receiver, reader) = make_reader(stream.try_clone().unwrap());
    let (writer_sender, writer) = make_write(stream.try_clone().unwrap());
    let io_threads = IoThreads::new(reader, writer);
    (writer_sender, reader_receiver, io_threads)
}

fn make_reader(stream: TcpStream) -> (Receiver<ClientMessage>, thread::JoinHandle<io::Result<()>>) {
    let (reader_sender, reader_receiver) = bounded::<ClientMessage>(0);
    let reader = thread::spawn(move || {
        let mut buf_read = BufReader::new(stream);
        while let Some(msg) = ClientMessage::read(&mut buf_read).unwrap() {
            let is_exit = match &msg {
                ClientMessage::Notification(n) => n.is_exit(),
                _ => false,
            };
            reader_sender.send(msg).unwrap();
            if is_exit {
                break;
            }
        }
        Ok(())
    });
    (reader_receiver, reader)
}

fn make_write(
    mut stream: TcpStream,
) -> (
    Sender<DeveloperResponse>,
    thread::JoinHandle<io::Result<()>>,
) {
    let (writer_sender, writer_receiver) = bounded::<DeveloperResponse>(0);
    let writer = thread::spawn(move || {
        writer_receiver
            .into_iter()
            .try_for_each(|it| it.write(&mut stream))
            .unwrap();
        Ok(())
    });
    (writer_sender, writer)
}
