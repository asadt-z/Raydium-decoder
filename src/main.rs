use smallvec::SmallVec;
use std::env;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum PoolKind {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema,
    Lifinity,
    Mercurial,
    Cykura,
    Serum,
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin,
    AldrinV2,
    Whirlpool,
    Invariant,
    Meteora,
    GooseFX,
    DeltaFi,
    Balansol,
    MarcoPolo,
    Dradex,
    LifinityV2,
    RaydiumClmm,
    Openbook,
    Phoenix,
    Symmetry,
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake,
    GooseFXV2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2,
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake,
    Clone,
    SanctumS,
    SanctumSAddLiquidity,
    SanctumSRemoveLiquidity,
    RaydiumCP,
    WhirlpoolSwapV2,
    OneIntro,
    PumpdotfunWrappedBuy,
    PumpdotfunWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric,
    FoxBuyFromEstimatedCost,
    FoxClaimPartial,
    SolFi,
    SolayerDelegateNoInit,
    SolayerUndelegateNoInit,
    TokenMill,
    DaosFunBuy,
    DaosFunSell,
    ZeroFi,
    StakeDexWithdrawWrappedSol,
    VirtualsBuy,
    VirtualsSell,
    Perena,
    PumpdotfunAmmBuy,
    PumpdotfunAmmSell,
    Gamma,
    MeteoraDlmmSwapV2,
    Woofi,
    MeteoraDammV2,
    MeteoraDynamicBondingCurveSwap,
    StabbleStableSwapV2,
    StabbleWeightedSwapV2,
    RaydiumLaunchlabBuy,
    RaydiumLaunchlabSell,
    BoopdotfunWrappedBuy,
    BoopdotfunWrappedSell,
    Plasma,
    GoonFi,
    HumidiFi,
    MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
    TesseraV,
    RaydiumStable,
    Unknown(u8),
}

impl PoolKind {
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

const KIND_SKIPS: [(PoolKind, usize); 91] = [
    (PoolKind::Saber, 0),
    (PoolKind::SaberAddDecimalsDeposit, 0),
    (PoolKind::SaberAddDecimalsWithdraw, 0),
    (PoolKind::TokenSwap, 0),
    (PoolKind::Sencha, 0),
    (PoolKind::Step, 0),
    (PoolKind::Cropper, 0),
    (PoolKind::Raydium, 0),
    (PoolKind::Crema, 1),
    (PoolKind::Lifinity, 0),
    (PoolKind::Mercurial, 0),
    (PoolKind::Cykura, 0),
    (PoolKind::Serum, 1),
    (PoolKind::MarinadeDeposit, 0),
    (PoolKind::MarinadeUnstake, 0),
    (PoolKind::Aldrin, 1),
    (PoolKind::AldrinV2, 1),
    (PoolKind::Whirlpool, 1),
    (PoolKind::Invariant, 1),
    (PoolKind::Meteora, 0),
    (PoolKind::GooseFX, 0),
    (PoolKind::DeltaFi, 1),
    (PoolKind::Balansol, 0),
    (PoolKind::MarcoPolo, 1),
    (PoolKind::Dradex, 1),
    (PoolKind::LifinityV2, 0),
    (PoolKind::RaydiumClmm, 0),
    (PoolKind::Openbook, 1),
    (PoolKind::Phoenix, 1),
    (PoolKind::Symmetry, 2),
    (PoolKind::TokenSwapV2, 0),
    (PoolKind::HeliumTreasuryManagementRedeemV0, 0),
    (PoolKind::StakeDexStakeWrappedSol, 0),
    (PoolKind::StakeDexSwapViaStake, 1),
    (PoolKind::GooseFXV2, 0),
    (PoolKind::Perps, 0),
    (PoolKind::PerpsAddLiquidity, 0),
    (PoolKind::PerpsRemoveLiquidity, 0),
    (PoolKind::MeteoraDlmm, 0),
    (PoolKind::OpenBookV2, 1),
    (PoolKind::RaydiumClmmV2, 0),
    (PoolKind::StakeDexPrefundWithdrawStakeAndDepositStake, 1),
    (PoolKind::Clone, 3),
    (PoolKind::SanctumS, 4),
    (PoolKind::SanctumSAddLiquidity, 2),
    (PoolKind::SanctumSRemoveLiquidity, 2),
    (PoolKind::RaydiumCP, 0),
    (PoolKind::WhirlpoolSwapV2, 2),
    (PoolKind::OneIntro, 0),
    (PoolKind::PumpdotfunWrappedBuy, 0),
    (PoolKind::PumpdotfunWrappedSell, 0),
    (PoolKind::PerpsV2, 0),
    (PoolKind::PerpsV2AddLiquidity, 0),
    (PoolKind::PerpsV2RemoveLiquidity, 0),
    (PoolKind::MoonshotWrappedBuy, 0),
    (PoolKind::MoonshotWrappedSell, 0),
    (PoolKind::StabbleStableSwap, 0),
    (PoolKind::StabbleWeightedSwap, 0),
    (PoolKind::Obric, 1),
    (PoolKind::FoxBuyFromEstimatedCost, 0),
    (PoolKind::FoxClaimPartial, 1),
    (PoolKind::SolFi, 1),
    (PoolKind::SolayerDelegateNoInit, 0),
    (PoolKind::SolayerUndelegateNoInit, 0),
    (PoolKind::TokenMill, 1),
    (PoolKind::DaosFunBuy, 0),
    (PoolKind::DaosFunSell, 0),
    (PoolKind::ZeroFi, 0),
    (PoolKind::StakeDexWithdrawWrappedSol, 0),
    (PoolKind::VirtualsBuy, 0),
    (PoolKind::VirtualsSell, 0),
    (PoolKind::Perena, 2),
    (PoolKind::PumpdotfunAmmBuy, 0),
    (PoolKind::PumpdotfunAmmSell, 0),
    (PoolKind::Gamma, 0),
    (PoolKind::MeteoraDlmmSwapV2, 1),
    (PoolKind::Woofi, 0),
    (PoolKind::MeteoraDammV2, 0),
    (PoolKind::MeteoraDynamicBondingCurveSwap, 0),
    (PoolKind::StabbleStableSwapV2, 0),
    (PoolKind::StabbleWeightedSwapV2, 0),
    (PoolKind::RaydiumLaunchlabBuy, 1),
    (PoolKind::RaydiumLaunchlabSell, 1),
    (PoolKind::BoopdotfunWrappedBuy, 0),
    (PoolKind::BoopdotfunWrappedSell, 0),
    (PoolKind::Plasma, 1),
    (PoolKind::GoonFi, 2),
    (PoolKind::HumidiFi, 2),
    (
        PoolKind::MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
        0,
    ),
    (PoolKind::TesseraV, 1),
    (PoolKind::RaydiumStable, 0),
];

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
    data_id: Option<u8>,
    data_length: u8,
    no_need: Option<[u8; 3]>,
    data: SmallVec<[PoolStep; 8]>,
    in_amount: u64,
    quoted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}

fn parse_jupiter_instruction(hex_str: &str) -> Option<ParsedInstruction> {
    let mut raw_bytes = vec![0; (hex_str.len() + 1) / 2]; // Safe size for odd lengths
    if faster_hex::hex_decode(hex_str.as_bytes(), &mut raw_bytes).is_err() {
        return None;
    }

    if raw_bytes.len() <= 40 {
        return None;
    }

    let mut idx: usize = 0;

    let not_needed = read_u64_le(&raw_bytes, &mut idx)?;

    let (data_id, data_length) = if raw_bytes.get(idx + 1..idx + 4) == Some(&[0, 0, 0]) {
        // No data_id, advance past the first byte (assuming it's the length or zero)
        idx += 1; // Adjust if the slice get is off by one; test with data
        (None, read_u8(&raw_bytes, &mut idx)?)
    } else {
        (
            Some(read_u8(&raw_bytes, &mut idx)?),
            read_u8(&raw_bytes, &mut idx)?,
        )
    };

    //let no_need = read_array3_le(&raw_bytes, &mut idx)?;
    let no_need = None;
    idx += 3;

    let data_length = if data_length == 0 { 1 } else { data_length };
    let mut data: SmallVec<[PoolStep; 8]> = SmallVec::new();

    // Manual first step with early exit
    let pool_id = read_u8(&raw_bytes, &mut idx)?;
    let (kind, skip) = if (pool_id as usize) < KIND_SKIPS.len() {
        KIND_SKIPS[pool_id as usize]
    } else {
        (PoolKind::Unknown(pool_id), 0)
    };

    if !kind.is_target() {
        return None; // No target in first swap
    }

    idx += skip;

    let percent = read_u8(&raw_bytes, &mut idx)?;
    let input_index = read_u8(&raw_bytes, &mut idx)?;
    let output_index = read_u8(&raw_bytes, &mut idx)?;

    data.push(PoolStep::Simple {
        kind,
        percent,
        input_index,
        output_index,
    });

    // Loop for remaining steps
    for _ in 1..data_length {
        let pool_id = read_u8(&raw_bytes, &mut idx)?;
        let (kind, skip) = if (pool_id as usize) < KIND_SKIPS.len() {
            KIND_SKIPS[pool_id as usize]
        } else {
            (PoolKind::Unknown(pool_id), 0)
        };

        idx += skip;

        let percent = read_u8(&raw_bytes, &mut idx)?;
        let input_index = read_u8(&raw_bytes, &mut idx)?;
        let output_index = read_u8(&raw_bytes, &mut idx)?;

        data.push(PoolStep::Simple {
            kind,
            percent,
            input_index,
            output_index,
        });
    }

    let in_amount = read_u64_le(&raw_bytes, &mut idx)?;
    let quoted_out_amount = read_u64_le(&raw_bytes, &mut idx)?;
    let slippage_bps = read_u16_le(&raw_bytes, &mut idx)?;
    let platform_fee_bps = read_u8(&raw_bytes, &mut idx)?;

    // Uncomment for debugging: println!("Parse time: {:?}", time.elapsed());

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
}

// Helper functions for manual reading (little-endian)
fn read_u8(bytes: &[u8], idx: &mut usize) -> Option<u8> {
    let val = *bytes.get(*idx)?;
    *idx += 1;
    Some(val)
}

fn read_u16_le(bytes: &[u8], idx: &mut usize) -> Option<u16> {
    let slice = bytes.get(*idx..*idx + 2)?;
    *idx += 2;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u64_le(bytes: &[u8], idx: &mut usize) -> Option<u64> {
    let slice = bytes.get(*idx..*idx + 8)?;
    *idx += 8;
    Some(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

fn main() {
    let time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run --release -- <hex_string>");
        return;
    }

    match parse_jupiter_instruction(&args[1]) {
        Some(parsed) => println!("{:#?}", parsed),
        None => println!("No target pools found."),
    }
    println!("Total time: {:?}", time.elapsed());
}
