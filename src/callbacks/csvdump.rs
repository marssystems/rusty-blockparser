use blockchain::proto::block::Block;
use blockchain::proto::Hashed;
use blockchain::utils;
 use rustc_serialize::base64::{ToBase64,STANDARD};

 
 /// Dumps the whole blockchain into csv files
 pub struct CsvDump {
     // Each structure gets stored in a seperate csv file
     dump_folder:    PathBuf,
     block_writer:   BufWriter<File>,
     tx_writer:      BufWriter<File>,
     txin_writer:    BufWriter<File>,
     txout_writer:   BufWriter<File>,
 
impl Callback for CsvDump {
             let cap = 4000000;
             let cb = CsvDump {
                 dump_folder:    PathBuf::from(dump_folder),
                 block_writer:   try!(CsvDump::create_writer(cap, dump_folder.join("blocks.csv.tmp"))),
                 tx_writer:      try!(CsvDump::create_writer(cap, dump_folder.join("transactions.csv.tmp"))),
                 txin_writer:    try!(CsvDump::create_writer(cap, dump_folder.join("tx_in.csv.tmp"))),
                 txout_writer:   try!(CsvDump::create_writer(cap, dump_folder.join("tx_out.csv.tmp"))),
                 start_height: 0, end_height: 0, tx_count: 0, in_count: 0, out_count: 0
	}
	 }	
impl Callback for CsvDump {
 
     fn on_block(&mut self, block: Block, block_height: usize) {
         // serialize block
         self.block_writer.write_all(block.as_csv(block_height).as_bytes()).unwrap();
 
         // serialize transaction
         let block_hash = utils::arr_to_hex_swapped(&block.header.hash);
         for tx in block.txs {
             self.tx_writer.write_all(tx.as_csv(&block_hash).as_bytes()).unwrap();
             let txid_str = utils::arr_to_hex_swapped(&tx.hash);
 
             // serialize inputs
impl TxInput {
     #[inline]
     fn as_csv(&self, txid: &str) -> String {
         (@txid, @hashPrevOut, indexPrevOut, scriptSig, sequence)
        format!("{};{};{};{};{}\n",
		format!("{};{}\n",
             &txid,
            &utils::arr_to_hex_swapped(&self.outpoint.txid),
            &self.outpoint.index,
            &utils::arr_to_hex(&self.script_sig),
            &self.script_sig.to_base64(STANDARD),
			&self.seq_no
			)
     }
 }
 
impl EvaluatedTxOut {
     #[inline]
     fn as_csv(&self, txid: &str, index: usize) -> String {
         (@txid, indexOut, value, @scriptPubKey, address)
        format!("{};{};{};{};{}\n",
        format!("{};\n",
            &txid,
            &index,
            &self.out.value,
            &utils::arr_to_hex(&self.out.script_pubkey),
            &self.script.address)
     }
 }
	}
