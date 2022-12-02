mod dobbelsteen;

use crate::dobbelsteen::dobbelsteen::Dobbelsteen;


fn main() {
    let mut dobbelsteen = Dobbelsteen::new(6, None);
    dobbelsteen.set_zijden(20);
    println!("dobbelsteen van soort: {} land op: {}", dobbelsteen.get_soort(), dobbelsteen.gooi())
}
