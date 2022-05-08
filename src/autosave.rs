use cursive::Cursive;
use scheduled_thread_pool::ScheduledThreadPool;
use crate::db;

pub fn start_autosave_loop(siv: &Cursive) {
    let cb_sink = siv.cb_sink().clone();
    let thread_pool = ScheduledThreadPool::new(1);
    thread_pool.execute_at_fixed_rate(
        core::time::Duration::from_secs(30),
        core::time::Duration::from_secs(30),
        move || { cb_sink.send(Box::new(db::save_to_db)).unwrap() }
    );
}