#![allow(warnings)]
use borsh::BorshDeserialize;
use hex;
use std::time::Instant;

#[derive(Debug)]

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
    data_id: u8,
    data_length: u8,
    no_need: [u8; 3],
    data: Vec<PoolStep>,
    in_amount: u64,
    quoted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}

fn parse_pool_step(input: &mut &[u8]) -> Option<PoolStep> {
    let pool_id = u8::deserialize(input).ok()?;

    let kind = PoolKind::from_id(pool_id);
    // do not need to change slice
    let skip = kind.skip_bytes();
    *input = &input[skip..];

    let percent = u8::deserialize(input).ok()?;
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
    // change input
    if input.len() <= 8 {
        return None;
    }

    let not_needed = u64::deserialize(&mut input).ok()?;
    let data_id = u8::deserialize(&mut input).ok()?; // 5
    let data_length = u8::deserialize(&mut input).ok()?; // 2 
    let no_need: [u8; 3] = <[u8; 3]>::deserialize(&mut input).ok()?; // 0,0,0

    let mut data = Vec::with_capacity(data_length as usize); // change size to u8*4
    for _ in 0..data_length {
        let step = parse_pool_step(&mut input)?;
        data.push(step);
    }

    let in_amount = u64::deserialize(&mut input).ok()?;
    let quoted_out_amount = u64::deserialize(&mut input).ok()?;
    let slippage_bps = u16::deserialize(&mut input).ok()?;
    let platform_fee_bps = u8::deserialize(&mut input).ok()?;

    let includes_target = data.iter().any(|step| match step {
        PoolStep::Simple { kind, .. } => kind.is_target(),
    });

    if includes_target {
        Some(ParsedInstruction {
            not_needed,
            data_id,
            data_length,
            no_need,
            data,
            in_amount,
            quoted_out_amount,
            slippage_bps,
            platform_fee_bps,
        })
    } else {
        None
    }
}

fn main() {
    let time = Instant::now();
    let raw_bytes =
        hex::decode("e517cb977ae3ad2a0100000049640001bceadd0463000000aed7373b00000000320000")
            .unwrap();
    println!("{:?}", raw_bytes);
    match parse_jupiter_instruction(&raw_bytes) {
        Some(parsed) => println!("{:#?}", parsed),
        None => println!("No target pools found."),
    }
    println!("{:?}", time.elapsed());
}
