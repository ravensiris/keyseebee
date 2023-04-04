use keyberon::action::{k, l, m, Action::*, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

const CUT: Action = m(&[LShift, Delete]);
const COPY: Action = m(&[LCtrl, Insert]);
const PASTE: Action = m(&[LShift, Insert]);
const L3_ENTER: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: &l(3),
    tap: &k(Enter),
};
const L1_SP: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &l(1),
    tap: &k(Space),
};
const CSPACE: Action = m(&[LCtrl, Space]);

const SHIFT_ESC: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &k(LShift),
    tap: &k(Escape),
};
const CTRL_INS: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &k(LCtrl),
    tap: &k(Insert),
};

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}
macro_rules! a {
    ($k:ident) => {
        m(&[RAlt, $k])
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [Escape Q W F P G J L U Y ; - ],
        [LShift A R S T D H N E I O Quote ],
        [ LCtrl Z X C V B K M , . / Tab ],
        [ t t LAlt LGui (1) Space Enter (2) BSpace RAlt t t ],
    }

    {
        [t 1 2 3 4 5 6 7 8 9 0 t],
        [t t t t t t t Left Down Up Right t],
        [t t t t t t t t t t t t],
        [t t t t t t t t t t t t],
    }

    {
        [t t t t t t t t t t t t],
        [t t t t t t  ^ '[' ']' '(' ')' t],
        [t t t t t t  t '_'  =  '{' '}' |],
        [t t t t t t t t t t t t],
    }

    {
        [t t t t t t t t t t t t],
        [t t t t t t t t t t t t],
        [t t t t t t t t t t t t],
        [t t t t t t t t t t t t],
    }
};
