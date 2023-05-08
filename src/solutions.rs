pub mod p1;
pub mod p100;
pub mod p101;
pub mod p102;
pub mod p1026;
pub mod p103;
pub mod p104;
pub mod p1046;
pub mod p1048;
pub mod p105;
pub mod p11;
pub mod p121;
pub mod p122;
pub mod p124;
pub mod p125;
pub mod p128;
pub mod p129;
pub mod p13;
pub mod p130;
pub mod p136;
pub mod p139;
pub mod p14;
pub mod p1413;
pub mod p143;
pub mod p1456;
pub mod p1491;
pub mod p15;
pub mod p152;
pub mod p153;
pub mod p1572;
pub mod p1657;
pub mod p168;
pub mod p17;
pub mod p1822;
pub mod p19;
pub mod p190;
pub mod p191;
pub mod p198;
pub mod p2;
pub mod p20;
pub mod p200;
pub mod p202;
pub mod p203;
pub mod p206;
pub mod p207;
pub mod p208;
pub mod p21;
pub mod p211;
pub mod p212;
pub mod p213;
pub mod p217;
pub mod p2215;
pub mod p226;
pub mod p23;
pub mod p230;
pub mod p2336;
pub mod p235;
pub mod p238;
pub mod p242;
pub mod p258;
pub mod p260;
pub mod p268;
pub mod p295;
pub mod p297;
pub mod p3;
pub mod p300;
pub mod p319;
pub mod p322;
pub mod p33;
pub mod p338;
pub mod p347;
pub mod p368;
pub mod p371;
pub mod p39;
pub mod p404;
pub mod p417;
pub mod p424;
pub mod p435;
pub mod p441;
pub mod p461;
pub mod p48;
pub mod p49;
pub mod p5;
pub mod p53;
pub mod p54;
pub mod p540;
pub mod p55;
pub mod p56;
pub mod p57;
pub mod p572;
pub mod p58;
pub mod p62;
pub mod p647;
pub mod p649;
pub mod p658;
pub mod p67;
pub mod p692;
pub mod p70;
pub mod p73;
pub mod p76;
pub mod p79;
pub mod p872;
pub mod p88;
pub mod p9;
pub mod p91;
pub mod p92;
pub mod p931;
pub mod p938;
pub mod p96;
pub mod p98;
pub mod p980;

pub fn run(n: &str) {
    match n {
        "1" => p1::run(),
        "2" => p2::run(),
        "3" => p3::run(),
        "5" => p5::run(),
        "9" => p9::run(),
        "11" => p11::run(),
        "13" => p13::run(),
        "14" => p14::run(),
        "15" => p15::run(),
        "17" => p17::run(),
        "19" => p19::run(),
        "20" => p20::run(),
        "21" => p21::run(),
        "23" => p23::run(),
        "33" => p33::run(),
        "39" => p39::run(),
        "48" => p48::run(),
        "49" => p49::run(),
        "53" => p53::run(),
        "54" => p54::run(),
        "55" => p55::run(),
        "56" => p56::run(),
        "57" => p57::run(),
        "58" => p58::run(),
        "62" => p62::run(),
        "67" => p67::run(),
        "70" => p70::run(),
        "73" => p73::run(),
        "76" => p76::run(),
        "79" => p79::run(),
        "88" => p88::run(),
        "91" => p91::run(),
        "92" => p92::run(),
        "96" => p96::run(),
        "98" => p98::run(),
        "100" => p100::run(),
        "101" => p101::run(),
        "102" => p102::run(),
        "103" => p103::run(),
        "104" => p104::run(),
        "105" => p105::run(),
        "121" => p121::run(),
        "122" => p122::run(),
        "124" => p124::run(),
        "125" => p125::run(),
        "128" => p128::run(),
        "129" => p129::run(),
        "130" => p130::run(),
        "136" => p136::run(),
        "139" => p139::run(),
        "143" => p143::run(),
        "152" => p152::run(),
        "153" => p153::run(),
        "168" => p168::run(),
        "190" => p190::run(),
        "191" => p191::run(),
        "198" => p198::run(),
        "200" => p200::run(),
        "202" => p202::run(),
        "203" => p203::run(),
        "206" => p206::run(),
        "207" => p207::run(),
        "208" => p208::run(),
        "211" => p211::run(),
        "212" => p212::run(),
        "213" => p213::run(),
        "217" => p217::run(),
        "226" => p226::run(),
        "230" => p230::run(),
        "235" => p235::run(),
        "238" => p238::run(),
        "242" => p242::run(),
        "258" => p258::run(),
        "260" => p260::run(),
        "268" => p268::run(),
        "295" => p295::run(),
        "297" => p297::run(),
        "300" => p300::run(),
        "319" => p319::run(),
        "322" => p322::run(),
        "338" => p338::run(),
        "347" => p347::run(),
        "368" => p368::run(),
        "371" => p371::run(),
        "404" => p404::run(),
        "417" => p417::run(),
        "424" => p424::run(),
        "435" => p435::run(),
        "441" => p441::run(),
        "461" => p461::run(),
        "540" => p540::run(),
        "572" => p572::run(),
        "647" => p647::run(),
        "649" => p649::run(),
        "658" => p658::run(),
        "692" => p692::run(),
        "872" => p872::run(),
        "931" => p931::run(),
        "938" => p938::run(),
        "980" => p980::run(),
        "1026" => p1026::run(),
        "1046" => p1046::run(),
        "1048" => p1048::run(),
        "1413" => p1413::run(),
        "1456" => p1456::run(),
        "1491" => p1491::run(),
        "1572" => p1572::run(),
        "1657" => p1657::run(),
        "1822" => p1822::run(),
        "2215" => p2215::run(),
        "2336" => p2336::run(),
        _ => println!("Solution not found."),
    }
}
