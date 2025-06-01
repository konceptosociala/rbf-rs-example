use relm_derive::Msg;

#[derive(Msg)]
pub enum TodoAppMsg {
    SetCurrentScreen(usize),
    Quit,
}