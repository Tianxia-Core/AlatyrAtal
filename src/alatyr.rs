// ffp: FFP-ALATYR-SPARK compute-kernel rust
//! `Alatyr` Layer (Slavic Primordial Spark of Svarog's Hammer):
//! High-velocity LLM inference metering, Counterintuitive Inverse-Exhaust (`AK-Zip` + `EGO`)
//! Semantic Density Scoring, and `Ebhu Njom` `< €0.50` (`<= 14ms`) Latency-vs-Value Tiering.

use serde::{Deserialize, Serialize};

/// Input telemetry for an inference or agentic subtask entering the `Alatyr` spark layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlatyrSparkInput {
    pub request_id: String,
    pub model_id: String,
    pub raw_input_tokens: u64,
    pub raw_output_tokens: u64,
    pub ak_zip_compressed_tokens: u64,
    pub mutual_information_bits: f64,
    pub compute_millijoules: f64,
    pub latency_ms: f64,
    pub eur_compute_notional: f64,
    pub is_mca_or_vault_unlock: bool,
}

/// Routing tier determined at the `Ebhu Njom` threshold stone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EbhuNjomRouteTier {
    /// Micro-conversion (`< €0.50` EUR equivalent): Stratum 0 + Stratum 1 + Jivaslokan (`<= 14ms`)
    /// with deterministic BLAKE3 probabilistic spot-audit.
    OptimisticFastLane14ms,
    /// High-stakes MCA underwriting, ETAMANGA vault conversion, or `>= €0.50` batch:
    /// Full Stratum 0..3 Omni-Lattice + ARSI counterfactual attribution quorum.
    FullFourStratumArsiQuorum,
}

/// Output receipt of the `Alatyr` metering & counterintuitive `AK-Zip` + `EGO` density gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlatyrSparkReceipt {
    pub request_id: String,
    pub raw_total_tokens: u64,
    pub compressed_effective_tokens: u64,
    pub ak_zip_compression_ratio: f64,
    pub ego_mi_per_millijoule: f64,
    pub ego_pruned_bloat: bool,
    pub semantic_density_multiplier: f64,
    pub quality_adjusted_compute_units: f64,
    pub route_tier: EbhuNjomRouteTier,
    pub spot_audit_selected: bool,
    pub spark_blake3_seal: String,
}

const EGO_MIN_MI_PER_MJ: f64 = 0.05;
const EBHU_NJOM_MICRO_THRESHOLD_EUR: f64 = 0.50;

/// Evaluates raw inference telemetry and applies the Counterintuitive Inverse-Exhaust Law:
/// Verbose filler is penalized via EGO pruning, while compact `AK-Zip` semantic density
/// earns up to a `2.50x` quality-adjusted compute multiplier without inflating token supply.
pub fn meter_alatyr_spark(input: &AlatyrSparkInput) -> AlatyrSparkReceipt {
    let raw_total = input.raw_input_tokens.saturating_add(input.raw_output_tokens).max(1);
    let compressed = input.ak_zip_compressed_tokens.max(1).min(raw_total);
    let ak_ratio = raw_total as f64 / compressed as f64;

    let mj = input.compute_millijoules.max(1.0);
    let mi_per_mj = input.mutual_information_bits.max(0.0) / mj;
    let ego_pruned = mi_per_mj < EGO_MIN_MI_PER_MJ;

    // Counterintuitive Inversion: higher compression & higher MI/mJ earn higher multiplier,
    // whereas verbose low-information token bloat is penalized (`0.25x`).
    let density_mult = if ego_pruned {
        0.25
    } else {
        (1.0 + (ak_ratio.ln().max(0.0) * 0.28)).clamp(1.0, 2.50)
    };

    let base_eur = input.eur_compute_notional.max(0.0001);
    let qacu = (base_eur * 1000.0 * density_mult * 10_000.0).round() / 10_000.0;

    let seal_payload = format!(
        "{}|{}|{}|{}|{:.6}|{:.4}",
        input.request_id, input.model_id, raw_total, compressed, base_eur, input.latency_ms
    );
    let hash = blake3::hash(seal_payload.as_bytes());
    let seal_hex = hash.to_hex().to_string();

    // Deterministic 6.25% (1/16) spot-audit on micro-conversions via first nibble == '0'
    let spot_audit = seal_hex.starts_with('0');
    let route_tier = if input.is_mca_or_vault_unlock || base_eur >= EBHU_NJOM_MICRO_THRESHOLD_EUR {
        EbhuNjomRouteTier::FullFourStratumArsiQuorum
    } else {
        EbhuNjomRouteTier::OptimisticFastLane14ms
    };

    AlatyrSparkReceipt {
        request_id: input.request_id.clone(),
        raw_total_tokens: raw_total,
        compressed_effective_tokens: compressed,
        ak_zip_compression_ratio: (ak_ratio * 100.0).round() / 100.0,
        ego_mi_per_millijoule: (mi_per_mj * 10_000.0).round() / 10_000.0,
        ego_pruned_bloat: ego_pruned,
        semantic_density_multiplier: (density_mult * 1000.0).round() / 1000.0,
        quality_adjusted_compute_units: qacu,
        route_tier,
        spot_audit_selected: spot_audit,
        spark_blake3_seal: seal_hex,
    }
}
