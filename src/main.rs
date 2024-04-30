use std::str::FromStr;

use fuel_asm::{op, GTFArgs, RegId};
use fuel_tx::{
    AssetId, Bytes32, Finalizable, Input, Output, TransactionBuilder, TxId, TxPointer,
    UniqueIdentifier, UtxoId,
};
use fuels::client::{FuelClient, PaginationRequest};
use fuels::crypto::{PublicKey, SecretKey};
use fuels::types::{BlockHeight, Nonce};

// #[tokio::main]
// async fn main() {
//     use itertools::Itertools;

//     let secret =
//         SecretKey::from_str("de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c")
//             .unwrap();
//     let public_key: PublicKey = (&secret).into();
//     let address = Input::owner(&public_key);
//     let client = FuelClient::new("127.0.0.1:4000").unwrap();

//     let script_data = [address.to_vec(), 100u64.to_be_bytes().to_vec()]
//         .into_iter()
//         .flatten()
//         .collect_vec();

//     let script: Vec<u8> = vec![
//         op::gtf(0x10, 0x00, GTFArgs::ScriptData.into()),
//         op::addi(0x11, 0x10, Bytes32::LEN as u16),
//         op::lw(0x11, 0x11, 0),
//         op::smo(0x10, 0x00, 0x00, 0x11),
//         op::ret(RegId::ONE),
//     ]
//     .into_iter()
//     .collect();

//     let outputs = vec![Output::change(address, 0, AssetId::BASE)];
//     let mut tx = TransactionBuilder::script(script, script_data);
//     tx.script_gas_limit(1_000_000);

//     for output in outputs {
//         tx.add_output(output);
//     }

//     tx.add_unsigned_coin_input(
//         secret,
//         UtxoId::new(
//             TxId::from_str("0000000000000000000000000000000000000000000000000000000000000001")
//                 .unwrap(),
//             0,
//         ),
//         1152921504606846976,
//         AssetId::default(),
//         TxPointer::default(),
//     );

//     let tx = tx.finalize();
//     let tx_id = tx.id(&0.into());

//     let response = client.submit_and_await_commit(&tx.into()).await.unwrap();
//     dbg!(&response);

//     let response = client
//         .transaction(&tx_id)
//         .await
//         .unwrap()
//         .expect(&format!("transaction should exist {tx_id}"));

//     dbg!(response);
// }

#[tokio::main]
async fn main() {
    let client = FuelClient::new("127.0.0.1:4000").unwrap();

    client.produce_blocks(1, None).await.unwrap();

    let nonce = Nonce::from_str("0xe9fe0fca4ca4b9e5c65d22031b28d1094aa67658ab234a642b0df7b58497fe19").unwrap();
    let tx_id = TxId::from_str("91516145ef118f644fc807de202f4f3a8634bb03c7aab44c0258e1717f3d27d6").unwrap();
    let block_height: BlockHeight = 2.into();

    let proof =
                client
                .message_proof(&tx_id, &nonce, None, Some(block_height))
                .await
                .unwrap()
                .expect("proof should exist");

    dbg!(proof);
}
