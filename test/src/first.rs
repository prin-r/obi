use bs58;
use hex;
use obi::{OBIDecode, OBIEncode};

#[derive(OBIDecode, OBIEncode)]
pub struct VerifyOracleDataResult {
    oracle_script_id: u64,
    request_time: u64,
    aggregation_time: u64,
    requested_validators_count: u64,
    sufficient_validator_count: u64,
    reported_validators_count: u64,
    params: Vec<u8>,
    data: Vec<u8>,
}

impl VerifyOracleDataResult {
    pub fn get_mock() -> Self {
        VerifyOracleDataResult {
            oracle_script_id: 1,
            request_time: 2,
            aggregation_time: 3,
            requested_validators_count: 4,
            sufficient_validator_count: 5,
            reported_validators_count: 6,
            params: vec![1, 2, 3, 4, 5, 6, 7, 8],
            data: vec![8, 7, 6, 5, 4, 3, 2, 1],
        }
    }
}

#[derive(OBIDecode, OBIEncode)]
pub struct Proof {
    signature: Vec<u8>,
    result: VerifyOracleDataResult,
}

impl Proof {
    pub fn get_mock() -> Self {
        Proof {
            signature: hex::decode("cf1a362c690312b7866833054654e302922d098ac3a4939daed46cd476d00a4104534c9a7eb428a1469c80fcf73f3602dcb0305160cbb1cd2f126ac2340955ff1b").unwrap(),
            result: VerifyOracleDataResult {
                oracle_script_id: 1,
                request_time: 2,
                aggregation_time: 3,
                requested_validators_count: 4,
                sufficient_validator_count: 5,
                reported_validators_count: 6,
                params: vec![1, 2, 3, 4, 5, 6, 7, 8],
                data: vec![166 ,66, 0, 0, 0, 0, 0, 0],
            },
        }
    }
}

#[derive(OBIDecode, OBIEncode)]
pub struct TestResult {
    symbol: String,
    multiplier: u64,
    what: u8,
}

impl TestResult {
    pub fn get_mock() -> Self {
        TestResult {
            symbol: String::from("GOOG"),
            multiplier: 1000000,
            what: 120,
        }
    }
}

#[derive(OBIDecode, OBIEncode, Clone, Debug, PartialEq)]
pub struct Price {
    px: u64,
}
impl Price {
    pub fn get_mock(x: u64) -> Self {
        Price { px: x }
    }
}

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct Req {
    pub client_id: String,
    pub oracle_script_id: u64,
    pub calldata: Vec<u8>,
    pub ans_count: u64,
    pub min_count: u64,
}

impl Req {
    pub fn get_mock() -> Self {
        Req {
            client_id: String::from("meichain"),
            oracle_script_id: 13,
            calldata: vec![5, 0, 0, 0, 94, 71, 83, 80, 67, 47, 161, 20, 33, 0, 0, 0, 0],
            ans_count: 7,
            min_count: 8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct Res {
    pub client_id: String,
    pub request_id: u64,
    pub ans_count: u64,
    pub request_time: u64,
    pub resolve_time: u64,
    pub resolve_status: u8,
    pub result: Vec<u8>,
}

impl Res {
    pub fn get_mock() -> Self {
        Res {
            client_id: String::from("mumu_chain"),
            request_id: 119,
            ans_count: 4,
            request_time: 1234567,
            resolve_time: 7654321,
            resolve_status: 3,
            result: vec![5, 0, 0, 0, 94, 71, 83, 80, 67, 47, 161, 20, 33, 0, 0, 0, 0],
        }
    }
}

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct Uest {
    pub req: Req,
    pub res: Res,
}

#[derive(OBIDecode, OBIEncode, Clone, Debug, PartialEq)]
pub struct VWVP {
    pub address: Vec<u8>,
    pub voting_power: u64,
}

impl VWVP {
    pub fn get_mock() -> Self {
        VWVP {
            address: hex::decode("bd365f5aa7e8f33698345200e6b8680f4ac64831").unwrap(),
            voting_power: 100,
        }
    }
}

#[derive(OBIDecode, OBIEncode, Clone, Debug, PartialEq)]
pub struct VWVPs {
    pub validators: Vec<VWVP>,
}

impl VWVPs {
    pub fn get_mock() -> Self {
        VWVPs {
            validators: vec![
                VWVP::get_mock(),
                VWVP::get_mock(),
                VWVP::get_mock(),
                VWVP::get_mock(),
            ],
        }
    }
}
