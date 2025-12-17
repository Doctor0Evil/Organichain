// contracts/SealedRefactorBridge.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IAuEtCspToken {
    function mintEnergy(address to, uint256 auEt, uint256 csp) external;
}

contract SealedRefactorBridge {
    struct SnapshotTuple {
        bytes32 sourceChainId;
        uint64 height;
        bytes32 denomHash;
        bytes32 accountHash;
        uint256 balance; // Bi
    }

    IAuEtCspToken public immutable energyToken;
    mapping(bytes32 => bool) public consumed; // replay key

    event Refactored(
        bytes32 indexed replayKey,
        address indexed recipient,
        uint256 auEt,
        uint256 csp
    );

    constructor(address _energyToken) {
        energyToken = IAuEtCspToken(_energyToken);
    }

    function computeSnapshotHash(SnapshotTuple memory s)
        public
        pure
        returns (bytes32)
    {
        return
            keccak256(
                abi.encodePacked(
                    s.sourceChainId,
                    s.height,
                    s.denomHash,
                    s.accountHash,
                    s.balance
                )
            );
    }

    function computeReplayKey(
        bytes32 sourceChainId,
        bytes32 originTxHash,
        uint64 nonce
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(sourceChainId, originTxHash, nonce));
    }

    function _mapToEnergy(
        uint256 balance,
        uint8 dSrc,
        uint8 dAln,
        uint256 cE_num,
        uint256 cE_den,
        uint256 cS_num,
        uint256 cS_den
    ) internal pure returns (uint256 auEt, uint256 csp) {
        // Asrc = balance * 10^-dSrc
        // AE = Asrc * (cE_num / cE_den)
        // BE = floor(AE * 10^dAln)
        uint256 factorSrc = 10 ** dSrc;
        uint256 factorAln = 10 ** dAln;
        // scale in integers: BE = floor(balance * cE_num * 10^(dAln-dSrc) / cE_den)
        uint256 scale = factorAln / factorSrc;
        auEt = (balance * cE_num * scale) / cE_den;
        csp = (balance * cS_num * scale) / cS_den;
    }

    function sealedRefactor(
        SnapshotTuple calldata snap,
        bytes32 expectedSnapHash,
        bytes32 originTxHash,
        uint64 originNonce,
        address recipient,
        uint8 dSrc,
        uint8 dAln,
        uint256 cE_num,
        uint256 cE_den,
        uint256 cS_num,
        uint256 cS_den
    ) external {
        // 1. verify snapshot hash (off‑chain light‑client proof must gate this entry in practice)
        bytes32 h = computeSnapshotHash(snap);
        require(h == expectedSnapHash, "snapshot mismatch");

        // 2. derive replay key
        bytes32 replayKey = computeReplayKey(
            snap.sourceChainId,
            originTxHash,
            originNonce
        );
        require(!consumed[replayKey], "replay");

        // 3. map balance → AU.ET/CSP
        (uint256 auEt, uint256 csp) = _mapToEnergy(
            snap.balance,
            dSrc,
            dAln,
            cE_num,
            cE_den,
            cS_num,
            cS_den
        );
        require(auEt > 0 || csp > 0, "zero energy");

        // 4. mark consumed & mint non-transferable energy
        consumed[replayKey] = true;
        energyToken.mintEnergy(recipient, auEt, csp);

        emit Refactored(replayKey, recipient, auEt, csp);
    }
}
