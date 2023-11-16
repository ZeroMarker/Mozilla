mod guess;
mod user;
mod basic;
mod owner;
mod slice;
mod rectangle;
mod ip;
mod message;


fn main() {
    basic::basic();
    // guess::guess();

    owner::owner();

    slice::slice();

    user::create_user();

    rectangle::rectangle();
}





