#![allow(warnings)]
use borsh::BorshDeserialize;
use hex;
use std::time::Instant;

#[derive(Debug)]
// Pools
enum PoolKind {
    Saber,
    Raydium,
    Whirlpool,
    Pheonix,
    MeteoraDlmm,
    RaydiumCP,
    WhirlpoolSwapV2,
    SolFi,
    PumpdotfunAmmBuy,
    PumpdotfunAmmSell,
    TesseraV,
    Unknown(u8),
}

impl PoolKind {
    fn from_id(id: u8) -> Self {
        match id {
            0 => PoolKind::Saber,
            7 => PoolKind::Raydium,
            17 => PoolKind::Whirlpool,
            28 => PoolKind::Pheonix,
            38 => PoolKind::MeteoraDlmm,
            46 => PoolKind::RaydiumCP,
            47 => PoolKind::WhirlpoolSwapV2,
            61 => PoolKind::SolFi,
            72 => PoolKind::PumpdotfunAmmBuy,
            73 => PoolKind::PumpdotfunAmmSell,
            89 => PoolKind::TesseraV,
            other => PoolKind::Unknown(other),
        }
    }
    // have to add all pools
    fn skip_bytes(&self) -> usize {
        match self {
            PoolKind::Raydium
            | PoolKind::Saber
            | PoolKind::MeteoraDlmm
            | PoolKind::RaydiumCP
            | PoolKind::PumpdotfunAmmSell => 0,
            PoolKind::Whirlpool
            | PoolKind::SolFi
            | PoolKind::PumpdotfunAmmBuy
            | PoolKind::Pheonix
            | PoolKind::TesseraV => 1,
            PoolKind::WhirlpoolSwapV2 => 2,
            PoolKind::Unknown(_) => 0,
        }
    }

    fn is_target(&self) -> bool {
        matches!(
            self,
            PoolKind::Raydium
                | PoolKind::RaydiumCP
                | PoolKind::PumpdotfunAmmBuy
                | PoolKind::PumpdotfunAmmSell
        )
    }
}

#[derive(Debug)]
enum PoolStep {
    Simple {
        kind: PoolKind,
        percent: u8,
        input_index: u8,
        output_index: u8,
    },
}

#[derive(Debug)]
struct ParsedInstruction {
    not_needed: u64,
    //data_id: u8, // Sometimes it is not used
    data_length: u8,
    no_need: [u8; 3], // 0,0,0
    data: Vec<PoolStep>,
    in_amount: u64,
    quoted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}

fn parse_pool_step(input: &mut &[u8]) -> Option<PoolStep> {
    let pool_id = u8::deserialize(input).ok()?;

    let kind = PoolKind::from_id(pool_id); // merge from_id and skip_bytes (returns 2 things Poolkind and skips bytes)

    println!("{:?} with id :{:?}", kind, pool_id);

    // do not need to change slice
    let skip = kind.skip_bytes();
    *input = &input[skip..];

    let percent = u8::deserialize(input).ok()?; //merge them
    let input_index = u8::deserialize(input).ok()?;
    let output_index = u8::deserialize(input).ok()?;

    Some(PoolStep::Simple {
        kind,
        percent,
        input_index,
        output_index,
    })
}

fn parse_jupiter_instruction(mut input: &[u8]) -> Option<ParsedInstruction> {
    // in parse_jupiter_instruction argument will be string of hex like "e517cb977ae3ad2a010000000764000100e1f50500000000bef1369500000000840300"
    // and first have to convert it to let raw_bytes = hex::decode("e517cb977ae3ad2a010000000764000100e1f50500000000bef1369500000000840300")
    //       .unwrap(); and then proceed to actual parsing

    if input.len() <= 8 {
        return None;
    }
    //WHAT IS DATA ID

    let not_needed = u64::deserialize(&mut input).ok()?;
    //let data_id = u8::deserialize(&mut input).ok()?; // 1  VARIES SOMETIMES ADD AND SOMETIMES NOT
    let mut data_length = u8::deserialize(&mut input).ok()?; // 0-> 1 
    let no_need: [u8; 3] = <[u8; 3]>::deserialize(&mut input).ok()?; // 

    if data_length == 0 {
        data_length = 1;
    }

    let mut temp_input = input;
    let first_step = parse_pool_step(&mut temp_input)?;

    let first_is_target = match &first_step {
        PoolStep::Simple { kind, .. } => kind.is_target(),
    };

    if !first_is_target {
        return None;
    }

    let mut data = Vec::with_capacity(data_length as usize);
    data.push(first_step);

    for _ in 1..data_length {
        let step = parse_pool_step(&mut input)?;
        data.push(step);
    }

    let in_amount = u64::deserialize(&mut input).ok()?; // merge steps
    let quoted_out_amount = u64::deserialize(&mut input).ok()?;
    let slippage_bps = u16::deserialize(&mut input).ok()?;
    let platform_fee_bps = u8::deserialize(&mut input).ok()?;

    Some(ParsedInstruction {
        not_needed,
        //data_id,
        data_length,
        no_need,
        data,
        in_amount,
        quoted_out_amount,
        slippage_bps,
        platform_fee_bps,
    })
}

fn main() {
    let time = Instant::now();
    let mut raw_bytes =
        hex::decode("e517cb977ae3ad2a010000000764000100e1f50500000000bef1369500000000840300")
            .unwrap();
    // let raw_bytes = [
    //     229, 23, 203, 151, 122, 227, 173, 42, 1, 0, 0, 0, 7, 100, 0, 1, 0, 225, 245, 5, 0, 0, 0, 0,
    //     190, 241, 54, 149, 0, 0, 0, 0, 132, 3, 0,
    // ];
    println!("{:?}", raw_bytes);
    match parse_jupiter_instruction(&raw_bytes) {
        Some(parsed) => println!("{:#?}", parsed),
        None => println!("No target pools found."),
    }
    println!("{:?}", time.elapsed());
}
