pub mod first;
use bs58;
use std::mem;

#[cfg(test)]
mod tests {
    use super::*;
    use first::*;
    use hex;
    use obi::{OBIDecode, OBIEncode};

    #[test]
    fn test_borsh_1() {
        let p = Proof::get_mock();
        let x = p.try_to_vec();
        println!("{:?}", hex::encode(p.try_to_vec().unwrap()));
    }

    #[test]
    fn test_borsh_5() {
        let req = Req::get_mock().try_to_vec().unwrap();
        let res = Res::get_mock().try_to_vec().unwrap();

        println!("{:?}", hex::encode(&req));
        println!("{:?}", hex::encode(&res));

        // let x = Uest { req, res }.try_to_vec().unwrap();

        // println!("{:?}", hex::encode(x).into_string());

        println!(
            "{:?}",
            hex::encode(
                Uest {
                    req: Req::get_mock(),
                    res: Res::get_mock()
                }
                .try_to_vec()
                .unwrap()
            ) // .into_string()
        );
    }

    #[test]
    fn test_borsh_6() {
        println!("{:?}", hex::encode(VWVPs::get_mock().try_to_vec().unwrap()));
    }
}
