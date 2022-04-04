mod daemon;
mod go;
mod register;
mod status;
mod tasks;
mod triggers;
mod unregister;
mod update;
mod withdraw;

fn main() {
    daemon::daemon();
    go::go();
    register::register();
    status::status();
    tasks::tasks();
    triggers::triggers();
    unregister::unregister();
    update::update();
    withdraw::withdraw();
}