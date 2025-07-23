#![allow(warnings)]
use borsh::BorshDeserialize;
use hex::{self, decode};
use std::env;
use std::time::Instant; // for me only

#[derive(Debug)]
// Pools
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
    fn get_kind_and_skip_bytes(id: u8) -> (Self, usize) {
        let kind = match id {
            0 => PoolKind::Saber,
            1 => PoolKind::SaberAddDecimalsDeposit,
            2 => PoolKind::SaberAddDecimalsWithdraw,
            3 => PoolKind::TokenSwap,
            4 => PoolKind::Sencha,
            5 => PoolKind::Step,
            6 => PoolKind::Cropper,
            7 => PoolKind::Raydium,
            8 => PoolKind::Crema,
            9 => PoolKind::Lifinity,
            10 => PoolKind::Mercurial,
            11 => PoolKind::Cykura,
            12 => PoolKind::Serum,
            13 => PoolKind::MarinadeDeposit,
            14 => PoolKind::MarinadeUnstake,
            15 => PoolKind::Aldrin,
            16 => PoolKind::AldrinV2,
            17 => PoolKind::Whirlpool,
            18 => PoolKind::Invariant,
            19 => PoolKind::Meteora,
            20 => PoolKind::GooseFX,
            21 => PoolKind::DeltaFi,
            22 => PoolKind::Balansol,
            23 => PoolKind::MarcoPolo,
            24 => PoolKind::Dradex,
            25 => PoolKind::LifinityV2,
            26 => PoolKind::RaydiumClmm,
            27 => PoolKind::Openbook,
            28 => PoolKind::Phoenix,
            29 => PoolKind::Symmetry,
            30 => PoolKind::TokenSwapV2,
            31 => PoolKind::HeliumTreasuryManagementRedeemV0,
            32 => PoolKind::StakeDexStakeWrappedSol,
            33 => PoolKind::StakeDexSwapViaStake,
            34 => PoolKind::GooseFXV2,
            35 => PoolKind::Perps,
            36 => PoolKind::PerpsAddLiquidity,
            37 => PoolKind::PerpsRemoveLiquidity,
            38 => PoolKind::MeteoraDlmm,
            39 => PoolKind::OpenBookV2,
            40 => PoolKind::RaydiumClmmV2,
            41 => PoolKind::StakeDexPrefundWithdrawStakeAndDepositStake,
            42 => PoolKind::Clone,
            43 => PoolKind::SanctumS,
            44 => PoolKind::SanctumSAddLiquidity,
            45 => PoolKind::SanctumSRemoveLiquidity,
            46 => PoolKind::RaydiumCP,
            47 => PoolKind::WhirlpoolSwapV2,
            48 => PoolKind::OneIntro,
            49 => PoolKind::PumpdotfunWrappedBuy,
            50 => PoolKind::PumpdotfunWrappedSell,
            51 => PoolKind::PerpsV2,
            52 => PoolKind::PerpsV2AddLiquidity,
            53 => PoolKind::PerpsV2RemoveLiquidity,
            54 => PoolKind::MoonshotWrappedBuy,
            55 => PoolKind::MoonshotWrappedSell,
            56 => PoolKind::StabbleStableSwap,
            57 => PoolKind::StabbleWeightedSwap,
            58 => PoolKind::Obric,
            59 => PoolKind::FoxBuyFromEstimatedCost,
            60 => PoolKind::FoxClaimPartial,
            61 => PoolKind::SolFi,
            62 => PoolKind::SolayerDelegateNoInit,
            63 => PoolKind::SolayerUndelegateNoInit,
            64 => PoolKind::TokenMill,
            65 => PoolKind::DaosFunBuy,
            66 => PoolKind::DaosFunSell,
            67 => PoolKind::ZeroFi,
            68 => PoolKind::StakeDexWithdrawWrappedSol,
            69 => PoolKind::VirtualsBuy,
            70 => PoolKind::VirtualsSell,
            71 => PoolKind::Perena,
            72 => PoolKind::PumpdotfunAmmBuy,
            73 => PoolKind::PumpdotfunAmmSell,
            74 => PoolKind::Gamma,
            75 => PoolKind::MeteoraDlmmSwapV2,
            76 => PoolKind::Woofi,
            77 => PoolKind::MeteoraDammV2,
            78 => PoolKind::MeteoraDynamicBondingCurveSwap,
            79 => PoolKind::StabbleStableSwapV2,
            80 => PoolKind::StabbleWeightedSwapV2,
            81 => PoolKind::RaydiumLaunchlabBuy,
            82 => PoolKind::RaydiumLaunchlabSell,
            83 => PoolKind::BoopdotfunWrappedBuy,
            84 => PoolKind::BoopdotfunWrappedSell,
            85 => PoolKind::Plasma,
            86 => PoolKind::GoonFi,
            87 => PoolKind::HumidiFi,
            88 => PoolKind::MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
            89 => PoolKind::TesseraV,
            90 => PoolKind::RaydiumStable,
            other => PoolKind::Unknown(other),
        };

        let skip = match kind {
            PoolKind::Saber
            | PoolKind::SaberAddDecimalsDeposit
            | PoolKind::SaberAddDecimalsWithdraw
            | PoolKind::TokenSwap
            | PoolKind::Sencha
            | PoolKind::Step
            | PoolKind::Cropper
            | PoolKind::Raydium
            | PoolKind::Lifinity
            | PoolKind::Mercurial
            | PoolKind::Cykura
            | PoolKind::MarinadeDeposit
            | PoolKind::MarinadeUnstake
            | PoolKind::Meteora
            | PoolKind::GooseFX
            | PoolKind::Balansol
            | PoolKind::LifinityV2
            | PoolKind::RaydiumClmm
            | PoolKind::TokenSwapV2
            | PoolKind::HeliumTreasuryManagementRedeemV0
            | PoolKind::StakeDexStakeWrappedSol
            | PoolKind::GooseFXV2
            | PoolKind::Perps
            | PoolKind::PerpsAddLiquidity
            | PoolKind::PerpsRemoveLiquidity
            | PoolKind::MeteoraDlmm
            | PoolKind::RaydiumClmmV2
            | PoolKind::RaydiumCP
            | PoolKind::OneIntro
            | PoolKind::PumpdotfunWrappedBuy
            | PoolKind::PumpdotfunWrappedSell
            | PoolKind::PerpsV2
            | PoolKind::PerpsV2AddLiquidity
            | PoolKind::PerpsV2RemoveLiquidity
            | PoolKind::MoonshotWrappedBuy
            | PoolKind::MoonshotWrappedSell
            | PoolKind::StabbleStableSwap
            | PoolKind::StabbleWeightedSwap
            | PoolKind::FoxBuyFromEstimatedCost
            | PoolKind::SolayerDelegateNoInit
            | PoolKind::SolayerUndelegateNoInit
            | PoolKind::DaosFunBuy
            | PoolKind::DaosFunSell
            | PoolKind::ZeroFi
            | PoolKind::StakeDexWithdrawWrappedSol
            | PoolKind::VirtualsBuy
            | PoolKind::VirtualsSell
            | PoolKind::PumpdotfunAmmBuy
            | PoolKind::PumpdotfunAmmSell
            | PoolKind::Gamma
            | PoolKind::Woofi
            | PoolKind::MeteoraDammV2
            | PoolKind::MeteoraDynamicBondingCurveSwap
            | PoolKind::StabbleStableSwapV2
            | PoolKind::StabbleWeightedSwapV2
            | PoolKind::BoopdotfunWrappedBuy
            | PoolKind::BoopdotfunWrappedSell
            | PoolKind::MeteoraDynamicBondingCurveSwapWithRemainingAccounts
            | PoolKind::RaydiumStable
            | PoolKind::PumpdotfunAmmSell => 0,

            PoolKind::Crema
            | PoolKind::Serum
            | PoolKind::Aldrin
            | PoolKind::AldrinV2
            | PoolKind::Whirlpool
            | PoolKind::Invariant
            | PoolKind::DeltaFi
            | PoolKind::MarcoPolo
            | PoolKind::Dradex
            | PoolKind::Openbook
            | PoolKind::Phoenix
            | PoolKind::StakeDexSwapViaStake
            | PoolKind::OpenBookV2
            | PoolKind::StakeDexPrefundWithdrawStakeAndDepositStake
            | PoolKind::Obric
            | PoolKind::FoxClaimPartial
            | PoolKind::SolFi
            | PoolKind::TokenMill
            | PoolKind::MeteoraDlmmSwapV2
            | PoolKind::RaydiumLaunchlabBuy
            | PoolKind::RaydiumLaunchlabSell
            | PoolKind::Plasma
            | PoolKind::TesseraV => 1,

            PoolKind::Symmetry
            | PoolKind::SanctumSAddLiquidity
            | PoolKind::SanctumSRemoveLiquidity
            | PoolKind::WhirlpoolSwapV2
            | PoolKind::Perena
            | PoolKind::GoonFi
            | PoolKind::HumidiFi => 2,

            PoolKind::Clone => 3,

            PoolKind::SanctumS => 4,

            PoolKind::Unknown(_) => 0,
        };

        (kind, skip)
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
    data_id: Option<u8>, // Sometimes it is not used
    data_length: u8,
    no_need: [u8; 3], // 0,0,0
    data: Vec<PoolStep>,
    in_amount: u64,
    quoted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}

fn parse_pool_step(input: &mut &[u8], step: u8) -> Option<PoolStep> {
    let pool_id = u8::deserialize(input).ok()?;

    let (kind, skip) = PoolKind::get_kind_and_skip_bytes(pool_id);

    if step == 1 && !kind.is_target() {
        return None; // No in target in first swap - 404
    }

    // println!("{:?} with id :{:?}\n\n", kind, pool_id);

    // do not need to change slice
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

fn parse_jupiter_instruction(hex_str: &str) -> Option<ParsedInstruction> {
    let time = Instant::now();
    let raw_bytes = hex::decode(hex_str).ok()?;

    //print!("Raw Bytes:{:?}\n\n", raw_bytes);

    let mut input = raw_bytes.as_slice();

    if input.len() <= 13 {
        return None;
    }
    //WHAT IS DATA IDW
    let not_needed = u64::deserialize(&mut input).ok()?;

    //Peek first byte after u64
    let peek_next_3 = input.get(1..4)?;

    let (data_id, data_length) = if peek_next_3 == [0, 0, 0] {
        // No data_id
        let data_length = u8::deserialize(&mut input).ok()?;
        (None, data_length)
    } else {
        // We have data_id
        let data_id = u8::deserialize(&mut input).ok()?;
        let data_length = u8::deserialize(&mut input).ok()?;
        (Some(data_id), data_length)
    };

    let no_need: [u8; 3] = <[u8; 3]>::deserialize(&mut input).ok()?; // 

    let data_length = if data_length == 0 { 1 } else { data_length };

    // let mut temp_input = input;
    let first_step = parse_pool_step(&mut input, 1)?;

    let mut data = Vec::with_capacity(data_length as usize);
    data.push(first_step);

    for i in 1..data_length {
        let step = parse_pool_step(&mut input, i + 1)?;
        data.push(step);
    }
    let in_amount = u64::deserialize(&mut input).ok()?; // merge steps
    let quoted_out_amount = u64::deserialize(&mut input).ok()?;
    let slippage_bps = u16::deserialize(&mut input).ok()?;
    let platform_fee_bps = u8::deserialize(&mut input).ok()?;

    println!("{:?}", time.elapsed());
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

fn main() {
    let time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <argument>");
        return;
    }

    match parse_jupiter_instruction(&args[1]) {
        Some(parsed) => println!("{:#?}", parsed),
        None => println!("No target pools found."),
    }
    println!("{:?}", time.elapsed());
}
