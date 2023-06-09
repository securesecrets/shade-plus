//! https://simple.wikipedia.org/wiki/Order_of_magnitude.
use ethnum::U256;

/// Gets the result of 10^x in constant time. Used for decimal precision calculations (i.e. normalizing different token amounts
/// based off their token decimals, etc). In most cases, x would be between 0 and 18, but we allow for up to 32 in case something special comes up.
///
/// @param x - integer between 0 and 77, inclusive (10^78 overflows U256)
///
/// @return result 10^x as U256
pub const fn exp10(x: u8) -> U256 {
    match x {
        0 => QUINTILLIONTH,
        1 => HUN_QUADRILLIONTH,
        2 => TEN_QUADRILLIONTH,
        3 => QUADRILLIONTH,
        4 => HUN_TRILLIONTH,
        5 => TEN_TRILLIONTH,
        6 => TRILLIONTH,
        7 => HUN_BILLIONTH,
        8 => TEN_BILLIONTH,
        9 => BILLIONTH,
        10 => HUN_MILLIONTH,
        11 => TEN_MILLIONTH,
        12 => MILLIONTH,
        13 => HUN_THOUSANDTH,
        14 => TEN_THOUSANDTH,
        15 => THOUSANDTH,
        16 => HUNDREDTH,
        17 => TENTH,
        18 => ONE,
        19 => TEN,
        20 => HUNDRED,
        21 => THOUSAND,
        22 => TEN_THOUSAND,
        23 => HUN_THOUSAND,
        24 => MILLION,
        25 => TEN_MILLION,
        26 => HUN_MILLION,
        27 => BILLION,
        28 => TEN_BILLION,
        29 => HUN_BILLION,
        30 => TRILLION,
        31 => TEN_TRILLION,
        32 => HUN_TRILLION,
        33 => QUADRILLION,
        34 => TEN_QUADRILLION,
        35 => HUN_QUADRILLION,
        36 => QUINTILLION,
        37 => TEN_QUINTILLION,
        38 => HUN_QUINTILLION,
        39 => SEXTILLION,
        40 => TEN_SEXTILLION,
        41 => HUN_SEXTILLION,
        42 => SEPTILLION,
        43 => TEN_SEPTILLION,
        44 => HUN_SEPTILLION,
        45 => OCTILLION,
        46 => TEN_OCTILLION,
        47 => HUN_OCTILLION,
        48 => NONILLION,
        49 => TEN_NONILLION,
        50 => HUN_NONILLION,
        51 => DECILLION,
        52 => TEN_DECILLION,
        53 => HUN_DECILLION,
        54 => UNDECILLION,
        55 => TEN_UNDECILLION,
        56 => HUN_UNDECILLION,
        57 => DUODECILLION,
        58 => TEN_DUODECILLION,
        59 => HUN_DUODECILLION,
        60 => TREDECILLION,
        61 => TEN_TREDECILLION,
        62 => HUN_TREDECILLION,
        63 => QUATTUORDECILLION,
        64 => TEN_QUATTUORDECILLION,
        65 => HUN_QUATTUORDECILLION,
        66 => QUINDECILLION,
        67 => TEN_QUINDECILLION,
        68 => HUN_QUINDECILLION,
        69 => SEXDECILLION,
        70 => TEN_SEXDECILLION,
        71 => HUN_SEXDECILLION,
        72 => SEPTENDECILLION,
        73 => TEN_SEPTENDECILLION,
        74 => HUN_SEPTENDECILLION,
        75 => OCTODECILLION,
        76 => TEN_OCTODECILLION,
        77 => HUN_OCTODECILLION,
        _ => panic!("Out of range."),
    }
}

pub const QUINTILLIONTH: U256 = U256::new(1u128);
pub const HUN_QUADRILLIONTH: U256 = U256::new(10u128);
pub const TEN_QUADRILLIONTH: U256 = U256::new(100u128);
pub const QUADRILLIONTH: U256 = U256::new(1000);
pub const HUN_TRILLIONTH: U256 = U256::new(10000);
pub const TEN_TRILLIONTH: U256 = U256::new(100000);
pub const TRILLIONTH: U256 = U256::new(1000000);
pub const HUN_BILLIONTH: U256 = U256::new(10000000);
pub const TEN_BILLIONTH: U256 = U256::new(100000000);
pub const BILLIONTH: U256 = U256::new(1000000000);
pub const HUN_MILLIONTH: U256 = U256::new(10000000000);
pub const TEN_MILLIONTH: U256 = U256::new(100000000000);
pub const MILLIONTH: U256 = U256::new(1000000000000);
pub const HUN_THOUSANDTH: U256 = U256::new(10000000000000);
pub const TEN_THOUSANDTH: U256 = U256::new(100000000000000);
pub const THOUSANDTH: U256 = U256::new(1000000000000000);
pub const HUNDREDTH: U256 = U256::new(10000000000000000);
pub const TENTH: U256 = U256::new(100_000_000_000_000_000);
pub const ONE: U256 = U256::new(1_000_000_000_000_000_000);
pub const TEN: U256 = U256::new(10000000000000000000);
pub const HUNDRED: U256 = U256::new(100000000000000000000);
pub const THOUSAND: U256 = U256::new(1000000000000000000000);
pub const TEN_THOUSAND: U256 = U256::new(10000000000000000000000);
pub const HUN_THOUSAND: U256 = U256::new(100000000000000000000000);
pub const MILLION: U256 = U256::new(1000000000000000000000000);
pub const TEN_MILLION: U256 = U256::new(10000000000000000000000000);
pub const HUN_MILLION: U256 = U256::new(100000000000000000000000000);
pub const BILLION: U256 = U256::new(1000000000000000000000000000);
pub const TEN_BILLION: U256 = U256::new(10000000000000000000000000000);
pub const HUN_BILLION: U256 = U256::new(100000000000000000000000000000);
pub const TRILLION: U256 = U256::new(1000000000000000000000000000000);
pub const TEN_TRILLION: U256 = U256::new(10000000000000000000000000000000);
pub const HUN_TRILLION: U256 = U256::new(100000000000000000000000000000000);
pub const QUADRILLION: U256 = U256::new(1000000000000000000000000000000000);
pub const TEN_QUADRILLION: U256 = U256::new(10000000000000000000000000000000000);
pub const HUN_QUADRILLION: U256 = U256::new(100000000000000000000000000000000000);
pub const QUINTILLION: U256 = U256::new(1000000000000000000000000000000000000);
pub const TEN_QUINTILLION: U256 = U256::new(10000000000000000000000000000000000000);
pub const HUN_QUINTILLION: U256 = U256::new(100000000000000000000000000000000000000);
pub const SEXTILLION: U256 = U256::from_words(2, 319435266158123073073250785136463577088);
pub const TEN_SEXTILLION: U256 = U256::from_words(29, 131811359292784559562136384478721867776);
pub const HUN_SEXTILLION: U256 = U256::from_words(293, 297266492165030205231240022491914043392);
pub const SEPTILLION: U256 = U256::from_words(2938, 250405986282794344605403365464994742272);
pub const TEN_SEPTILLION: U256 = U256::from_words(29387, 122083294381374201810411402627569942528);
pub const HUN_SEPTILLION: U256 = U256::from_words(293873, 199985843050926627713990203980394790912);
pub const OCTILLION: U256 = U256::from_words(2938735, 298446595904573959823029002645106851840);
pub const TEN_OCTILLION: U256 = U256::from_words(29387358, 262207023678231890523293166996922826752);
pub const HUN_OCTILLION: U256 =
    U256::from_words(293873587, 240093668335749660989309417946850787328);
pub const NONILLION: U256 = U256::from_words(2938735877, 18960114910927365649471927446130393088);
pub const TEN_NONILLION: U256 =
    U256::from_words(29387358770, 189601149109273656494719274461303930880);
pub const HUN_NONILLION: U256 =
    U256::from_words(293873587705, 194599656488044247630319707454198251520);
pub const DECILLION: U256 =
    U256::from_words(2938735877055, 244584730275750158986324037383141457920);
pub const TEN_DECILLION: U256 =
    U256::from_words(29387358770557, 63870734310932345619618121809037099008);
pub const HUN_DECILLION: U256 =
    U256::from_words(293873587705571, 298424976188384992732806610658602778624);
pub const UNDECILLION: U256 =
    U256::from_words(2938735877055718, 261990826516342219621069247131882094592);
pub const TEN_UNDECILLION: U256 =
    U256::from_words(29387358770557187, 237931696716852951967070219296443465728);
pub const HUN_UNDECILLION: U256 =
    U256::from_words(293873587705571876, 337622765642898738890454548373825388544);
pub const DUODECILLION: U256 =
    U256::from_words(2938735877055718769, 313686354140541217734174016852339982336);
pub const TEN_DUODECILLION: U256 =
    U256::from_words(29387358770557187699, 74322239116966006171368701637485920256);
pub const HUN_DUODECILLION: U256 = U256::from_words(
    293873587705571876992,
    62657657327783134786937801511322779648,
);
pub const TREDECILLION: U256 = U256::from_words(
    2938735877055718769921,
    286294206356892884406003407681459585024,
);
pub const TEN_TREDECILLION: U256 = U256::from_words(
    29387358770557187699218,
    140683128201421136353037217360450158592,
);
pub const HUN_TREDECILLION: U256 = U256::from_words(
    293873587705571876992184,
    45701814330457509676873743877428740096,
);
pub const QUATTUORDECILLION: U256 = U256::from_words(
    2938735877055718769921841,
    116735776383636633305362831342519189504,
);
pub const TEN_QUATTUORDECILLION: U256 = U256::from_words(
    29387358770557187699218413,
    146510663073550942663504491129887260672,
);
pub const HUN_QUATTUORDECILLION: U256 = U256::from_words(
    293873587705571876992184134,
    103977163051755572781546481571799760896,
);
pub const QUINDECILLION: U256 = U256::from_words(
    2938735877055718769921841343,
    18924529754740337425340993422692974592,
);
pub const TEN_QUINDECILLION: U256 = U256::from_words(
    29387358770557187699218413430,
    189245297547403374253409934226929745920,
);
pub const HUN_QUINDECILLION: U256 = U256::from_words(
    293873587705571876992184134305,
    191041140869341425217226305110456401920,
);
pub const SEXDECILLION: U256 = U256::from_words(
    2938735877055718769921841343055,
    208999574088721934855390013945722961920,
);
pub const TEN_SEXDECILLION: U256 = U256::from_words(
    29387358770557187699218413430556,
    48301539361588567773652494866620350464,
);
pub const HUN_SEXDECILLION: U256 = U256::from_words(
    293873587705571876992184134305561,
    142733026694947214273150341234435293184,
);
pub const SEPTENDECILLION: U256 = U256::from_words(
    2938735877055718769921841343055614,
    66200799265718288878004982617280086016,
);
pub const TEN_SEPTENDECILLION: U256 = U256::from_words(
    29387358770557187699218413430556141,
    321725625736244425316675218741032648704,
);
pub const HUN_SEPTENDECILLION: U256 = U256::from_words(
    293873587705571876992184134305561419,
    154714955073998081996380720524412583936,
);
pub const OCTODECILLION: U256 = U256::from_words(
    2938735877055718769921841343055614194,
    186020083056226966110308775517052993536,
);
pub const TEN_OCTODECILLION: U256 = U256::from_words(
    29387358770557187699218413430556141945,
    158788995957577343786214718011688878080,
);
pub const HUN_OCTODECILLION: U256 = U256::from_words(
    0xDD15FE86AFFAD91249EF0EB713F39EBE,
    0xAA987B6E6FD2A0000000000000000000,
);

#[cfg(test)]
mod test {
    use super::*;
    use ethnum::U256;

    #[test]
    fn test_const() {
        let mut expected_string = "1".to_string();
        for i in 0..78_u8 {
            let result = std::panic::catch_unwind(|| exp10(i));
            if i == 78 {
                assert!(result.is_err());
            } else {
                assert!(result.is_ok());
                assert_eq!(
                    U256::from_str_prefixed(&expected_string).unwrap(),
                    result.unwrap()
                );
            }
            expected_string.push('0');
        }
    }
}
