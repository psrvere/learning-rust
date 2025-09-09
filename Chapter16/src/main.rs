mod thread;
mod message_passing;
mod shared_state;

fn main() {
    thread::examples();
    message_passing::examples();
    shared_state::examples();
}
