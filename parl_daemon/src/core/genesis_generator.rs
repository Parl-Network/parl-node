use parl_core::block::Block;
use parl_core::address::Address;
use parl_core::crypto::Hash;
use parl_core::network::Network;
use parl_core::transaction::Transaction;
use parl_core::config::get_minimum_difficulty;

fn main() {
    // Dirección del desarrollador
    let dev_address = Address::from_string("prl:tnw644wltt78fnellgtf6sesu9v4hecsm36308s3cua3k0g40p3qqd3vrel").unwrap();

    // Crear una transacción coinbase (sin entrada, solo salida)
    let coinbase_tx = Transaction::new_coinbase(dev_address.clone(), 0);

    // Crear el bloque génesis
    let genesis_block = Block::new_genesis(
        Network::Mainnet,
        vec![coinbase_tx],
        get_minimum_difficulty(),
    );

    // Mostrar bloque en hexadecimal
    let hex = hex::encode(bincode::serialize(&genesis_block).unwrap());
    println!("GENESIS BLOCK HEX:\n{}\n", hex);

    // Mostrar hash del bloque génesis
    let hash: Hash = genesis_block.hash();
    println!("GENESIS BLOCK HASH:\n{:?}", hash);
}
