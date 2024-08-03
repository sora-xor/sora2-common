alloy_core::sol! {
    enum AssetType {
        Unregistered,
        Evm,
        Sora,
        Native
    }

    function unlock(
        address token,
        bytes32 sender,
        address recipient,
        uint256 amount
    ) external;

    function createNewToken(
        string memory name,
        string memory symbol,
        bytes32 sidechainAssetId
    ) external;

    function addTokenToWhitelist(
        address token,
        AssetType assetType
    ) external;

    function removeTokenFromWhitelist(
        address token
    ) external;

    function receivePayment() external payable;

    function prepeareForMigration() external;

}
