use crate::querier::{query_balance, query_token_balance, query_token_symbol};
use apollo_proto_rust::cosmos::base::v1beta1::Coin as ProtoCoin;
use cosmwasm_std::{
    Addr, Api, Coin, CustomQuery, MessageInfo, QuerierWrapper, StdError, StdResult, Uint128,
};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, fmt, str::FromStr};

/// ## Description
/// This enum describes asset.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Asset {
    /// the available type of asset from [`AssetInfo`]
    pub info: AssetInfo,
    /// the amount of an asset
    pub amount: Uint128,
}

impl Asset {
    pub fn empty() -> Self {
        Asset {
            info: AssetInfo::empty(),
            amount: Uint128::zero(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.info.is_empty() && self.amount.is_zero()
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.info)
    }
}

impl From<Coin> for Asset {
    fn from(coin: Coin) -> Self {
        Self {
            info: AssetInfo::NativeToken { denom: coin.denom },
            amount: coin.amount,
        }
    }
}

impl TryInto<Coin> for Asset {
    type Error = StdError;

    fn try_into(self) -> StdResult<Coin> {
        match self.info {
            AssetInfo::Token { .. } => Err(StdError::generic_err(
                "Cannot convert an non-native token to Coin.",
            )),
            AssetInfo::NativeToken { denom } => Ok(Coin {
                denom,
                amount: self.amount,
            }),
        }
    }
}

impl From<ProtoCoin> for Asset {
    fn from(proto_coin: ProtoCoin) -> Self {
        Self {
            info: AssetInfo::NativeToken {
                denom: proto_coin.denom,
            },
            amount: Uint128::from_str(&proto_coin.amount).unwrap(),
        }
    }
}

impl TryInto<ProtoCoin> for Asset {
    type Error = StdError;

    fn try_into(self) -> StdResult<ProtoCoin> {
        match self.info {
            AssetInfo::Token { .. } => Err(StdError::generic_err(
                "Cannot convert an non-native token to ProtoCoin.",
            )),
            AssetInfo::NativeToken { denom } => Ok(ProtoCoin {
                denom,
                amount: self.amount.to_string(),
            }),
        }
    }
}

impl Asset {
    /// ## Description
    /// Returns true if token is native token. Otherwise returns false.
    /// ## Params
    /// * **self** is the type of the caller object.
    pub fn is_native_token(&self) -> bool {
        self.info.is_native_token()
    }

    /// ## Description
    /// Approves the amount of native tokens. Returns [`Ok`] if successful, otherwise returns [`Err`].
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **message_info** is the object of type [`MessageInfo`]
    pub fn assert_sent_native_token_balance(&self, message_info: &MessageInfo) -> StdResult<()> {
        if let AssetInfo::NativeToken { denom } = &self.info {
            match message_info.funds.iter().find(|x| x.denom == *denom) {
                Some(coin) => {
                    if self.amount == coin.amount {
                        Ok(())
                    } else {
                        Err(StdError::generic_err("Native token balance mismatch between the argument and the transferred"))
                    }
                }
                None => {
                    if self.amount.is_zero() {
                        Ok(())
                    } else {
                        Err(StdError::generic_err("Native token balance mismatch between the argument and the transferred"))
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    /// ## Description
    /// Implements Assets Into() trait
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **assets** is the object of type [`Asset`]
    pub fn array_into<T>(assets: [Asset; 2]) -> [T; 2]
    where
        Asset: Into<T>,
    {
        [assets[0].clone().into(), assets[1].clone().into()]
    }

    /// ## Description
    /// Implements Assets From() trait
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **assets** is the object of type [`Asset`]
    pub fn array_from<T>(assets: [T; 2]) -> [Asset; 2]
    where
        Asset: From<T>,
        T: Clone,
    {
        [
            Asset::from(assets[0].clone()),
            Asset::from(assets[1].clone()),
        ]
    }
}

/// ## Description
/// This enum describes available types of Token.
/// ## Examples
/// ```
/// # use cosmwasm_std::Addr;
/// # use apollo_asset::asset::AssetInfo::{NativeToken, Token};
/// Token { contract_addr: Addr::unchecked("terra...") };
/// NativeToken { denom: String::from("uluna") };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    /// Token
    Token {
        /// contract [`Addr`]
        contract_addr: Addr,
    },
    /// Native token
    NativeToken {
        /// denom [`String`]
        denom: String,
    },
}

impl AssetInfo {
    pub fn empty() -> Self {
        AssetInfo::NativeToken {
            denom: "".to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AssetInfo::NativeToken { denom } => denom.is_empty(),
            AssetInfo::Token { contract_addr } => contract_addr.as_str().is_empty(),
        }
    }
}

impl fmt::Display for AssetInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetInfo::NativeToken { denom } => write!(f, "{}", denom),
            AssetInfo::Token { contract_addr } => write!(f, "{}", contract_addr),
        }
    }
}

impl From<Addr> for AssetInfo {
    fn from(contract_addr: Addr) -> Self {
        Self::Token { contract_addr }
    }
}

impl From<Coin> for AssetInfo {
    fn from(coin: Coin) -> Self {
        Self::NativeToken { denom: coin.denom }
    }
}

impl From<ProtoCoin> for AssetInfo {
    fn from(proto_coin: ProtoCoin) -> Self {
        Self::NativeToken {
            denom: proto_coin.denom,
        }
    }
}

impl From<Asset> for AssetInfo {
    fn from(asset: Asset) -> Self {
        asset.info
    }
}

impl From<AssetInfo> for Option<Addr> {
    fn from(val: AssetInfo) -> Self {
        match val {
            AssetInfo::Token { contract_addr } => Some(contract_addr),
            AssetInfo::NativeToken { denom: _ } => None,
        }
    }
}

// impl Into<Option<Addr>> for AssetInfo {
//     fn into(self) -> Option<Addr> {
//         match self {
//             AssetInfo::Token { contract_addr } => Some(contract_addr),
//             AssetInfo::NativeToken { denom: _ } => None,
//         }
//     }
// }

impl TryInto<Addr> for AssetInfo {
    type Error = StdError;

    fn try_into(self) -> StdResult<Addr> {
        match self {
            AssetInfo::Token { contract_addr } => Ok(contract_addr),
            AssetInfo::NativeToken { denom: _ } => Err(StdError::generic_err(
                "Cannot convert an non-native token to Addr.",
            )),
        }
    }
}

/// Balance Map
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");

/// Implement [`AssetInfo`] functions
impl AssetInfo {
    /// Constructor
    pub fn new(api: &dyn Api, token: &str) -> Self {
        match api.addr_validate(token) {
            Ok(contract_addr) => AssetInfo::Token { contract_addr },
            Err(_) => AssetInfo::NativeToken {
                denom: token.to_string(),
            },
        }
    }

    /// ## Description
    /// Returns true if the caller is a native token. Otherwise returns false.
    /// ## Params
    /// * **self** is the type of the caller object
    pub fn is_native_token(&self) -> bool {
        match self {
            AssetInfo::NativeToken { .. } => true,
            AssetInfo::Token { .. } => false,
        }
    }

    /// ## Description
    /// Returns balance of token in a pool.
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **addr** is the address for which the balance is requested.
    pub fn query_balance<C: CustomQuery>(
        &self,
        querier: &QuerierWrapper<C>,
        addr: Addr,
    ) -> StdResult<Uint128> {
        match self {
            AssetInfo::Token { contract_addr, .. } => {
                // TODO - look into why raw query is inconsistent
                match BALANCES.query(querier, contract_addr.clone(), &addr)? {
                    None => query_token_balance(querier, contract_addr.clone(), addr),
                    Some(balance) => Ok(balance),
                }
            }
            AssetInfo::NativeToken { denom, .. } => query_balance(querier, addr, denom.to_string()),
        }
    }

    /// ## Description
    /// Returns True if the calling token is equal to the token specified in the input parameters.
    /// Otherwise returns False.
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **asset** is object of type [`AssetInfo`].
    pub fn equal(&self, asset: &AssetInfo) -> bool {
        match self {
            AssetInfo::Token { contract_addr, .. } => {
                let self_contract_addr = contract_addr;
                match asset {
                    AssetInfo::Token { contract_addr, .. } => self_contract_addr == contract_addr,
                    AssetInfo::NativeToken { .. } => false,
                }
            }
            AssetInfo::NativeToken { denom, .. } => {
                let self_denom = denom;
                match asset {
                    AssetInfo::Token { .. } => false,
                    AssetInfo::NativeToken { denom, .. } => self_denom == denom,
                }
            }
        }
    }

    /// ## Description
    /// If caller object is a native token of type ['AssetInfo`] then his `denom` field convert to a byte string.
    ///
    /// If caller object is a token of type ['AssetInfo`] then his `contract_addr` field convert to a byte string.
    /// ## Params
    /// * **self** is the type of the caller object.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            AssetInfo::NativeToken { denom } => denom.as_bytes(),
            AssetInfo::Token { contract_addr } => contract_addr.as_bytes(),
        }
    }

    /// ## Description
    /// Returns [`Ok`] if the token of type [`AssetInfo`] is in lowercase and valid. Otherwise returns [`Err`].
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **api** is a object of type [`Api`]
    pub fn check(&self, api: &dyn Api) -> StdResult<()> {
        match self {
            AssetInfo::Token { contract_addr } => {
                addr_validate_to_lower(api, contract_addr.as_str())?;
            }
            AssetInfo::NativeToken { denom } => {
                if denom != &denom.to_lowercase() {
                    return Err(StdError::generic_err(format!(
                        "Native token denom {} should be lowercase",
                        denom
                    )));
                }
            }
        }
        Ok(())
    }

    /// ## Description
    /// Convert AssetInfo into an Addr.
    /// ## Params
    /// * **self** is the type of the caller object.
    pub fn to_addr(self) -> StdResult<Addr> {
        match self {
            AssetInfo::Token { contract_addr } => Ok(contract_addr),
            AssetInfo::NativeToken { .. } => Err(StdError::generic_err(
                "Cannot convert a AssetInfo::NativeToken to an Addr",
            )),
        }
    }

    /// ## Description
    /// Implement From().
    /// TODO: This function needs to be refactored
    /// ## Params
    /// * **self** is the type of the caller object.
    pub fn from_str(api: &dyn Api, str: &str) -> StdResult<Self> {
        match api.addr_validate(str) {
            Ok(contract_addr) => Ok(Self::Token { contract_addr }),
            //TODO: Find a better way to find a NativeToken, this could include also failed str cases
            Err(_) => Ok(Self::NativeToken {
                denom: str.to_string(),
            }),
        }
    }

    /// ## Description
    /// Convert an amount into [`Coin`].
    /// ## Params
    /// * **self** is the type of the caller object.
    /// * **amount** token quantity
    pub fn to_coin(self, amount: Uint128) -> StdResult<Coin> {
        match self {
            AssetInfo::Token { .. } => Err(StdError::generic_err(
                "Cannot convert an non-native token to Coin.",
            )),
            AssetInfo::NativeToken { denom } => Ok(Coin { denom, amount }),
        }
    }

    /// ## Description
    /// Convert an amount into [`ProtoCoin`].
    /// ## Params
    /// * **self** is the type of the caller object.
    /// * **amount** token quantity
    pub fn to_proto_coin(self, amount: Uint128) -> StdResult<ProtoCoin> {
        match self {
            AssetInfo::Token { .. } => Err(StdError::generic_err(
                "Cannot convert an non-native token to ProtoCoin.",
            )),
            AssetInfo::NativeToken { denom } => Ok(ProtoCoin {
                denom,
                amount: amount.to_string(),
            }),
        }
    }

    /// ## Description
    /// Convert an amount into [`Asset`].
    /// ## Params
    /// * **self** is the type of the caller object.
    /// * **amount** token quantity
    pub fn to_asset(self, amount: Uint128) -> Asset {
        Asset { info: self, amount }
    }

    /// ## Description
    /// Implements Assets Into() trait
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **assets** is the object of type [`Asset`]
    /// TODO: Repeated function
    pub fn array_into<T>(assets: [AssetInfo; 2]) -> [T; 2]
    where
        AssetInfo: Into<T>,
    {
        [assets[0].clone().into(), assets[1].clone().into()]
    }
}

/// ## Description
/// Returns the validated address in lowercase on success. Otherwise returns [`Err`]
/// ## Params
/// * **api** is a object of type [`Api`]
///
/// * **addr** is the object of type [`Addr`]
pub fn addr_validate_to_lower(api: &dyn Api, addr: &str) -> StdResult<Addr> {
    if addr.to_lowercase() != addr {
        return Err(StdError::generic_err(format!(
            "Address {} should be lowercase",
            addr
        )));
    }
    api.addr_validate(addr)
}

const TOKEN_SYMBOL_MAX_LENGTH: usize = 4;

/// ## Description
/// Returns formatted liquidity token name
/// ## Params
/// * **asset_infos** is array with two items the type of [`AssetInfo`].
///
/// * **querier** is the object of type [`QuerierWrapper`].
pub fn format_lp_token_name(
    asset_infos: [AssetInfo; 2],
    querier: &QuerierWrapper,
) -> StdResult<String> {
    let mut short_symbols: Vec<String> = vec![];
    for asset_info in asset_infos {
        let short_symbol: String = match asset_info {
            AssetInfo::NativeToken { denom } => {
                denom.chars().take(TOKEN_SYMBOL_MAX_LENGTH).collect()
            }
            AssetInfo::Token { contract_addr } => query_token_symbol(querier, contract_addr)?
                .chars()
                .take(TOKEN_SYMBOL_MAX_LENGTH)
                .collect(),
        };
        short_symbols.push(short_symbol);
    }
    Ok(format!("{}-{}-LP", short_symbols[0], short_symbols[1]).to_uppercase())
}
