use katana_primitives::block::{Block, BlockHash, FinalityStatus, Header, SealedBlockWithStatus};
use katana_primitives::execution::TypedTransactionExecutionInfo;
use katana_primitives::fee::FeeInfo;
use katana_primitives::receipt::{InvokeTxReceipt, Receipt};
use katana_primitives::transaction::{InvokeTx, Tx, TxHash, TxWithHash};
use katana_primitives::Felt;

pub fn generate_dummy_txs_and_receipts(
    count: usize,
) -> (Vec<TxWithHash>, Vec<Receipt>, Vec<TypedTransactionExecutionInfo>) {
    let mut txs = Vec::with_capacity(count);
    let mut receipts = Vec::with_capacity(count);
    let mut executions = Vec::with_capacity(count);

    // TODO: generate random txs and receipts variants
    for _ in 0..count {
        txs.push(TxWithHash {
            hash: TxHash::from(rand::random::<u128>()),
            transaction: Tx::Invoke(InvokeTx::V1(Default::default())),
        });

        receipts.push(Receipt::Invoke(InvokeTxReceipt {
            revert_error: None,
            events: Vec::new(),
            messages_sent: Vec::new(),
            fee: FeeInfo::default(),
            execution_resources: Default::default(),
        }));
        executions.push(TypedTransactionExecutionInfo::default());
    }

    (txs, receipts, executions)
}

pub fn generate_dummy_blocks_and_receipts(
    count: u64,
) -> Vec<(SealedBlockWithStatus, Vec<Receipt>, Vec<TypedTransactionExecutionInfo>)> {
    let mut blocks = Vec::with_capacity(count as usize);
    let mut parent_hash: BlockHash = 0u8.into();

    for i in 0..count {
        let tx_count = (rand::random::<u64>() % 10) as usize;
        let (body, receipts, executions) = generate_dummy_txs_and_receipts(tx_count);

        let header = Header { parent_hash, number: i, ..Default::default() };
        let block = Block { header, body }.seal_with_hash(Felt::from(rand::random::<u128>()));

        parent_hash = block.hash;

        blocks.push((
            SealedBlockWithStatus { block, status: FinalityStatus::AcceptedOnL2 },
            receipts,
            executions,
        ));
    }

    blocks
}

pub fn generate_dummy_blocks_empty(count: u64) -> Vec<SealedBlockWithStatus> {
    let mut blocks = Vec::with_capacity(count as usize);
    let mut parent_hash: BlockHash = 0u8.into();

    for i in 0..count {
        let header = Header { parent_hash, number: i, ..Default::default() };
        let body = vec![];

        let block = Block { header, body }.seal_with_hash(Felt::from(rand::random::<u128>()));

        parent_hash = block.hash;

        blocks.push(SealedBlockWithStatus { block, status: FinalityStatus::AcceptedOnL2 });
    }

    blocks
}
