use cursive::Cursive;
use crate::GlobalContext;

pub fn fetch(s: &mut Cursive) -> &mut GlobalContext {
    s.user_data::<GlobalContext>().expect("Global context should be defined")
}