use phf::phf_map;
use vi::{
    processor::{LetterModification, ToneMark},
    Action, Definition,
};

// Custom vni method with ư short hand using the w character
const MY_VNI: Definition = phf_map! {
    '1' => &[Action::AddTonemark(ToneMark::Acute)],
    '2' => &[Action::AddTonemark(ToneMark::Grave)],
    '3' => &[Action::AddTonemark(ToneMark::HookAbove)],
    '4' => &[Action::AddTonemark(ToneMark::Tilde)],
    '5' => &[Action::AddTonemark(ToneMark::Underdot)],
    '6' => &[Action::ModifyLetter(LetterModification::Circumflex)],
    '7' => &[Action::ModifyLetter(LetterModification::Horn)],
    '8' => &[Action::ModifyLetter(LetterModification::Breve)],
    '9' => &[Action::ModifyLetter(LetterModification::Dyet)],
    'z' => &[Action::ResetInsertedƯ, Action::InsertƯ],
    '0' => &[Action::RemoveToneMark],
};

fn main() {
    let inputs = "Xin hay4 mo73 toang het61 nhzng4 canh1 cza3 cua3 qua1 khz1 de963 thuyen62 toi6 nzong gio1 lang4 quen6 ra khoi7";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        vi::transform_buffer(&MY_VNI, word.chars(), &mut result);
        result.push(' ');
    }

    println!("{result}"); // prints "Xin hãy mở toang hết những cánh cửa của quá khứ để thuyền tôi nương gió lãng quên ra khơi"
}
