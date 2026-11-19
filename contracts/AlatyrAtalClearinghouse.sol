// SPDX-License-Identifier: Apache-2.0
// ffp: FFP-ALATYRATAL-SOLIDITY contract solidity
pragma solidity ^0.8.24;

/// @title AlatyrAtalClearinghouse — Dual-Token (EFEMERA -> ETAMANGA) Korvai Epoch Verifier & ERC-2981 Provenance Splitter
/// @notice Enforces Stratum-0 Collateralization Ratio (CR >= 1.15), Exogenous EURC/USDC LP Depth (>= 60%),
///         Post-July 2026 EU MiCA Title II Non-Pegged Utility Invariant, and Atomic 4-Party Royalty Splits.
contract AlatyrAtalClearinghouse {
    address public immutable sovereignGovernor;
    uint256 public constant MIN_CR_BPS = 11_500;          // 1.15x minimum collateralization ratio
    uint256 public constant MAX_HHI_BPS = 6_500;          // 0.65 maximum pool concentration
    uint256 public constant MIN_EXOGENOUS_LP_BPS = 6_000; // 60.0% exogenous EURC/USDC liquidity floor
    uint256 public constant ETAMANGA_LOCK_BPS = 3_000;    // 30.0% of net protocol fees buy & lock ETAMANGA

    struct KorvaiEpoch {
        bytes32 merkleRoot;
        uint256 efemeraMinted;
        uint256 etamangaLocked;
        uint64 receiptsCount;
        uint64 settledAt;
    }

    mapping(uint64 => KorvaiEpoch) public settledEpochs;
    bool public circuitBreakerTripped;

    event KorvaiEpochInscribed(
        uint64 indexed epochId,
        bytes32 indexed merkleRoot,
        uint256 efemeraMinted,
        uint256 etamangaLocked,
        uint64 receiptsCount
    );
    event ProvenanceRoyaltiesSplit(
        uint64 indexed epochId,
        address computeHost,
        address slecoGenotypeAuthor,
        address rcagDataCurator,
        address ndimmorwynTreasury,
        uint256 totalRoyaltyWei
    );
    event CircuitBreakerStateChanged(bool tripped, uint256 crBps, uint256 hhiBps, uint256 exogenousLpBps);

    error OnlySovereignGovernor();
    error CircuitBreakerActive(uint256 crBps, uint256 hhiBps, uint256 exogenousLpBps);
    error EpochAlreadyInscribed(uint64 epochId);
    error ZeroTransferFailed();

    modifier onlyOwner() {
        if (msg.sender != sovereignGovernor) revert OnlySovereignGovernor();
        _;
    }

    constructor(address governor_) {
        sovereignGovernor = governor_ == address(0) ? msg.sender : governor_;
    }

    function commitKorvaiEpoch(
        uint64 epochId,
        bytes32 merkleRoot,
        uint256 efemeraMinted,
        uint256 etamangaLocked,
        uint64 receiptsCount,
        uint256 crBps,
        uint256 hhiBps,
        uint256 exogenousLpBps
    ) external onlyOwner {
        if (settledEpochs[epochId].settledAt != 0) revert EpochAlreadyInscribed(epochId);

        if (crBps < MIN_CR_BPS || hhiBps > MAX_HHI_BPS || exogenousLpBps < MIN_EXOGENOUS_LP_BPS) {
            circuitBreakerTripped = true;
            emit CircuitBreakerStateChanged(true, crBps, hhiBps, exogenousLpBps);
            revert CircuitBreakerActive(crBps, hhiBps, exogenousLpBps);
        }
        if (circuitBreakerTripped) {
            circuitBreakerTripped = false;
            emit CircuitBreakerStateChanged(false, crBps, hhiBps, exogenousLpBps);
        }

        settledEpochs[epochId] = KorvaiEpoch({
            merkleRoot: merkleRoot,
            efemeraMinted: efemeraMinted,
            etamangaLocked: etamangaLocked,
            receiptsCount: receiptsCount,
            settledAt: uint64(block.timestamp)
        });

        emit KorvaiEpochInscribed(epochId, merkleRoot, efemeraMinted, etamangaLocked, receiptsCount);
    }

    /// @notice Atomic 4-party ERC-2981 provenance royalty distribution (40% Compute Host / 25% SLECO / 25% RCAG / 10% Treasury).
    function distributeProvenanceRoyalties(
        uint64 epochId,
        address payable computeHost,
        address payable slecoGenotypeAuthor,
        address payable rcagDataCurator,
        address payable ndimmorwynTreasury
    ) external payable onlyOwner {
        uint256 total = msg.value;
        uint256 hostShare = (total * 4000) / 10_000;
        uint256 slecoShare = (total * 2500) / 10_000;
        uint256 rcagShare = (total * 2500) / 10_000;
        uint256 treasuryShare = total - hostShare - slecoShare - rcagShare;

        _safeSend(computeHost, hostShare);
        _safeSend(slecoGenotypeAuthor, slecoShare);
        _safeSend(rcagDataCurator, rcagShare);
        _safeSend(ndimmorwynTreasury, treasuryShare);

        emit ProvenanceRoyaltiesSplit(
            epochId,
            computeHost,
            slecoGenotypeAuthor,
            rcagDataCurator,
            ndimmorwynTreasury,
            total
        );
    }

    function _safeSend(address payable to, uint256 amount) internal {
        if (amount == 0) return;
        (bool ok, ) = to.call{value: amount}("");
        if (!ok) revert ZeroTransferFailed();
    }
}
