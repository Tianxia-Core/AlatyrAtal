// ffp: FFP-ALATYRATAL-CLI-MCP concurrent-daemon rust
//! `alatyratal` v1.0.0 — Sovereign CLI & Model Context Protocol (MCP) Server for
//! `AlatyrAtal` (Unit of `Ndimmôrwyn`).

use alatyratal::{
    commit_korvai_epoch_batch, compute_3yr_commercial_projections, evaluate_omni_lattice,
    inscribe_atal_settlement, meter_alatyr_spark, verify_mica_title_ii_compliance,
    AlatyrSparkInput, EbhuNjomRouteTier, MicaCompliancePolicy, OracularInput, Stratum0Context,
};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

const VERSION: &str = "1.0.0";

fn print_usage() {
    println!(
        "alatyratal v{VERSION} (Apache-2.0) — Unit of Ndimmôrwyn · RASA-X-MACHINA · tianxiaOS
Proof-of-Quality-Adjusted-Inference Clearinghouse & Dual-Token (EFEMERA -> ETAMANGA) Engine

USAGE:
  alatyratal clear [--eur 0.25] [--raw-tokens 500] [--compressed 2] [--cr 1.42]
  alatyratal underwrite-mca [--advance-eur 25000] [--cr 1.38]      # Use Case 1: Ndimmôrwyn + MURUA MCA
  alatyratal x402-swarm [--hops 8] [--eur-per-hop 0.04]            # Use Case 2: Zero-KYC HTTP x402 Swarm
  alatyratal royalty-split [--gcv-eur 120.0]                       # Use Case 3: RCAG + ERC-2981 Royalties
  alatyratal mica-audit [--reasonwash-test]                        # Use Case 4: EU AI Act & MiCA Firewall
  alatyratal edge-konnakol [--nodes 5] [--spike 50.0]              # Use Case 5: RASOOM-NKG + Konnakol [3,2]
  alatyratal economics [--currency EUR|USD]                        # Years 1-3 Commercialization & Burn Model
  alatyratal init-clean [target-dir]                               # Bootstrap zero-PII Clean-Room seed
  alatyratal mcp-stdio                                             # Start JSON-RPC 2.0 MCP Server ($0.00)"
    );
}

fn flag_f64(args: &[String], name: &str, default: f64) -> f64 {
    args.windows(2)
        .find(|w| w[0] == name)
        .and_then(|w| w[1].parse::<f64>().ok())
        .unwrap_or(default)
}

fn flag_u64(args: &[String], name: &str, default: u64) -> u64 {
    args.windows(2)
        .find(|w| w[0] == name)
        .and_then(|w| w[1].parse::<u64>().ok())
        .unwrap_or(default)
}

fn flag_str(args: &[String], name: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].clone())
        .unwrap_or_else(|| default.to_string())
}

fn execute_full_pipeline(
    req_id: &str,
    eur: f64,
    raw_tok: u64,
    comp_tok: u64,
    cr: f64,
    is_mca: bool,
    rcag: bool,
    offramp: bool,
    cited_lp: f64,
    ablated_lp: f64,
) -> Value {
    let spark = meter_alatyr_spark(&AlatyrSparkInput {
        request_id: req_id.to_string(),
        model_id: "bonsai-bitnet-b1.58-sovereign".to_string(),
        raw_input_tokens: raw_tok / 2,
        raw_output_tokens: raw_tok - (raw_tok / 2),
        ak_zip_compressed_tokens: comp_tok,
        mutual_information_bits: 18.5,
        compute_millijoules: 42.0,
        latency_ms: if eur < 0.50 && !is_mca { 11.4 } else { 64.0 },
        eur_compute_notional: eur,
        is_mca_or_vault_unlock: is_mca,
    });

    let verdict = evaluate_omni_lattice(&OracularInput {
        stratum0: Stratum0Context {
            collateralization_ratio_cr: cr,
            ..Stratum0Context::default()
        },
        spot_efemera_eur: 0.125,
        kalman_prior_eur: 0.122,
        sybil_anomaly_score: 0.04,
        prompt_text: format!("Verified sovereign execution {req_id}"),
        activation_trajectory: vec![0.31, -0.14, 0.52, 0.19, -0.22, 0.44],
        safe_reference_trajectory: vec![0.30, -0.15, 0.50, 0.18, -0.21, 0.45],
        cited_logprob: cited_lp,
        ablated_logprob: ablated_lp,
        route_tier: spark.route_tier,
    });

    let policy = MicaCompliancePolicy::default();
    let inscription = inscribe_atal_settlement(
        &spark, &verdict, eur, 2.40, rcag, is_mca, offramp, 0, 1, &policy,
    );

    json!({
        "engine": "AlatyrAtal",
        "version": VERSION,
        "parent_organism": "Ndimmorwyn",
        "alatyr_spark": spark,
        "oracular_verdict": verdict,
        "atal_inscription": inscription,
    })
}

fn run_mcp_stdio() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(trimmed) else {
            continue;
        };
        let Some(id) = req.get("id").cloned() else {
            continue;
        };
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let result = match method {
            "initialize" => json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "alatyratal", "version": VERSION }
            }),
            "tools/list" => json!({
                "tools": [
                    {
                        "name": "alatyratal_clear_inference",
                        "description": "Clear an LLM compute or MCA event through Alatyr (AK-Zip/EGO), 4-Stratum Omni-Lattice Oracle, and Atal (EFEMERA->ETAMANGA BME + MiCA Title II).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "eur": { "type": "number" },
                                "raw_tokens": { "type": "integer" },
                                "compressed_tokens": { "type": "integer" },
                                "cr": { "type": "number" }
                            }
                        }
                    },
                    {
                        "name": "alatyratal_economics",
                        "description": "Return Years 1-3 Conservative/Base/Optimistic commercialization & 30% ETAMANGA burn projections.",
                        "inputSchema": {
                            "type": "object",
                            "properties": { "currency": { "type": "string" } }
                        }
                    }
                ]
            }),
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(json!({}));
                let tname = params.get("name").and_then(Value::as_str).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                let payload = if tname == "alatyratal_economics" {
                    let cur = args.get("currency").and_then(Value::as_str).unwrap_or("EUR");
                    json!(compute_3yr_commercial_projections(0.92, cur))
                } else {
                    let eur = args.get("eur").and_then(Value::as_f64).unwrap_or(0.25);
                    let raw = args.get("raw_tokens").and_then(Value::as_u64).unwrap_or(500);
                    let comp = args.get("compressed_tokens").and_then(Value::as_u64).unwrap_or(2);
                    let cr = args.get("cr").and_then(Value::as_f64).unwrap_or(1.42);
                    execute_full_pipeline("mcp-clear-01", eur, raw, comp, cr, false, true, true, -0.18, -2.45)
                };
                json!({ "content": [{ "type": "text", "text": serde_json::to_string_pretty(&payload).unwrap_or_default() }] })
            }
            _ => json!({}),
        };
        println!("{}", json!({ "jsonrpc": "2.0", "id": id, "result": result }));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--version" | "-V" | "version" => {
            println!("alatyratal {VERSION} (Apache-2.0 · Unit of Ndimmôrwyn)");
        }
        "--help" | "-h" | "help" => print_usage(),
        "clear" => {
            let eur = flag_f64(&args, "--eur", 0.25);
            let raw = flag_u64(&args, "--raw-tokens", 500);
            let comp = flag_u64(&args, "--compressed", 2);
            let cr = flag_f64(&args, "--cr", 1.42);
            let out = execute_full_pipeline("alatyr-clear-001", eur, raw, comp, cr, false, true, true, -0.15, -2.60);
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        "underwrite-mca" => {
            let adv = flag_f64(&args, "--advance-eur", 25_000.0);
            let cr = flag_f64(&args, "--cr", 1.38);
            let out = execute_full_pipeline("mca-murua-ejawen-001", adv, 2400, 16, cr, true, true, true, -0.12, -2.85);
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        "x402-swarm" => {
            let hops = flag_u64(&args, "--hops", 8) as usize;
            let eur_hop = flag_f64(&args, "--eur-per-hop", 0.04);
            let policy = MicaCompliancePolicy::default();
            let mut inscriptions = Vec::with_capacity(hops);
            for h in 0..hops {
                let spark = meter_alatyr_spark(&AlatyrSparkInput {
                    request_id: format!("x402-kamadhenu-hop-{h}"),
                    model_id: "bitnet-b1.58-avx2".to_string(),
                    raw_input_tokens: 250,
                    raw_output_tokens: 250,
                    ak_zip_compressed_tokens: 2,
                    mutual_information_bits: 16.0,
                    compute_millijoules: 28.0,
                    latency_ms: 9.8,
                    eur_compute_notional: eur_hop,
                    is_mca_or_vault_unlock: false,
                });
                let verdict = evaluate_omni_lattice(&OracularInput {
                    stratum0: Stratum0Context::default(),
                    spot_efemera_eur: 0.125,
                    kalman_prior_eur: 0.124,
                    sybil_anomaly_score: 0.02,
                    prompt_text: "x402 verified agentic hop".to_string(),
                    activation_trajectory: vec![0.2, -0.1, 0.4, 0.1],
                    safe_reference_trajectory: vec![0.2, -0.1, 0.4, 0.1],
                    cited_logprob: -0.2,
                    ablated_logprob: -2.1,
                    route_tier: EbhuNjomRouteTier::OptimisticFastLane14ms,
                });
                inscriptions.push(inscribe_atal_settlement(
                    &spark, &verdict, eur_hop, 2.40, true, false, false, h, h as u64, &policy,
                ));
            }
            let batch = commit_korvai_epoch_batch(1, &inscriptions);
            println!("{}", serde_json::to_string_pretty(&batch).unwrap());
        }
        "royalty-split" => {
            let gcv = flag_f64(&args, "--gcv-eur", 120.0);
            let out = execute_full_pipeline("rcag-erc2981-001", gcv, 1200, 12, 1.45, false, true, false, -0.14, -2.50);
            println!("{}", serde_json::to_string_pretty(&out["atal_inscription"]["erc2981_split"]).unwrap());
        }
        "mica-audit" => {
            let rw_test = args.iter().any(|a| a == "--reasonwash-test");
            let policy = MicaCompliancePolicy::default();
            let mica_res = verify_mica_title_ii_compliance(&policy);
            let ablated = if rw_test { -0.20 } else { -2.75 };
            let out = execute_full_pipeline("eu-ai-act-mica-001", 5.0, 800, 8, 1.40, false, true, true, -0.18, ablated);
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "mica_title_ii_check": mica_res.is_ok(),
                    "reasonwash_simulation": rw_test,
                    "audit_result": out
                }))
                .unwrap()
            );
        }
        "edge-konnakol" => {
            let nodes = flag_u64(&args, "--nodes", 5) as usize;
            let spike = flag_f64(&args, "--spike", 50.0);
            let mut schedule = Vec::with_capacity(nodes);
            for n in 0..nodes {
                let (slot, damped_ms) = alatyratal::schedule_konnakol_tremor(n, n as u64 + 1, spike);
                schedule.push(json!({
                    "mesh_node": format!("10.77.0.{}", n + 1),
                    "konnakol_tdma_slot": slot,
                    "traffic_spike_x": spike,
                    "tremor_damped_latency_ms": damped_ms,
                    "nsibidi_glyph": "⟦NSB:0x01⟧"
                }));
            }
            println!("{}", serde_json::to_string_pretty(&schedule).unwrap());
        }
        "economics" => {
            let cur = flag_str(&args, "--currency", "EUR");
            let proj = compute_3yr_commercial_projections(0.92, &cur);
            println!("{}", serde_json::to_string_pretty(&proj).unwrap());
        }
        "init-clean" | "init" => {
            let dir = if args.len() > 2 && !args[2].starts_with('-') {
                PathBuf::from(&args[2])
            } else {
                PathBuf::from("./alatyratal-cleanroom")
            };
            fs::create_dir_all(dir.join("schemas")).unwrap_or_else(|e| {
                eprintln!("mkdir error: {e}");
                process::exit(1);
            });
            let ddl = "-- ffp: FFP-ALATYRATAL-CLEAN-SEED wire-codec sql\nPRAGMA journal_mode = WAL;\nCREATE TABLE IF NOT EXISTS atal_korvai_epochs (\n  epoch_id INTEGER PRIMARY KEY,\n  receipts_count INTEGER NOT NULL,\n  efemera_minted REAL NOT NULL,\n  etamanga_locked REAL NOT NULL,\n  korvai_merkle_root TEXT NOT NULL\n);\n";
            fs::write(dir.join("schemas/alatyratal_seed.sql"), ddl).unwrap();
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "ok": true,
                    "edition": "PUBLIC_CLEANROOM_SEED",
                    "initialized_dir": dir,
                    "zero_pii": true
                }))
                .unwrap()
            );
        }
        "mcp-stdio" | "mcp" => run_mcp_stdio(),
        other => {
            eprintln!("Unknown subcommand '{other}'");
            print_usage();
            process::exit(2);
        }
    }
}
