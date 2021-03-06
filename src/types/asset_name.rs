use std::fmt::Display;

pub fn asset_name_to_code(name: &str) -> String {
    let code = match name {
        "BTC" | "XBT" => "XXBT",
        "XRP" => "XXRP",
        "XLM" => "XXLM",
        "LTC" => "XLTC",
        "XDG" => "XXDG",
        "ETH" => "XETH",
        "ETC" => "XETC",
        "REP" => "XREP",
        "ZEC" => "XZEC",
        "XMR" => "XXMR",
        "MLN" => "XMLN",

        "USD" => "ZUSD",
        "EUR" => "ZEUR",
        "GBP" => "ZGBP",
        "CAD" => "ZCAD",
        "AUD" => "ZAUD",
        "JPY" => "ZJPY",

        _ => name,
    };

    String::from(code)
}

pub struct AssetName {
    pub name: String,
}

impl Display for AssetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl AssetName {
    pub fn from(name: &str) -> Self {
        Self {
            name: asset_name_to_code(name),
        }
    }
}

impl Into<AssetName> for &str {
    fn into(self) -> AssetName {
        AssetName::from(self)
    }
}

impl Into<AssetName> for String {
    fn into(self) -> AssetName {
        AssetName::from(&self)
    }
}

impl From<AssetName> for String {
    fn from(asset: AssetName) -> Self {
        asset.to_string()
    }
}

impl From<&AssetName> for String {
    fn from(asset: &AssetName) -> Self {
        asset.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_name() {
        let asset = AssetName::from("BTC");
        assert_eq!(asset.to_string(), "XXBT");

        let asset = AssetName::from("XBT");
        assert_eq!(asset.to_string(), "XXBT");

        let asset = AssetName::from("ETH");
        assert_eq!(asset.to_string(), "XETH");

        let asset: AssetName = "USD".into();
        assert_eq!(asset.to_string(), "ZUSD");

        let a: String = asset.into();
        assert_eq!(a, "ZUSD");
    }
}
