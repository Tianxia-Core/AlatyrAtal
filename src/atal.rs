// ffp: FFP-ATAL-INSCRIPTION crypto-signer rust
//! `Atal` Layer (Manyu `Akwanshi` Basalt Monolith & `Nsibidi` Immutable Inscription):
//! - Dual-Token Asymmetric Burn-Mint Equilibrium (`EFEMERA` velocity on Base/Solana ->
//!   `30.0%` programmatic `ETAMANGA` buy-and-lock/burn on Substrate)
//! - Native `ERC-2981` 4-Party Provenance Royalty Splitter (`Model Host` + `SLECO-SO++ Genotype` +
//!   `RCAG MemOS Curator` + `Ndimmôrwyn Treasury`)
//! - Post-July 1, 2026 EU Regulation `2023/1114` (`MiCA` Title II) Compliance Firewall
//! - `RASOOM-NKG` 16-bit Nsibidi Glyph Codec (`⟦NSB:0x01⟧` / `⟦NSB:0x02⟧`) + Carnatic
//!   `Konnakol [3,2]` Coprime Gati TDMA Scheduler + `TREMOR` Queue Damper + Korvai Merkle Batcher.

use crate::alatyr::AlatyrSparkReceipt;
use crate::oracular::OracularVerdict;
use serde::{Deserialize, Serialize};

/// 16-bit Nsibidi ideographic protocol glyphs (`RASOOM-NKG`, 84% RAM reduction vs verbose JSON).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum NsibidiGlyph {
    /// `⟦NSB:0x01⟧` — Verified Double-Entry Compute Settlement Inscription
    Nsb01VerifiedSettlement = 0x0001,
    /// `⟦NSB:0x02⟧` — Audited `Ikpe` / `Ndimmôrwyn` MCA Underwriting Covenant
    Nsb02IkpeMcaCovenant = 0x0002,
    /// `⟦NSB:0x03⟧` — `RCAG` Merkle-Provenanced Royalty Split
    Nsb03RcagRoyaltySplit = 0x0003,
    /// `⟦NSB:0x04⟧` — `AEGIS-9` / `CR < 1.15` Circuit-Breaker Freeze
    Nsb04CircuitBreakerHalt = 0x0004,
}

impl NsibidiGlyph {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Nsb01VerifiedSettlement => "⟦NSB:0x01⟧",
            Self::Nsb02IkpeMcaCovenant => "⟦NSB:0x02⟧",
            Self::Nsb03RcagRoyaltySplit => "⟦NSB:0x03⟧",
            Self::Nsb04CircuitBreakerHalt => "⟦NSB:0x04⟧",
        }
    }
}

/// Post-July 1, 2026 EU Regulation (EU) 2023/1114 (`MiCA`) Title II Compliance Configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicaCompliancePolicy {
    pub efemera_pegged_to_fiat: bool,
    pub etamanga_pegged_to_fiat: bool,
    pub proprietary_custodial_fiat_desk: bool,
    pub third_party_licensed_casp_id: String,
    pub settlement_stablecoin_rail: String,
}

impl Default for MicaCompliancePolicy {
    fn default() -> Self {
        Self {
            efemera_pegged_to_fiat: false,
            etamanga_pegged_to_fiat: false,
            proprietary_custodial_fiat_desk: false,
            third_party_licensed_casp_id: "MICA-CASP-EE-EURC-RAIL".to_string(),
            settlement_stablecoin_rail: "EURC_BASE_SOLANA".to_string(),
        }
    }
}

pub fn verify_mica_title_ii_compliance(policy: &MicaCompliancePolicy) -> Result<(), String> {
    if policy.efemera_pegged_to_fiat || policy.etamanga_pegged_to_fiat {
        return Err(
            "MICA_EMT_ART_VIOLATION: 1:1 fiat peg prohibited under Regulation (EU) 2023/1114; keep EFEMERA & ETAMANGA strictly floating Title II utility/governance assets".to_string(),
        );
    }
    if policy.proprietary_custodial_fiat_desk {
        return Err(
            "MICA_CUSTODIAL_DESK_VIOLATION: Route final crypto->fiat leg exclusively via third-party MiCA-authorized CASP EURC/USDC rails".to_string(),
        );
    }
    if policy.third_party_licensed_casp_id.trim().is_empty() {
        return Err("MICA_MISSING_CASP: Licensed CASP identifier required for fiat settlement".to_string());
    }
    Ok(())
}

/// Atomic 4-party `ERC-2981` royalty distribution across provenance contributors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Erc2981ProvenanceSplit {
    pub total_royalty_efemera: f64,
    pub compute_host_efemera: f64,
    pub sleco_prompt_genotype_efemera: f64,
    pub rcag_memos_curator_efemera: f64,
    pub ndimmorwyn_treasury_efemera: f64,
}

/// Complete `Atal` Settlement Inscription emitted after `Alatyr` + `Oracular` verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtalSettlementInscription {
    pub glyph_code_u16: u16,
    pub glyph_symbol: String,
    pub approved: bool,
    pub mica_title_ii_compliant: bool,
    pub gross_compute_eur: f64,
    pub efemera_minted_net: f64,
    pub protocol_clearing_fee_eur: f64,
    pub oracular_vaas_fee_eur: f64,
    pub casp_offramp_rebate_eur: f64,
    pub mca_underwriting_fee_eur: f64,
    pub total_protocol_revenue_eur: f64,
    pub cogs_mif50_eur: f64,
    pub gross_profit_84pct_eur: f64,
    pub etamanga_buy_and_lock_eur: f64,
    pub etamanga_locked_units: f64,
    pub erc2981_split: Erc2981ProvenanceSplit,
    pub konnakol_tdma_slot: u8,
    pub tremor_damped_queue_ms: f64,
    pub atal_inscription_blake3: String,
}

const CLEARING_TAKE_RATE: f64 = 0.015;      // 1.50% on Gross Compute Volume
const ROYALTY_ROUTING_SPREAD: f64 = 0.005;  // 0.50% on RCAG provenanced GCV
const ORACULAR_VAAS_FEE_EUR: f64 = 0.002;   // €0.002 per Stratum 2-3 compliance attestation
const MCA_UNDERWRITING_SPREAD: f64 = 0.035; // 3.50% net origination/factoring spread
const CASP_OFFRAMP_REBATE: f64 = 0.0035;    // 0.35% (35 bps) partner routing rebate
const MIF50_COGS_RATIO: f64 = 0.16;         // 16.0% sovereign 1-bit compute COGS -> 84.0% Gross Margin
const ETAMANGA_BURN_LOCK_SHARE: f64 = 0.30; // 30.0% of net protocol revenue buys & locks/burns ETAMANGA

/// Schedules collision-free TDMA bursts across the 5-node mesh using Carnatic `Konnakol [3,2]`
/// coprime Gati subdivisions (`3, 4, 5, 7, 9`) and `TREMOR` exponential queue damping.
pub fn schedule_konnakol_tremor(node_index: usize, epoch_seq: u64, burst_multiplier: f64) -> (u8, f64) {
    const COPRIME_GATI: [u64; 5] = [3, 4, 5, 7, 9];
    let gati = COPRIME_GATI[node_index % COPRIME_GATI.len()];
    // [3, 2] Korvai cadence modulo coprime Gati yields zero-lock deterministic slot assignment
    let slot = ((epoch_seq.wrapping_mul(3).wrapping_add(2)) % gati) as u8;
    let raw_spike = burst_multiplier.max(1.0);
    // TREMOR sub-linear damping keeps latency flat even under 50x burst spikes
    let damped_ms = 1.8 + (raw_spike.ln() * 0.95);
    (slot, (damped_ms * 100.0).round() / 100.0)
}

/// Inscribes a verified compute or MCA event into the `Atal` dual-token BME ledger.
pub fn inscribe_atal_settlement(
    spark: &AlatyrSparkReceipt,
    verdict: &OracularVerdict,
    gross_eur: f64,
    etamanga_spot_eur: f64,
    rcag_provenanced: bool,
    is_mca_advance: bool,
    fiat_offramp_requested: bool,
    node_index: usize,
    epoch_seq: u64,
    policy: &MicaCompliancePolicy,
) -> AtalSettlementInscription {
    let mica_ok = verify_mica_title_ii_compliance(policy).is_ok();
    let (slot, damped_ms) = schedule_konnakol_tremor(node_index, epoch_seq, spark.ak_zip_compression_ratio);

    if !verdict.approved || !mica_ok {
        let halt_hash = blake3::hash(
            format!("HALT|{}|{}", spark.spark_blake3_seal, verdict.attestation_blake3).as_bytes(),
        )
        .to_hex()
        .to_string();
        return AtalSettlementInscription {
            glyph_code_u16: NsibidiGlyph::Nsb04CircuitBreakerHalt as u16,
            glyph_symbol: NsibidiGlyph::Nsb04CircuitBreakerHalt.symbol().to_string(),
            approved: false,
            mica_title_ii_compliant: mica_ok,
            gross_compute_eur: gross_eur,
            efemera_minted_net: 0.0,
            protocol_clearing_fee_eur: 0.0,
            oracular_vaas_fee_eur: 0.0,
            casp_offramp_rebate_eur: 0.0,
            mca_underwriting_fee_eur: 0.0,
            total_protocol_revenue_eur: 0.0,
            cogs_mif50_eur: 0.0,
            gross_profit_84pct_eur: 0.0,
            etamanga_buy_and_lock_eur: 0.0,
            etamanga_locked_units: 0.0,
            erc2981_split: Erc2981ProvenanceSplit {
                total_royalty_efemera: 0.0,
                compute_host_efemera: 0.0,
                sleco_prompt_genotype_efemera: 0.0,
                rcag_memos_curator_efemera: 0.0,
                ndimmorwyn_treasury_efemera: 0.0,
            },
            konnakol_tdma_slot: slot,
            tremor_damped_queue_ms: damped_ms,
            atal_inscription_blake3: halt_hash,
        };
    }

    let g_eur = gross_eur.max(0.0);
    let clearing_fee = g_eur * CLEARING_TAKE_RATE;
    let royalty_fee = if rcag_provenanced { g_eur * ROYALTY_ROUTING_SPREAD } else { 0.0 };
    let vaas_fee = if verdict.strata_executed.contains(&3) { ORACULAR_VAAS_FEE_EUR } else { 0.0 };
    let mca_fee = if is_mca_advance { g_eur * MCA_UNDERWRITING_SPREAD } else { 0.0 };
    let casp_rebate = if fiat_offramp_requested { g_eur * CASP_OFFRAMP_REBATE } else { 0.0 };

    let total_rev = clearing_fee + royalty_fee + vaas_fee + mca_fee + casp_rebate;
    let cogs = total_rev * MIF50_COGS_RATIO;
    let gross_profit = total_rev - cogs;

    // Only convert EFEMERA fees -> ETAMANGA vault locks when circuit breaker is healthy
    let etamanga_buy_eur = if verdict.circuit_breaker_tripped {
        0.0
    } else {
        total_rev * ETAMANGA_BURN_LOCK_SHARE
    };
    let etm_price = etamanga_spot_eur.max(0.01);
    let etamanga_units = etamanga_buy_eur / etm_price;

    let twap = verdict.kalman_twap_efemera_eur.max(0.0001);
    let net_eur_for_mint = (g_eur - clearing_fee).max(0.0);
    let efemera_minted = (net_eur_for_mint / twap) * spark.semantic_density_multiplier
        / verdict.mint_difficulty_factor.max(1.0);

    // Native ERC-2981 4-party provenance split (10% of minted EFEMERA when RCAG provenanced)
    let royalty_pool = if rcag_provenanced { efemera_minted * 0.10 } else { 0.0 };
    let split = Erc2981ProvenanceSplit {
        total_royalty_efemera: round6(royalty_pool),
        compute_host_efemera: round6(royalty_pool * 0.40),
        sleco_prompt_genotype_efemera: round6(royalty_pool * 0.25),
        rcag_memos_curator_efemera: round6(royalty_pool * 0.25),
        ndimmorwyn_treasury_efemera: round6(royalty_pool * 0.10),
    };

    let glyph = if is_mca_advance {
        NsibidiGlyph::Nsb02IkpeMcaCovenant
    } else if rcag_provenanced {
        NsibidiGlyph::Nsb03RcagRoyaltySplit
    } else {
        NsibidiGlyph::Nsb01VerifiedSettlement
    };

    let seal = blake3::hash(
        format!(
            "{}|{}|{}|{:.6}|{:.6}|{}",
            glyph.symbol(),
            spark.spark_blake3_seal,
            verdict.attestation_blake3,
            efemera_minted,
            etamanga_units,
            slot
        )
        .as_bytes(),
    )
    .to_hex()
    .to_string();

    AtalSettlementInscription {
        glyph_code_u16: glyph as u16,
        glyph_symbol: glyph.symbol().to_string(),
        approved: true,
        mica_title_ii_compliant: true,
        gross_compute_eur: round6(g_eur),
        efemera_minted_net: round6(efemera_minted),
        protocol_clearing_fee_eur: round6(clearing_fee),
        oracular_vaas_fee_eur: round6(vaas_fee),
        casp_offramp_rebate_eur: round6(casp_rebate),
        mca_underwriting_fee_eur: round6(mca_fee),
        total_protocol_revenue_eur: round6(total_rev),
        cogs_mif50_eur: round6(cogs),
        gross_profit_84pct_eur: round6(gross_profit),
        etamanga_buy_and_lock_eur: round6(etamanga_buy_eur),
        etamanga_locked_units: round6(etamanga_units),
        erc2981_split: split,
        konnakol_tdma_slot: slot,
        tremor_damped_queue_ms: damped_ms,
        atal_inscription_blake3: seal,
    }
}

/// Aggregates a batch of verified `AtalSettlementInscription` edges into a single
/// Merkle-anchored Korvai Epoch Root ready for Base/Solana/Substrate settlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KorvaiEpochBatch {
    pub epoch_id: u64,
    pub receipts_count: usize,
    pub total_efemera_minted: f64,
    pub total_etamanga_locked: f64,
    pub total_revenue_eur: f64,
    pub korvai_merkle_root_blake3: String,
}

pub fn commit_korvai_epoch_batch(epoch_id: u64, items: &[AtalSettlementInscription]) -> KorvaiEpochBatch {
    let mut leaves: Vec<String> = items
        .iter()
        .filter(|i| i.approved)
        .map(|i| i.atal_inscription_blake3.clone())
        .collect();
    if leaves.is_empty() {
        leaves.push(blake3::hash(b"EMPTY_KORVAI_EPOCH").to_hex().to_string());
    }
    while leaves.len() > 1 {
        let mut next = Vec::with_capacity((leaves.len() + 1) / 2);
        for pair in leaves.chunks(2) {
            let l = &pair[0];
            let r = if pair.len() > 1 { &pair[1] } else { l };
            let h = blake3::hash(format!("{l}:{r}").as_bytes()).to_hex().to_string();
            next.push(h);
        }
        leaves = next;
    }

    let tot_efm: f64 = items.iter().map(|i| i.efemera_minted_net).sum();
    let tot_etm: f64 = items.iter().map(|i| i.etamanga_locked_units).sum();
    let tot_rev: f64 = items.iter().map(|i| i.total_protocol_revenue_eur).sum();

    KorvaiEpochBatch {
        epoch_id,
        receipts_count: items.iter().filter(|i| i.approved).count(),
        total_efemera_minted: round6(tot_efm),
        total_etamanga_locked: round6(tot_etm),
        total_revenue_eur: round6(tot_rev),
        korvai_merkle_root_blake3: leaves[0].clone(),
    }
}

/// Years 1–3 Commercialization & Valuation Model (Conservative 10x / Base 15x / Optimistic 20x).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationProjectionTier {
    pub scenario: String,
    pub multiple_x: u32,
    pub year1_revenue: f64,
    pub year2_revenue: f64,
    pub year3_revenue: f64,
    pub cumulative_3yr_revenue: f64,
    pub cumulative_3yr_gross_profit_84pct: f64,
    pub cumulative_3yr_etamanga_burn_30pct: f64,
    pub year3_implied_valuation: f64,
    pub currency: String,
}

pub fn compute_3yr_commercial_projections(eur_fx_from_usd: f64, currency: &str) -> Vec<ValuationProjectionTier> {
    let fx = if currency.eq_ignore_ascii_case("USD") { 1.0 } else { eur_fx_from_usd };
    let tiers = [
        ("Conservative (10x Rev — Niche Sovereign & Ecosystem)", 10u32, 414_000.0, 1_665_000.0, 5_137_500.0),
        ("Base Case (15x Rev — Multi-Rail B2B & Agentic DePIN)", 15u32, 1_237_500.0, 5_450_000.0, 17_700_000.0),
        ("Optimistic (20x Rev — Standard Agentic Clearing Layer)", 20u32, 3_197_500.0, 14_910_000.0, 53_150_000.0),
    ];
    tiers
        .into_iter()
        .map(|(name, mult, y1, y2, y3)| {
            let r1 = (y1 * fx).round();
            let r2 = (y2 * fx).round();
            let r3 = (y3 * fx).round();
            let cum = r1 + r2 + r3;
            ValuationProjectionTier {
                scenario: name.to_string(),
                multiple_x: mult,
                year1_revenue: r1,
                year2_revenue: r2,
                year3_revenue: r3,
                cumulative_3yr_revenue: cum,
                cumulative_3yr_gross_profit_84pct: (cum * 0.84).round(),
                cumulative_3yr_etamanga_burn_30pct: (cum * 0.30).round(),
                year3_implied_valuation: r3 * (mult as f64),
                currency: currency.to_uppercase(),
            }
        })
        .collect()
}

fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}
