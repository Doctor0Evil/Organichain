// contracts/AuEtCspToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract AuEtCspToken is ERC20, AccessControl, IAuEtCspToken {
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");

    uint8 private immutable _decimals;
    uint256 public immutable maxTotalSupply;

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint256 maxTotalSupply_,
        address admin
    ) ERC20(name_, symbol_) {
        _decimals = decimals_;
        maxTotalSupply = maxTotalSupply_;
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function mintEnergy(
        address to,
        uint256 auEt,
        uint256 csp
    ) external onlyRole(BRIDGE_ROLE) {
        uint256 amount = auEt + csp; // internal 1:1 mapping to ERC20 units
        require(
            totalSupply() + amount <= maxTotalSupply,
            "cap exceeded"
        );
        _mint(to, amount);
    }

    // --- non-transferable semantics: block transfers and approvals ---

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override {
        super._beforeTokenTransfer(from, to, amount);
        if (from != address(0) && to != address(0)) {
            revert("non-transferable energy");
        }
    }

    function approve(address, uint256) public pure override returns (bool) {
        revert("approvals disabled");
    }

    function transfer(address, uint256) public pure override returns (bool) {
        revert("non-transferable");
    }

    function transferFrom(
        address,
        address,
        uint256
    ) public pure override returns (bool) {
        revert("non-transferable");
    }
}
