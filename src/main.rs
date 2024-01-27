mod fix_parser;
mod message_struct;
mod message_handler;

// use std::io::{stdin};

use fix_parser::parse;

fn main() {

    let msg1:String = String::from("8=FIX.4.2|35=R|34=1|49=SenderCompID|56=TargetCompID|11=QuoteReqID123|55=AAPL|54=1|38=100|40=2|");
    let msg2:String = String::from("8=FIX.4.2|35=AK|34=4|49=SenderCompID|56=TargetCompID|11=OrderID456|17=ExecID789|150=F|55=MSFT|32=50|31=121.25");
    let msg3:String = String::from("8=FIX.4.2|35=8|34=3|49=SenderCompID|56=TargetCompID|17=ExecID789|150=F|39=2|55=AAPL|32=25|31=120.75|");
    let msg4:String = String::from("8=FIX.4.2|35=D|34=2|49=SenderCompID|56=TargetCompID|11=OrderID456|55=MSFT|54=1|38=50|44=120.50|");

    print!("msg");

    parse(String::from(msg1));
    parse(String::from(msg2));
    parse(String::from(msg3));
    parse(String::from(msg4));

}
