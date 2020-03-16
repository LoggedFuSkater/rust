
use super::*;

struct TestData {
    input: &'static str,
    padding: u8,
    expected_result: &'static str
}

#[test]
fn test() {
    for test in TEST_CASES {
        let result = obfp(test.input, test.padding);

        assert_eq!(&result, test.expected_result, "input: {},\npadding: {}",test.input, test.padding)
    }
}
const TEST_CASES: &[TestData] = &[
    // Empty input
    TestData {
        input:          "",
        padding:        0,
        expected_result: "ExtremelyTipsyBunny",
    },
    // Test padding, positive and negative cases. Also, same input -> same output regardless of padding size.
    TestData {
        input:          "asdf",
        padding:        0,
        expected_result: "QuickRootedSwing",
    },
    TestData {
        input:          "asdf",
        padding:        2,
        expected_result: "QuickRootedSwing5012",
    },
    TestData {
        input:          "asdf",
        padding:        4,
        expected_result: "QuickRootedSwing5012F6C6",
    },
    TestData {
        input:          "asdf",
        padding:        8,
        expected_result: "QuickRootedSwing5012F6C60B27661C",
    },
    TestData {
        input:          "asdf",
        padding:        200,
        expected_result: "QuickRootedSwing5012F6C60B27661C",
    },
    // Test a few unique UUID:s
    TestData {
        input:          "ac968750-7ca2-4dde-908b-aacbbed2f470",
        padding:        1,
        expected_result: "SomewhatCleanPearF4",
    },
    TestData {
        input:          "3e3278cd-6030-400d-9c0d-ef9be0119853",
        padding:        5,
        expected_result: "QuickTipsyBobA2DEC84AEE",
    },
    TestData {
        input:          "6745dc33-2fbd-4311-8884-10aab93199a9",
        padding:        7,
        expected_result: "NextSluggishSwing7F2343BF6927EA",
    },
    // Big data blob
    TestData {
        input: "mc093284750932nv2ono2hvfnoh3fo9ch3fxh23omhf293u4hfcqoiuwnhfc093u4hfc2938hnfc209u3hfc092hu3fc092nu3hfc92u3h4fc92nu3h4nfc923h40fc92h340fu2h34fc9u2nh3409uh2304hufc2093u4hfc0\nfcn9n2j43fc 9hu23cfj32fc2\nfc234ufh2o3ihfoh4f92c3hnfc928h43c92mj3fc23\ncfhfcliuw hfroiwuehgoiwuregoiwuecpowi hcpoqiwjecpoiqwhecp9824r+9u3h4f9283 h4f8w73hfwo83fou3wh4fcpoqihfp2u3h4fc983h4fcpu3nh4fcpoh3pf2h34pfc8h3p48hcqp348hfcqp384hfcpq834nfcpq9834hfcpq3h4fc",
        padding:        0,
        expected_result: "SometimesCleanGame",
    },
];
