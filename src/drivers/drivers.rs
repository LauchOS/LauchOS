use crate::drivers::keyboard;

pub async fn start(){
    keyboard::start().await; // start keyboard driver
}