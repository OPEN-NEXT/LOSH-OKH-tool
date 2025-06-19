/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///A person or organization
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A person or organization",
///  "anyOf": [
///    {
///      "$ref": "#/$defs/person"
///    },
///    {
///      "$ref": "#/$defs/organization"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Agent {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<Person>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<Organization>,
}
impl ::std::convert::From<&Agent> for Agent {
    fn from(value: &Agent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Agent {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl Agent {
    pub fn builder() -> builder::Agent {
        Default::default()
    }
}
/**relative or absolute path to files that are neither source files nor their exports, but still useful in the repository (e.g. KiCAD library files);\
multiple inputs possible (with one entry each)*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "relative or absolute path to files that are neither source files nor their exports, but still useful in the repository (e.g. KiCAD library files);\\\nmultiple inputs possible (with one entry each)",
///  "examples": [
///    "lib/lib1.lib",
///    ".mdlrc",
///    [
///      "lib/lib1.lib",
///      ".mdlrc"
///    ]
///  ],
///  "$ref": "#/$defs/relPathOrWebUrlMulti"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Auxiliary(pub RelPathOrWebUrlMulti);
impl ::std::ops::Deref for Auxiliary {
    type Target = RelPathOrWebUrlMulti;
    fn deref(&self) -> &RelPathOrWebUrlMulti {
        &self.0
    }
}
impl ::std::convert::From<Auxiliary> for RelPathOrWebUrlMulti {
    fn from(value: Auxiliary) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Auxiliary> for Auxiliary {
    fn from(value: &Auxiliary) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrlMulti> for Auxiliary {
    fn from(value: RelPathOrWebUrlMulti) -> Self {
        Self(value)
    }
}
///URL or repo-relative path to the bill of materials
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "URL or repo-relative path to the bill of materials",
///  "examples": [
///    "sBoM.csv",
///    "BOM.csv",
///    "bom.csv"
///  ],
///  "$ref": "#/$defs/relPathOrWebUrlMulti"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Bom(pub RelPathOrWebUrlMulti);
impl ::std::ops::Deref for Bom {
    type Target = RelPathOrWebUrlMulti;
    fn deref(&self) -> &RelPathOrWebUrlMulti {
        &self.0
    }
}
impl ::std::convert::From<Bom> for RelPathOrWebUrlMulti {
    fn from(value: Bom) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Bom> for Bom {
    fn from(value: &Bom) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrlMulti> for Bom {
    fn from(value: RelPathOrWebUrlMulti) -> Self {
        Self(value)
    }
}
///`CpcId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "examples": [
///    "A01B33/00",
///    "A41G",
///    "A01",
///    "A",
///    "B23K",
///    "B25J9/026",
///    "B62K",
///    "B63C",
///    "D03D 35/00",
///    "D03D 5/00",
///    "D06B",
///    "F16M 11/2078",
///    "F16M11/2078",
///    "G01N",
///    "G05B",
///    "G06C 7/02",
///    "H01H",
///    "H01Q",
///    "H02J",
///    "H02J 1/00",
///    "H04",
///    "H04W",
///    "H05K",
///    "Y02P",
///    "H02J 1/00",
///    "H02J 1/12",
///    "H02J 1/123",
///    "H02J 1/1234",
///    "H02J 1/12345",
///    "H02J 1/123456",
///    [
///      "D03D 35/00",
///      "D03D 5/00"
///    ]
///  ],
///  "anyOf": [
///    {
///      "type": "string",
///      "pattern": "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$"
///    },
///    {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "pattern": "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$"
///      }
///    }
///  ],
///  "$comment": "Get a CPC-ID from here <https://worldwide.espacenet.com/classification>"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CpcId {
    Variant0(CpcIdVariant0),
    Variant1(::std::vec::Vec<CpcIdVariant1Item>),
}
impl ::std::convert::From<&Self> for CpcId {
    fn from(value: &CpcId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CpcIdVariant0> for CpcId {
    fn from(value: CpcIdVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<CpcIdVariant1Item>> for CpcId {
    fn from(value: ::std::vec::Vec<CpcIdVariant1Item>) -> Self {
        Self::Variant1(value)
    }
}
///`CpcIdVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CpcIdVariant0(::std::string::String);
impl ::std::ops::Deref for CpcIdVariant0 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CpcIdVariant0> for ::std::string::String {
    fn from(value: CpcIdVariant0) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CpcIdVariant0> for CpcIdVariant0 {
    fn from(value: &CpcIdVariant0) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CpcIdVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$",
                )
                .unwrap()
        });
        if (&*PATTERN).find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CpcIdVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CpcIdVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CpcIdVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CpcIdVariant0 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`CpcIdVariant1Item`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CpcIdVariant1Item(::std::string::String);
impl ::std::ops::Deref for CpcIdVariant1Item {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CpcIdVariant1Item> for ::std::string::String {
    fn from(value: CpcIdVariant1Item) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CpcIdVariant1Item> for CpcIdVariant1Item {
    fn from(value: &CpcIdVariant1Item) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CpcIdVariant1Item {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$",
                )
                .unwrap()
        });
        if (&*PATTERN).find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[A-HY][0-9]{2}[A-HJ-NP-Z]( ?[12]?[0-9]{1,3}[/][0-9]{2,6})?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CpcIdVariant1Item {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CpcIdVariant1Item {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CpcIdVariant1Item {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CpcIdVariant1Item {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`Date`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "examples": [
///    "2000-04-06",
///    "0001-0-0",
///    "1984-10-1"
///  ],
///  "type": "string",
///  "format": "date"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Date(pub ::chrono::naive::NaiveDate);
impl ::std::ops::Deref for Date {
    type Target = ::chrono::naive::NaiveDate;
    fn deref(&self) -> &::chrono::naive::NaiveDate {
        &self.0
    }
}
impl ::std::convert::From<Date> for ::chrono::naive::NaiveDate {
    fn from(value: Date) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Date> for Date {
    fn from(value: &Date) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::chrono::naive::NaiveDate> for Date {
    fn from(value: ::chrono::naive::NaiveDate) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Date {
    type Err = <::chrono::naive::NaiveDate as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Date {
    type Error = <::chrono::naive::NaiveDate as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Date {
    type Error = <::chrono::naive::NaiveDate as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Date {
    type Error = <::chrono::naive::NaiveDate as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Date {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`Doi`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "examples": [
///    "https://doi.org/10.1080/10509585.2015.1092083",
///    "10.1080/10509585.2015.1092083",
///    "doi: 10.1080/10509585.2015.1092083",
///    "DOI: 10.1080/10509585.2015.1092083"
///  ],
///  "type": "string",
///  "pattern": "^(doi: |DOI: |https://doi.org/)?10\\.\\d{4,9}\\/[-._;()/:a-zA-Z0-9]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Doi(::std::string::String);
impl ::std::ops::Deref for Doi {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Doi> for ::std::string::String {
    fn from(value: Doi) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Doi> for Doi {
    fn from(value: &Doi) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Doi {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(doi: |DOI: |https://doi.org/)?10\\.\\d{4,9}\\/[-._;()/:a-zA-Z0-9]+$",
                )
                .unwrap()
        });
        if (&*PATTERN).find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(doi: |DOI: |https://doi.org/)?10\\.\\d{4,9}\\/[-._;()/:a-zA-Z0-9]+$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Doi {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Doi {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Doi {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Doi {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`DoiOrWebUrl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/doi"
///    },
///    {
///      "$ref": "#/$defs/webUrl"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DoiOrWebUrl {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<Doi>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<WebUrl>,
}
impl ::std::convert::From<&DoiOrWebUrl> for DoiOrWebUrl {
    fn from(value: &DoiOrWebUrl) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DoiOrWebUrl {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl DoiOrWebUrl {
    pub fn builder() -> builder::DoiOrWebUrl {
        Default::default()
    }
}
///`DoiOrWebUrlMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/doiOrWebUrl"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/doiOrWebUrl"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DoiOrWebUrlMulti {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DoiOrWebUrl>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::vec::Vec<DoiOrWebUrl>>,
}
impl ::std::convert::From<&DoiOrWebUrlMulti> for DoiOrWebUrlMulti {
    fn from(value: &DoiOrWebUrlMulti) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DoiOrWebUrlMulti {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl DoiOrWebUrlMulti {
    pub fn builder() -> builder::DoiOrWebUrlMulti {
        Default::default()
    }
}
///`Email`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "examples": [
///    "jane.doe@email.com",
///    "john.doe@email.com",
///    "ester.something@good.org"
///  ],
///  "type": "string",
///  "format": "email"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct Email(pub ::std::string::String);
impl ::std::ops::Deref for Email {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Email> for ::std::string::String {
    fn from(value: Email) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Email> for Email {
    fn from(value: &Email) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Email {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Email {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Email {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
/**relative or absolute path to export file (e.g. STEP export of 3D model or PDF export of drawing);\
multiple inputs possible (with one entry each)*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "relative or absolute path to export file (e.g. STEP export of 3D model or PDF export of drawing);\\\nmultiple inputs possible (with one entry each)",
///  "examples": [
///    "3D-parts/assembly.stp",
///    "public/user-manual.pdf",
///    [
///      "3D-parts/assembly.stp",
///      "public/user-manual.pdf"
///    ]
///  ],
///  "$ref": "#/$defs/relPathOrWebUrlMulti"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Export(pub RelPathOrWebUrlMulti);
impl ::std::ops::Deref for Export {
    type Target = RelPathOrWebUrlMulti;
    fn deref(&self) -> &RelPathOrWebUrlMulti {
        &self.0
    }
}
impl ::std::convert::From<Export> for RelPathOrWebUrlMulti {
    fn from(value: Export) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Export> for Export {
    fn from(value: &Export) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrlMulti> for Export {
    fn from(value: RelPathOrWebUrlMulti) -> Self {
        Self(value)
    }
}
///`Image`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    {
///      "$ref": "#/$defs/imageObject"
///    }
///  ],
///  "$comment": "A single image, either as a simple link (relative path or URL) or as a complex image object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Image {
    RelPathOrWebUrl(RelPathOrWebUrl),
    ImageObject(ImageObject),
}
impl ::std::convert::From<&Self> for Image {
    fn from(value: &Image) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrl> for Image {
    fn from(value: RelPathOrWebUrl) -> Self {
        Self::RelPathOrWebUrl(value)
    }
}
impl ::std::convert::From<ImageObject> for Image {
    fn from(value: ImageObject) -> Self {
        Self::ImageObject(value)
    }
}
///`ImageMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/image"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/image"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ImageMulti {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<Image>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::vec::Vec<Image>>,
}
impl ::std::convert::From<&ImageMulti> for ImageMulti {
    fn from(value: &ImageMulti) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ImageMulti {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl ImageMulti {
    pub fn builder() -> builder::ImageMulti {
        Default::default()
    }
}
///a single image reference (project relative path or absolute URL), optionally with additional meta-data
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "a single image reference (project relative path or absolute URL), optionally with additional meta-data",
///  "examples": [
///    {
///      "depicts": "This OSH projects logo",
///      "location": "res/media/img/logo.svg"
///    },
///    {
///      "depicts": "A diagram depicting the flow of the different liquids through the machine in an abstract manner",
///      "location": "res/media/img/liquid-flow.svg"
///    },
///    {
///      "depicts": "A photo of the finished hardware, taken with a white background.",
///      "location": "res/assets/media/img/photo-white-bg.png"
///    },
///    {
///      "depicts": "Community supplied photo of the underbelly of our machine",
///      "location": "https://image-hoster-xzy.net/accounts/our-user/our-project/our-image.png"
///    },
///    {
///      "depicts": {
///        "language": "en",
///        "text": "A photo of the finished hardware, taken with a white background."
///      },
///      "location": "res/assets/media/img/photo-white-bg.png"
///    },
///    {
///      "depicts": [
///        {
///          "language": "en",
///          "text": "A photo of the finished hardware, taken with a white background."
///        },
///        {
///          "language": "de",
///          "text": "Ein Foto der fertigen Maschiene, aufgenommen vor einem weissen Hintergrund."
///        }
///      ],
///      "location": "res/assets/media/img/photo-white-bg.png"
///    }
///  ],
///  "type": "object",
///  "required": [
///    "location"
///  ],
///  "properties": {
///    "depicts": {
///      "description": "Human oriented description of what is visible in the image.\nThis matters for example:\n- for visually impaired or blind people\n- in case the image is for some reason not available\n- to put as a caption next to the image",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "$ref": "#/$defs/lang-text"
///        },
///        {
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/lang-text"
///          }
///        }
///      ]
///    },
///    "location": {
///      "description": "Project relative path or absolute URL linking to the image file",
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    "slots": {
///      "description": "Denotes the slot the image fills within the subject it belongs to.\nYou may also think of it as the 'role' the image plays for its parent.\nThe available slots are predefined,\nsee the [OKH image slots](http://w3id.org/oseg/ont/okhimg#slots);\nthere you will also read about the ability to define custom ones,\nthough you might also consider proposing a new common tag\n[in an issue](https://github.com/iop-alliance/OpenKnowHow/issues/new).\nAn image can fill multiple slots,\nbut each slot can be filled at most once.\nThis is useful for things like the projects icon,\nor the left-side view of the 3D model.",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "oneOf": [
///          {
///            "pattern": "^c_[0-9a-z._-]+$"
///          },
///          {
///            "enum": [
///              "icon-main",
///              "icon-main-bw",
///              "logo",
///              "logo-bw",
///              "model-3d",
///              "model-from-back",
///              "model-from-below",
///              "model-from-front",
///              "model-from-left",
///              "model-from-right",
///              "model-from-above",
///              "model-main",
///              "organization-logo",
///              "organization-logo-bw",
///              "photo-packaging",
///              "photo-thing-main",
///              "social",
///              "symbol"
///            ]
///          }
///        ]
///      }
///    },
///    "tags": {
///      "description": "Links to a tag fit to describe the image.\nThe available tags are predefined,\nsee the [OKH image tags](http://w3id.org/oseg/ont/okhimg#tags);\nthere you will also read about the ability to define custom ones,\nthough you might also consider proposing a new common tag\n[in an issue](https://github.com/iop-alliance/OpenKnowHow/issues/new).\nAn image can have multiple tags\nand each tag can be used by multiple images\neven within a single project.",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "oneOf": [
///          {
///            "pattern": "^c_[0-9a-z._-]+$"
///          },
///          {
///            "enum": [
///              "art",
///              "assembly",
///              "bw",
///              "color",
///              "diagram",
///              "drawing",
///              "gray",
///              "icon",
///              "logo",
///              "model",
///              "photo",
///              "screenshot"
///            ]
///          }
///        ]
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageObject {
    /**Human oriented description of what is visible in the image.
This matters for example:
- for visually impaired or blind people
- in case the image is for some reason not available
- to put as a caption next to the image*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub depicts: ::std::option::Option<ImageObjectDepicts>,
    ///Project relative path or absolute URL linking to the image file
    pub location: RelPathOrWebUrl,
    /**Denotes the slot the image fills within the subject it belongs to.
You may also think of it as the 'role' the image plays for its parent.
The available slots are predefined,
see the [OKH image slots](http://w3id.org/oseg/ont/okhimg#slots);
there you will also read about the ability to define custom ones,
though you might also consider proposing a new common tag
[in an issue](https://github.com/iop-alliance/OpenKnowHow/issues/new).
An image can fill multiple slots,
but each slot can be filled at most once.
This is useful for things like the projects icon,
or the left-side view of the 3D model.*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub slots: ::std::vec::Vec<ImageObjectSlotsItem>,
    /**Links to a tag fit to describe the image.
The available tags are predefined,
see the [OKH image tags](http://w3id.org/oseg/ont/okhimg#tags);
there you will also read about the ability to define custom ones,
though you might also consider proposing a new common tag
[in an issue](https://github.com/iop-alliance/OpenKnowHow/issues/new).
An image can have multiple tags
and each tag can be used by multiple images
even within a single project.*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<ImageObjectTagsItem>,
}
impl ::std::convert::From<&ImageObject> for ImageObject {
    fn from(value: &ImageObject) -> Self {
        value.clone()
    }
}
impl ImageObject {
    pub fn builder() -> builder::ImageObject {
        Default::default()
    }
}
/**Human oriented description of what is visible in the image.
This matters for example:
- for visually impaired or blind people
- in case the image is for some reason not available
- to put as a caption next to the image*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Human oriented description of what is visible in the image.\nThis matters for example:\n- for visually impaired or blind people\n- in case the image is for some reason not available\n- to put as a caption next to the image",
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "$ref": "#/$defs/lang-text"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/lang-text"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ImageObjectDepicts {
    Variant0(::std::string::String),
    Variant1(LangText),
    Variant2(::std::vec::Vec<LangText>),
}
impl ::std::convert::From<&Self> for ImageObjectDepicts {
    fn from(value: &ImageObjectDepicts) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LangText> for ImageObjectDepicts {
    fn from(value: LangText) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<LangText>> for ImageObjectDepicts {
    fn from(value: ::std::vec::Vec<LangText>) -> Self {
        Self::Variant2(value)
    }
}
///`ImageObjectSlotsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "oneOf": [
///    {
///      "pattern": "^c_[0-9a-z._-]+$"
///    },
///    {
///      "enum": [
///        "icon-main",
///        "icon-main-bw",
///        "logo",
///        "logo-bw",
///        "model-3d",
///        "model-from-back",
///        "model-from-below",
///        "model-from-front",
///        "model-from-left",
///        "model-from-right",
///        "model-from-above",
///        "model-main",
///        "organization-logo",
///        "organization-logo-bw",
///        "photo-packaging",
///        "photo-thing-main",
///        "social",
///        "symbol"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ImageObjectSlotsItem {
    Variant0(ImageObjectSlotsItemVariant0),
    Variant1(ImageObjectSlotsItemVariant1),
}
impl ::std::convert::From<&Self> for ImageObjectSlotsItem {
    fn from(value: &ImageObjectSlotsItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ImageObjectSlotsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectSlotsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectSlotsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectSlotsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ImageObjectSlotsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<ImageObjectSlotsItemVariant0> for ImageObjectSlotsItem {
    fn from(value: ImageObjectSlotsItemVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<ImageObjectSlotsItemVariant1> for ImageObjectSlotsItem {
    fn from(value: ImageObjectSlotsItemVariant1) -> Self {
        Self::Variant1(value)
    }
}
///`ImageObjectSlotsItemVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "pattern": "^c_[0-9a-z._-]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ImageObjectSlotsItemVariant0(::std::string::String);
impl ::std::ops::Deref for ImageObjectSlotsItemVariant0 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ImageObjectSlotsItemVariant0> for ::std::string::String {
    fn from(value: ImageObjectSlotsItemVariant0) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ImageObjectSlotsItemVariant0>
for ImageObjectSlotsItemVariant0 {
    fn from(value: &ImageObjectSlotsItemVariant0) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ImageObjectSlotsItemVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^c_[0-9a-z._-]+$").unwrap() });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^c_[0-9a-z._-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectSlotsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectSlotsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectSlotsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ImageObjectSlotsItemVariant0 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ImageObjectSlotsItemVariant1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "icon-main",
///    "icon-main-bw",
///    "logo",
///    "logo-bw",
///    "model-3d",
///    "model-from-back",
///    "model-from-below",
///    "model-from-front",
///    "model-from-left",
///    "model-from-right",
///    "model-from-above",
///    "model-main",
///    "organization-logo",
///    "organization-logo-bw",
///    "photo-packaging",
///    "photo-thing-main",
///    "social",
///    "symbol"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ImageObjectSlotsItemVariant1 {
    #[serde(rename = "icon-main")]
    IconMain,
    #[serde(rename = "icon-main-bw")]
    IconMainBw,
    #[serde(rename = "logo")]
    Logo,
    #[serde(rename = "logo-bw")]
    LogoBw,
    #[serde(rename = "model-3d")]
    Model3d,
    #[serde(rename = "model-from-back")]
    ModelFromBack,
    #[serde(rename = "model-from-below")]
    ModelFromBelow,
    #[serde(rename = "model-from-front")]
    ModelFromFront,
    #[serde(rename = "model-from-left")]
    ModelFromLeft,
    #[serde(rename = "model-from-right")]
    ModelFromRight,
    #[serde(rename = "model-from-above")]
    ModelFromAbove,
    #[serde(rename = "model-main")]
    ModelMain,
    #[serde(rename = "organization-logo")]
    OrganizationLogo,
    #[serde(rename = "organization-logo-bw")]
    OrganizationLogoBw,
    #[serde(rename = "photo-packaging")]
    PhotoPackaging,
    #[serde(rename = "photo-thing-main")]
    PhotoThingMain,
    #[serde(rename = "social")]
    Social,
    #[serde(rename = "symbol")]
    Symbol,
}
impl ::std::convert::From<&Self> for ImageObjectSlotsItemVariant1 {
    fn from(value: &ImageObjectSlotsItemVariant1) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ImageObjectSlotsItemVariant1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::IconMain => write!(f, "icon-main"),
            Self::IconMainBw => write!(f, "icon-main-bw"),
            Self::Logo => write!(f, "logo"),
            Self::LogoBw => write!(f, "logo-bw"),
            Self::Model3d => write!(f, "model-3d"),
            Self::ModelFromBack => write!(f, "model-from-back"),
            Self::ModelFromBelow => write!(f, "model-from-below"),
            Self::ModelFromFront => write!(f, "model-from-front"),
            Self::ModelFromLeft => write!(f, "model-from-left"),
            Self::ModelFromRight => write!(f, "model-from-right"),
            Self::ModelFromAbove => write!(f, "model-from-above"),
            Self::ModelMain => write!(f, "model-main"),
            Self::OrganizationLogo => write!(f, "organization-logo"),
            Self::OrganizationLogoBw => write!(f, "organization-logo-bw"),
            Self::PhotoPackaging => write!(f, "photo-packaging"),
            Self::PhotoThingMain => write!(f, "photo-thing-main"),
            Self::Social => write!(f, "social"),
            Self::Symbol => write!(f, "symbol"),
        }
    }
}
impl ::std::str::FromStr for ImageObjectSlotsItemVariant1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "icon-main" => Ok(Self::IconMain),
            "icon-main-bw" => Ok(Self::IconMainBw),
            "logo" => Ok(Self::Logo),
            "logo-bw" => Ok(Self::LogoBw),
            "model-3d" => Ok(Self::Model3d),
            "model-from-back" => Ok(Self::ModelFromBack),
            "model-from-below" => Ok(Self::ModelFromBelow),
            "model-from-front" => Ok(Self::ModelFromFront),
            "model-from-left" => Ok(Self::ModelFromLeft),
            "model-from-right" => Ok(Self::ModelFromRight),
            "model-from-above" => Ok(Self::ModelFromAbove),
            "model-main" => Ok(Self::ModelMain),
            "organization-logo" => Ok(Self::OrganizationLogo),
            "organization-logo-bw" => Ok(Self::OrganizationLogoBw),
            "photo-packaging" => Ok(Self::PhotoPackaging),
            "photo-thing-main" => Ok(Self::PhotoThingMain),
            "social" => Ok(Self::Social),
            "symbol" => Ok(Self::Symbol),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectSlotsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectSlotsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectSlotsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ImageObjectTagsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "oneOf": [
///    {
///      "pattern": "^c_[0-9a-z._-]+$"
///    },
///    {
///      "enum": [
///        "art",
///        "assembly",
///        "bw",
///        "color",
///        "diagram",
///        "drawing",
///        "gray",
///        "icon",
///        "logo",
///        "model",
///        "photo",
///        "screenshot"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ImageObjectTagsItem {
    Variant0(ImageObjectTagsItemVariant0),
    Variant1(ImageObjectTagsItemVariant1),
}
impl ::std::convert::From<&Self> for ImageObjectTagsItem {
    fn from(value: &ImageObjectTagsItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ImageObjectTagsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectTagsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectTagsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectTagsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ImageObjectTagsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<ImageObjectTagsItemVariant0> for ImageObjectTagsItem {
    fn from(value: ImageObjectTagsItemVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<ImageObjectTagsItemVariant1> for ImageObjectTagsItem {
    fn from(value: ImageObjectTagsItemVariant1) -> Self {
        Self::Variant1(value)
    }
}
///`ImageObjectTagsItemVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "pattern": "^c_[0-9a-z._-]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ImageObjectTagsItemVariant0(::std::string::String);
impl ::std::ops::Deref for ImageObjectTagsItemVariant0 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ImageObjectTagsItemVariant0> for ::std::string::String {
    fn from(value: ImageObjectTagsItemVariant0) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ImageObjectTagsItemVariant0> for ImageObjectTagsItemVariant0 {
    fn from(value: &ImageObjectTagsItemVariant0) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ImageObjectTagsItemVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^c_[0-9a-z._-]+$").unwrap() });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^c_[0-9a-z._-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectTagsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectTagsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectTagsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ImageObjectTagsItemVariant0 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ImageObjectTagsItemVariant1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "art",
///    "assembly",
///    "bw",
///    "color",
///    "diagram",
///    "drawing",
///    "gray",
///    "icon",
///    "logo",
///    "model",
///    "photo",
///    "screenshot"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ImageObjectTagsItemVariant1 {
    #[serde(rename = "art")]
    Art,
    #[serde(rename = "assembly")]
    Assembly,
    #[serde(rename = "bw")]
    Bw,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "diagram")]
    Diagram,
    #[serde(rename = "drawing")]
    Drawing,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "logo")]
    Logo,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "screenshot")]
    Screenshot,
}
impl ::std::convert::From<&Self> for ImageObjectTagsItemVariant1 {
    fn from(value: &ImageObjectTagsItemVariant1) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ImageObjectTagsItemVariant1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Art => write!(f, "art"),
            Self::Assembly => write!(f, "assembly"),
            Self::Bw => write!(f, "bw"),
            Self::Color => write!(f, "color"),
            Self::Diagram => write!(f, "diagram"),
            Self::Drawing => write!(f, "drawing"),
            Self::Gray => write!(f, "gray"),
            Self::Icon => write!(f, "icon"),
            Self::Logo => write!(f, "logo"),
            Self::Model => write!(f, "model"),
            Self::Photo => write!(f, "photo"),
            Self::Screenshot => write!(f, "screenshot"),
        }
    }
}
impl ::std::str::FromStr for ImageObjectTagsItemVariant1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "art" => Ok(Self::Art),
            "assembly" => Ok(Self::Assembly),
            "bw" => Ok(Self::Bw),
            "color" => Ok(Self::Color),
            "diagram" => Ok(Self::Diagram),
            "drawing" => Ok(Self::Drawing),
            "gray" => Ok(Self::Gray),
            "icon" => Ok(Self::Icon),
            "logo" => Ok(Self::Logo),
            "model" => Ok(Self::Model),
            "photo" => Ok(Self::Photo),
            "screenshot" => Ok(Self::Screenshot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageObjectTagsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageObjectTagsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageObjectTagsItemVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///a text and the language it is written in
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "a text and the language it is written in",
///  "type": "object",
///  "required": [
///    "language",
///    "text"
///  ],
///  "properties": {
///    "language": {
///      "description": "The BCP 47 language tag the content is written in",
///      "$ref": "#/$defs/language"
///    },
///    "text": {
///      "description": "the text content",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LangText {
    ///The BCP 47 language tag the content is written in
    pub language: Language,
    ///the text content
    pub text: ::std::string::String,
}
impl ::std::convert::From<&LangText> for LangText {
    fn from(value: &LangText) -> Self {
        value.clone()
    }
}
impl LangText {
    pub fn builder() -> builder::LangText {
        Default::default()
    }
}
///Language as a BCP 47 language tag
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Language as a BCP 47 language tag",
///  "examples": [
///    "en",
///    "de",
///    "es",
///    "zh"
///  ],
///  "type": "string",
///  "pattern": "^(((en-GB-oed|i-ami|i-bnn|i-default|i-enochian|i-hak|i-klingon|i-lux|i-mingo|i-navajo|i-pwn|i-tao|i-tay|i-tsu|sgn-BE-FR|sgn-BE-NL|sgn-CH-DE)|(art-lojban|cel-gaulish|no-bok|no-nyn|zh-guoyu|zh-hakka|zh-min|zh-min-nan|zh-xiang))|((([A-Za-z]{2,3}(-([A-Za-z]{3}(-[A-Za-z]{3}){0,2}))?)|[A-Za-z]{4}|[A-Za-z]{5,8})(-([A-Za-z]{4}))?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-([0-9A-WY-Za-wy-z](-[A-Za-z0-9]{2,8})+))*(-(x(-[A-Za-z0-9]{1,8})+))?)|(x(-[A-Za-z0-9]{1,8})+))$",
///  "$comment": "autocomplete"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Language(::std::string::String);
impl ::std::ops::Deref for Language {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Language> for ::std::string::String {
    fn from(value: Language) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Language> for Language {
    fn from(value: &Language) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Language {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(((en-GB-oed|i-ami|i-bnn|i-default|i-enochian|i-hak|i-klingon|i-lux|i-mingo|i-navajo|i-pwn|i-tao|i-tay|i-tsu|sgn-BE-FR|sgn-BE-NL|sgn-CH-DE)|(art-lojban|cel-gaulish|no-bok|no-nyn|zh-guoyu|zh-hakka|zh-min|zh-min-nan|zh-xiang))|((([A-Za-z]{2,3}(-([A-Za-z]{3}(-[A-Za-z]{3}){0,2}))?)|[A-Za-z]{4}|[A-Za-z]{5,8})(-([A-Za-z]{4}))?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-([0-9A-WY-Za-wy-z](-[A-Za-z0-9]{2,8})+))*(-(x(-[A-Za-z0-9]{1,8})+))?)|(x(-[A-Za-z0-9]{1,8})+))$",
                )
                .unwrap()
        });
        if (&*PATTERN).find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(((en-GB-oed|i-ami|i-bnn|i-default|i-enochian|i-hak|i-klingon|i-lux|i-mingo|i-navajo|i-pwn|i-tao|i-tay|i-tsu|sgn-BE-FR|sgn-BE-NL|sgn-CH-DE)|(art-lojban|cel-gaulish|no-bok|no-nyn|zh-guoyu|zh-hakka|zh-min|zh-min-nan|zh-xiang))|((([A-Za-z]{2,3}(-([A-Za-z]{3}(-[A-Za-z]{3}){0,2}))?)|[A-Za-z]{4}|[A-Za-z]{5,8})(-([A-Za-z]{4}))?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-([0-9A-WY-Za-wy-z](-[A-Za-z0-9]{2,8})+))*(-(x(-[A-Za-z0-9]{1,8})+))?)|(x(-[A-Za-z0-9]{1,8})+))$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Language {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Language {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Language {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///IETF BCP 47 language tag(s)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "IETF BCP 47 language tag(s)",
///  "anyOf": [
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/language"
///      }
///    },
///    {
///      "$ref": "#/$defs/language"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum LanguageMulti {
    Variant0(::std::vec::Vec<Language>),
    Variant1(Language),
}
impl ::std::convert::From<&Self> for LanguageMulti {
    fn from(value: &LanguageMulti) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Language>> for LanguageMulti {
    fn from(value: ::std::vec::Vec<Language>) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<Language> for LanguageMulti {
    fn from(value: Language) -> Self {
        Self::Variant1(value)
    }
}
///This is a JSON-Schema which can validate an 'okh.toml' file, which holds an Open Source Hardware (OSH) projects Open Know-How (OKH) meta-data.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://w3id.org/oseg/schema/okh.json",
///  "title": "Manifest",
///  "description": "This is a JSON-Schema which can validate an 'okh.toml' file, which holds an Open Source Hardware (OSH) projects Open Know-How (OKH) meta-data.",
///  "type": "object",
///  "required": [
///    "function",
///    "license",
///    "licensor",
///    "name",
///    "okhv",
///    "repo"
///  ],
///  "properties": {
///    "$schema": {
///      "description": "Link to OKH JSON-Schema",
///      "type": "string",
///      "enum": [
///        "https://json.schemastore.org/okh.json",
///        "https://w3id.org/oseg/schema/okh.json"
///      ]
///    },
///    "attestation": {
///      "description": "reference to one or multiple valid attestation(s) that the documentation is complete and fully qualifies as open source hardware;\\\nissuing conformity assessment bodies according to DIN SPEC 3105-2:\\\n- [Open Hardware Observatory](https://en.oho.wiki/wiki/Request_certification_for_your_project)\\\n- [Open Source Ecology Germany](https://gitlab.opensourceecology.de/verein/projekte/cab/CAB)\\\n- [OSHWA certification programme](https://certification.oshwa.org/)",
///      "$ref": "#/$defs/relPathOrWebUrlMulti"
///    },
///    "auxiliary": {
///      "$ref": "#/$defs/auxiliary"
///    },
///    "bom": {
///      "$ref": "#/$defs/bom"
///    },
///    "contribution-guide": {
///      "description": "repo-relative path to the contribution guide",
///      "examples": [
///        "CONTRIBUTING.md",
///        "CONTRIB.md",
///        "CONTRIBUTING",
///        "CONTRIB"
///      ],
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    "cpc-patent-class": {
///      "description": "patent class identifier of the Cooperative Patent Classification that describes best the field of technology of the OSH module.\\\nGet it from here: <https://worldwide.espacenet.com/classification>",
///      "$ref": "#/$defs/cpcId"
///    },
///    "documentation-language": {
///      "description": "IETF BCP 47 language tag(s) for the language(s) in which the documentation is written",
///      "$ref": "#/$defs/languageMulti"
///    },
///    "documentation-readiness-level": {
///      "description": "ODRL-ID representing the development stage of the documentation; get it from: <https://w3id.org/oseg/ont/otrl>",
///      "default": "ODRL-1",
///      "oneOf": [
///        {
///          "title": "Documentation process commenced",
///          "description": "Published information under free open source license",
///          "const": "ODRL-1"
///        },
///        {
///          "title": "Collaborative documentation in progress",
///          "description": "Provision of documentation files and in editable formats enabling collaboration development",
///          "const": "ODRL-2"
///        },
///        {
///          "title": "Full documentation published",
///          "description": "Complete documentation as per DIN SPEC 3105-1",
///          "const": "ODRL-3"
///        },
///        {
///          "title": "Full documentation published & audited",
///          "description": "Public evidence of documentation maturity",
///          "const": "ODRL-3*"
///        },
///        {
///          "title": "Full documentation for product qualification",
///          "description": "Product qualification documents published enabling decentralized commercial distribution",
///          "const": "ODRL-4"
///        }
///      ]
///    },
///    "export": {
///      "$ref": "#/$defs/export"
///    },
///    "forkOf": {
///      "$ref": "#/$defs/webUrl"
///    },
///    "function": {
///      "description": "functional description, e.g. what it actually does, what problem it solves, for whom, under which conditions etc.\\\nSo if you wish that someone finds & uses your OSH specifically e.g. for COVID-19-crisis response, include relevant keywords in this field",
///      "examples": [
///        "A fully programmable, impeccably built, open source, split mechanical keyboard designed for extreme productivity and ergonomics.",
///        "A Handibot tool is a new kind of portable, digitally-controlled power tool for cutting, drilling, carving, and many other machining operations - the first Universal Digital Power Tool (UDPT) - or just, a Smart Tool. If you're familiar with industrial CNC (computer numerically controlled) equipment, think of the Handibot tool as a portable version of CNC. ",
///        "A tandem bicycle, with practically the same size, weight, and cost of a standard bicycle.",
///        "CARLA is a resistant bicycle trailer, which can be coupled with any regular bike and can transport easily 150 kg load. CARLA distinguishes itself for the outstanding agility and a very small turning circle. CARLA bicycle trailer is very well in tune with e-bikes for example, with a mid-motor.",
///        "FarmBot Genesis is top-of-the-line FarmBot model designed with the most features and flexibility. It is suitable for growing food with the highest level of precision, running complex experiments, and capable of being easily modified and extended to do more.",
///        "Flipper Zero is a multi tool device for geeks with a curious personality of a cyber-dolphin who really loves to hack. It can interact with digital systems in real life and grow while you are hacking. The idea of Flipper Zero is to combine all the phreaking hardware tools you'd need for hacking on the go.",
///        "GEK Gasifier comes as an assemble-yourself kit that provides stand-alone wood gas for a variety of end uses.",
///        "Inkplate 6 is a powerful, Wi-Fi enabled ESP32 based six-inch e-paper display - recycled from a Kindle e-reader.",
///        "KORUZA innovates the design of a free-space optical communication system reusing mass produced Small Form-factor Pluggable (SFP) electro-optical transceivers, combining the latest advances in low-cost 3D printing using the Fused Deposition Modelling (F DM) method with bare-minimum custom electronics design.",
///        "LittleRP is an Open Source Resin 3D Printer ",
///        "mechanical setup for the COSI Measure",
///        "MNT Reform is the radical, ultimate open hardware laptop, designed and assembled in Berlin.",
///        "OpenEVSE supplies open source charging station hardware and software solutions to manufactures and individuals. ",
///        "OpenROV is a DIY telerobotics community centered around underwater exploration & adventure.",
///        "Opentrons makes robots for biologists. The robots automate experiments that would otherwise be done by hand, allowing our users to spend more time pursuing answers to the 21st century's most important questions, and less time pipetting.",
///        "The AXIOM Beta is an open source, open hardware, professional-grade digital film camera made by apertus°. It is designed to be modular e.g. interchangeable sensor front end etc. and supports recording at 4K resolution.",
///        "The Corne is a 40% split mechanical USB general purpose keyboard. It is made up of two halves with 3x6 column staggered keys and 3 thumb keys. It has full RGB back-lighting, and is fully programmable using the popular Open Source QMK firmware. The basic design principles are, to have a minimal, ergonomically arranged set of keys that are all reachable by moving a finger by at most a distance of one key in diagonal.",
///        "The Corne keyboard is a split keyboard with 3x6 column staggered keys and 3 thumb keys, based on Helix. Crkbd stands for Corne Keyboard.",
///        "The Electric Eel Wheel is an affordable electric spinning wheel that is revolutionizing the fiber world!  The uptake is controlled with a unique scotch tension design and the yarn flows through a clever flyer assembly. It is an extremely light and portable design.",
///        "The goal of GliaX-Faceshield is to create a low cost, high quality, reusable face shield that can be quickly deployed.",
///        "The Lasersaur is a beautiful laser cutter with an outstanding price-performance ratio. We designed it to fill the need of makers, designers, architects and researchers who want a safe and highly-capable machine. Unlike others it's open source and comes fully loaded with knowledge to run, maintain, and modify.",
///        "The robot having functional characteristics of animal such as Run, Walk, Crawl, Walk and run in different heights and take push ups operated autonomously and via android app.",
///        "This charge controller is a so-called maximum power point tracker (MPPT), which automatically adapts its input voltage to the connected solar panel to extracts as much power as possible. The MPPT function can only be achieved using a DC/DC converter, which is the core part of the charge controller PCB. It can be recognized by the large inductor and the large electrolytic input and output filter capacitors."
///      ],
///      "type": "string"
///    },
///    "image": {
///      "$ref": "#/$defs/imageMulti"
///    },
///    "license": {
///      "description": "An SPDX license _expression\n(see: <https://spdx.github.io/spdx-spec/v2-draft/SPDX-license-expressions/>),\nusually a single SPDX license ID\n(see complete list: <https://spdx.org/licenses/>),\nor a combination of those,\ncombined with AND or OR.\nIf your license is not SPDX registered (yet),\nuse a custom string prefixed with 'LicenseRef-',\nfor example 'LicenseRef-MyVeryOwnLicense-1.3';\nplease use the 'SPDX identifier' from\n<https://scancode-licensedb.aboutcode.org/>,\nif your license is there but not SPDX registered.\nUse 'LicenseRef-NOASSERTION' if no license is specified,\n'LicenseRef-NONE' if no license is specified\n(which usually means: all rights reserved),\nor 'LicenseRef-AllRightsReserved' or similar\nfor projects clearly indicating that they are proprietary.",
///      "examples": [
///        "GPL-3.0-or-later",
///        "AGPL-3.0-or-later",
///        "GPL-3.0-only",
///        "AGPL-3.0-only",
///        "0BSD",
///        "CC0-1.0",
///        "CC-BY-SA-4.0",
///        "CC-BY-4.0",
///        "Unlicense",
///        "LicenseRef-NOASSERTION",
///        "LicenseRef-NONE",
///        "LicenseRef-AllRightsReserved",
///        "LicenseRef-RandomNonSpdxRegisteredLicense",
///        "LicenseRef-MyVeryOwnLicense"
///      ],
///      "$ref": "#/$defs/spdxLicenseExpression",
///      "$comment": "NOTE: When no SPDX key is found by the crawler, metadata might not be indexed on the server until the alternative license has been whitelisted by maintainers. At OKH, we need to make sure that all results indexed by our own server instance are actually open source."
///    },
///    "licensor": {
///      "description": "organization/individual behind the hardware design (holder of intellectual property)",
///      "examples": [
///        "John Doe <john.doe@email.com>",
///        "Jane Doe (FSF) <jane.doe@email.com>",
///        "Michael Mueller <mm@email.com>",
///        "Jinz Jixxer (OSI) <jj@email.com>",
///        "Pru Ner (GNU) <abc@email.com>",
///        [
///          "John Doe <john.doe@email.com>",
///          "Jane Doe (FSF) <jane.doe@email.com>"
///        ],
///        {
///          "email": "john.doe@email.com",
///          "name": "John Doe"
///        },
///        {
///          "email": "jane.doe@email.com",
///          "memberOf": "FSF",
///          "name": "Jane Doe"
///        },
///        [
///          {
///            "email": "john.doe@email.com",
///            "name": "John Doe"
///          },
///          {
///            "email": "jane.doe@email.com",
///            "memberOf": "FSF",
///            "name": "Jane Doe"
///          }
///        ]
///      ],
///      "anyOf": [
///        {
///          "$ref": "#/$defs/stringMulti"
///        },
///        {
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/agent"
///          }
///        },
///        {
///          "$ref": "#/$defs/agent"
///        }
///      ]
///    },
///    "manufacturing-instructions": {
///      "description": "URL or repo-relative path to manufacturing instructions; multiple inputs possible (with one entry each)",
///      "examples": [
///        "Documentation/Assembly_Guide/AssemblyGuide.md"
///      ],
///      "$ref": "#/$defs/relPathOrWebUrlMulti"
///    },
///    "mass": {
///      "$ref": "#/$defs/mass"
///    },
///    "name": {
///      "description": "Working title of the OSH component",
///      "$ref": "#/$defs/name"
///    },
///    "okhv": {
///      "description": "version of OKH specification the OSH projects metadata follows (different version → different data fields in this file)",
///      "const": "2.4",
///      "$comment": "ui-hidden"
///    },
///    "organization": {
///      "description": "organization of the licensor or that represents (some of) the contributors of to project",
///      "$ref": "#/$defs/organizationMulti"
///    },
///    "outer-dimensions": {
///      "$ref": "#/$defs/outerDimensions"
///    },
///    "part": {
///      "description": "physical component(s) of this open source hardware module, for which technical documentation (design files etc.) is available under a free/open license",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/part"
///      }
///    },
///    "publication": {
///      "description": "The DOI(s) or web URL(s) of one or multiple associated publication(s)",
///      "$ref": "#/$defs/doiOrWebUrlMulti"
///    },
///    "rdf": {
///      "description": "absolute HTTP IRI (ending int '/' or '#') representing the RDF namespace of the triples generated from the manifest, and optionally the name of the prefix",
///      "$ref": "#/$defs/rdfNamespace"
///    },
///    "readme": {
///      "description": "repo-relative path (or absolute HTTP(S) URL) to to the corresponding ReadMe, which is the (human) entry-point into (the sources of) an OSH project",
///      "examples": [
///        "README.md",
///        "README.txt",
///        "README",
///        "README-JP.md",
///        "README-JP",
///        [
///          "README-EN.md",
///          "README-JP.md"
///        ]
///      ],
///      "$ref": "#/$defs/relPathOrWebUrlMulti"
///    },
///    "release": {
///      "description": "URL or repo local path to the release package of this version of the OSH module",
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    "repo": {
///      "description": "URL to the (human browsable) place where development happens;\ntypically a (git) repository or Wiki page.\nFollowing this link, people should be able to contribute to the development:\nreporting issues, suggesting changes, connecting to the team etc.",
///      "$ref": "#/$defs/webUrl"
///    },
///    "software": {
///      "description": "associated software package(s) used to operate this piece of open source hardware",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/software"
///      }
///    },
///    "source": {
///      "$ref": "#/$defs/source"
///    },
///    "standard-compliance": {
///      "description": "document-number of the official standard the OSH module complies;\\\nmultiple inputs possible (with one entry each)",
///      "examples": [
///        "DIN SPEC 3105",
///        "DIN EN 1335",
///        "ISO 1337",
///        [
///          "DIN SPEC 3105",
///          "ISO 1337"
///        ]
///      ],
///      "$ref": "#/$defs/stringMulti"
///    },
///    "technology-readiness-level": {
///      "description": "OTRL-ID representing the development stage of the OSH module; get it from: <https://w3id.org/oseg/ont/otrl>",
///      "default": "OTRL-1",
///      "oneOf": [
///        {
///          "title": "Ideation",
///          "description": "Product idea; needs are identified and initial specifications are defined",
///          "const": "OTRL-1"
///        },
///        {
///          "title": "Conception",
///          "description": "Mature product concept has been formulated",
///          "const": "OTRL-2"
///        },
///        {
///          "title": "Development",
///          "description": "Product model is developed",
///          "const": "OTRL-3"
///        },
///        {
///          "title": "Prototyping and testing",
///          "description": "Full functional prototype is built and tested",
///          "const": "OTRL-4"
///        },
///        {
///          "title": "Manufacturing development",
///          "description": "Fairly reliable processes identified and characterized",
///          "const": "OTRL-5"
///        },
///        {
///          "title": "Product qualification",
///          "description": "Certificate marking conformity assessment or comparable",
///          "const": "OTRL-6"
///        }
///      ]
///    },
///    "tsdc": {
///      "$ref": "#/$defs/tsdc"
///    },
///    "user-manual": {
///      "description": "URL or repo-relative path to user manual",
///      "examples": [
///        "Documentation/User_Guide/UserGuide.md"
///      ],
///      "$ref": "#/$defs/relPathOrWebUrlMulti"
///    },
///    "version": {
///      "description": "version of this Module, preferably following the [semantic versioning-scheme v2.0.0](https://semver.org/#semantic-versioning-200)",
///      "examples": [
///        "2.3.4",
///        "1.0.0-alpha",
///        "1.0.0-alpha.1",
///        "1.0.0-alpha.beta",
///        "1.0.0-beta",
///        "1.0.0-beta.2",
///        "1.0.0-beta.11",
///        "1.0.0-rc.1",
///        "1.0.0",
///        "0.0.1-24-g6a8a3a7",
///        "v0.3.1",
///        "6a8a3a7",
///        "baf0e65d8d93e1b64e883dfd2ffc5b838a22ca25"
///      ],
///      "type": "string"
///    }
///  },
///  "additionalProperties": false,
///  "$comment": "NOTE: The properties 'ui-hidden' and 'ui-recommended' used in this schema, are non-standardized, and currently unused. They could be used to create a form, and are here for consistency with <https://git.iostud.io/makernet/iop-cdb/-/blob/dev/server/assets/okh.okhdf>."
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /**reference to one or multiple valid attestation(s) that the documentation is complete and fully qualifies as open source hardware;\
issuing conformity assessment bodies according to DIN SPEC 3105-2:\
- [Open Hardware Observatory](https://en.oho.wiki/wiki/Request_certification_for_your_project)\
- [Open Source Ecology Germany](https://gitlab.opensourceecology.de/verein/projekte/cab/CAB)\
- [OSHWA certification programme](https://certification.oshwa.org/)*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attestation: ::std::option::Option<RelPathOrWebUrlMulti>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auxiliary: ::std::option::Option<Auxiliary>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bom: ::std::option::Option<Bom>,
    ///repo-relative path to the contribution guide
    #[serde(
        rename = "contribution-guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub contribution_guide: ::std::option::Option<RelPathOrWebUrl>,
    /**patent class identifier of the Cooperative Patent Classification that describes best the field of technology of the OSH module.\
Get it from here: <https://worldwide.espacenet.com/classification>*/
    #[serde(
        rename = "cpc-patent-class",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cpc_patent_class: ::std::option::Option<CpcId>,
    ///IETF BCP 47 language tag(s) for the language(s) in which the documentation is written
    #[serde(
        rename = "documentation-language",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub documentation_language: ::std::option::Option<LanguageMulti>,
    ///ODRL-ID representing the development stage of the documentation; get it from: <https://w3id.org/oseg/ont/otrl>
    #[serde(
        rename = "documentation-readiness-level",
        default = "defaults::manifest_documentation_readiness_level"
    )]
    pub documentation_readiness_level: ManifestDocumentationReadinessLevel,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub export: ::std::option::Option<Export>,
    #[serde(
        rename = "forkOf",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fork_of: ::std::option::Option<WebUrl>,
    /**functional description, e.g. what it actually does, what problem it solves, for whom, under which conditions etc.\
So if you wish that someone finds & uses your OSH specifically e.g. for COVID-19-crisis response, include relevant keywords in this field*/
    pub function: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<ImageMulti>,
    /**An SPDX license _expression
(see: <https://spdx.github.io/spdx-spec/v2-draft/SPDX-license-expressions/>),
usually a single SPDX license ID
(see complete list: <https://spdx.org/licenses/>),
or a combination of those,
combined with AND or OR.
If your license is not SPDX registered (yet),
use a custom string prefixed with 'LicenseRef-',
for example 'LicenseRef-MyVeryOwnLicense-1.3';
please use the 'SPDX identifier' from
<https://scancode-licensedb.aboutcode.org/>,
if your license is there but not SPDX registered.
Use 'LicenseRef-NOASSERTION' if no license is specified,
'LicenseRef-NONE' if no license is specified
(which usually means: all rights reserved),
or 'LicenseRef-AllRightsReserved' or similar
for projects clearly indicating that they are proprietary.*/
    pub license: SpdxLicenseExpression,
    ///organization/individual behind the hardware design (holder of intellectual property)
    pub licensor: ManifestLicensor,
    ///URL or repo-relative path to manufacturing instructions; multiple inputs possible (with one entry each)
    #[serde(
        rename = "manufacturing-instructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub manufacturing_instructions: ::std::option::Option<RelPathOrWebUrlMulti>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mass: ::std::option::Option<Mass>,
    ///Working title of the OSH component
    pub name: Name,
    ///version of OKH specification the OSH projects metadata follows (different version → different data fields in this file)
    pub okhv: ::serde_json::Value,
    ///organization of the licensor or that represents (some of) the contributors of to project
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<OrganizationMulti>,
    #[serde(
        rename = "outer-dimensions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub outer_dimensions: ::std::option::Option<OuterDimensions>,
    ///physical component(s) of this open source hardware module, for which technical documentation (design files etc.) is available under a free/open license
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub part: ::std::vec::Vec<Part>,
    ///The DOI(s) or web URL(s) of one or multiple associated publication(s)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub publication: ::std::option::Option<DoiOrWebUrlMulti>,
    ///absolute HTTP IRI (ending int '/' or '#') representing the RDF namespace of the triples generated from the manifest, and optionally the name of the prefix
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rdf: ::std::option::Option<RdfNamespace>,
    ///repo-relative path (or absolute HTTP(S) URL) to to the corresponding ReadMe, which is the (human) entry-point into (the sources of) an OSH project
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub readme: ::std::option::Option<RelPathOrWebUrlMulti>,
    ///URL or repo local path to the release package of this version of the OSH module
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub release: ::std::option::Option<RelPathOrWebUrl>,
    /**URL to the (human browsable) place where development happens;
typically a (git) repository or Wiki page.
Following this link, people should be able to contribute to the development:
reporting issues, suggesting changes, connecting to the team etc.*/
    pub repo: WebUrl,
    ///Link to OKH JSON-Schema
    #[serde(
        rename = "$schema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub schema: ::std::option::Option<ManifestSchema>,
    ///associated software package(s) used to operate this piece of open source hardware
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub software: ::std::vec::Vec<Software>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<Source>,
    /**document-number of the official standard the OSH module complies;\
multiple inputs possible (with one entry each)*/
    #[serde(
        rename = "standard-compliance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub standard_compliance: ::std::option::Option<StringMulti>,
    ///OTRL-ID representing the development stage of the OSH module; get it from: <https://w3id.org/oseg/ont/otrl>
    #[serde(
        rename = "technology-readiness-level",
        default = "defaults::manifest_technology_readiness_level"
    )]
    pub technology_readiness_level: ManifestTechnologyReadinessLevel,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tsdc: ::std::option::Option<Tsdc>,
    ///URL or repo-relative path to user manual
    #[serde(
        rename = "user-manual",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_manual: ::std::option::Option<RelPathOrWebUrlMulti>,
    ///version of this Module, preferably following the [semantic versioning-scheme v2.0.0](https://semver.org/#semantic-versioning-200)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Manifest> for Manifest {
    fn from(value: &Manifest) -> Self {
        value.clone()
    }
}
impl Manifest {
    pub fn builder() -> builder::Manifest {
        Default::default()
    }
}
///ODRL-ID representing the development stage of the documentation; get it from: <https://w3id.org/oseg/ont/otrl>
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "ODRL-ID representing the development stage of the documentation; get it from: <https://w3id.org/oseg/ont/otrl>",
///  "default": "ODRL-1",
///  "oneOf": [
///    {
///      "title": "Documentation process commenced",
///      "description": "Published information under free open source license",
///      "const": "ODRL-1"
///    },
///    {
///      "title": "Collaborative documentation in progress",
///      "description": "Provision of documentation files and in editable formats enabling collaboration development",
///      "const": "ODRL-2"
///    },
///    {
///      "title": "Full documentation published",
///      "description": "Complete documentation as per DIN SPEC 3105-1",
///      "const": "ODRL-3"
///    },
///    {
///      "title": "Full documentation published & audited",
///      "description": "Public evidence of documentation maturity",
///      "const": "ODRL-3*"
///    },
///    {
///      "title": "Full documentation for product qualification",
///      "description": "Product qualification documents published enabling decentralized commercial distribution",
///      "const": "ODRL-4"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ManifestDocumentationReadinessLevel {
    ///Published information under free open source license
    #[serde(rename = "ODRL-1")]
    Odrlx1,
    ///Provision of documentation files and in editable formats enabling collaboration development
    #[serde(rename = "ODRL-2")]
    Odrlx2,
    ///Complete documentation as per DIN SPEC 3105-1
    #[serde(rename = "ODRL-3")]
    Odrlx3,
    ///Public evidence of documentation maturity
    #[serde(rename = "ODRL-3*")]
    Odrlx3x,
    ///Product qualification documents published enabling decentralized commercial distribution
    #[serde(rename = "ODRL-4")]
    Odrlx4,
}
impl ::std::convert::From<&Self> for ManifestDocumentationReadinessLevel {
    fn from(value: &ManifestDocumentationReadinessLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ManifestDocumentationReadinessLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Odrlx1 => write!(f, "ODRL-1"),
            Self::Odrlx2 => write!(f, "ODRL-2"),
            Self::Odrlx3 => write!(f, "ODRL-3"),
            Self::Odrlx3x => write!(f, "ODRL-3*"),
            Self::Odrlx4 => write!(f, "ODRL-4"),
        }
    }
}
impl ::std::str::FromStr for ManifestDocumentationReadinessLevel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ODRL-1" => Ok(Self::Odrlx1),
            "ODRL-2" => Ok(Self::Odrlx2),
            "ODRL-3" => Ok(Self::Odrlx3),
            "ODRL-3*" => Ok(Self::Odrlx3x),
            "ODRL-4" => Ok(Self::Odrlx4),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ManifestDocumentationReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ManifestDocumentationReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ManifestDocumentationReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ManifestDocumentationReadinessLevel {
    fn default() -> Self {
        ManifestDocumentationReadinessLevel::Odrlx1
    }
}
///organization/individual behind the hardware design (holder of intellectual property)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "organization/individual behind the hardware design (holder of intellectual property)",
///  "examples": [
///    "John Doe <john.doe@email.com>",
///    "Jane Doe (FSF) <jane.doe@email.com>",
///    "Michael Mueller <mm@email.com>",
///    "Jinz Jixxer (OSI) <jj@email.com>",
///    "Pru Ner (GNU) <abc@email.com>",
///    [
///      "John Doe <john.doe@email.com>",
///      "Jane Doe (FSF) <jane.doe@email.com>"
///    ],
///    {
///      "email": "john.doe@email.com",
///      "name": "John Doe"
///    },
///    {
///      "email": "jane.doe@email.com",
///      "memberOf": "FSF",
///      "name": "Jane Doe"
///    },
///    [
///      {
///        "email": "john.doe@email.com",
///        "name": "John Doe"
///      },
///      {
///        "email": "jane.doe@email.com",
///        "memberOf": "FSF",
///        "name": "Jane Doe"
///      }
///    ]
///  ],
///  "anyOf": [
///    {
///      "$ref": "#/$defs/stringMulti"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/agent"
///      }
///    },
///    {
///      "$ref": "#/$defs/agent"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestLicensor {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<StringMulti>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::vec::Vec<Agent>>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<Agent>,
}
impl ::std::convert::From<&ManifestLicensor> for ManifestLicensor {
    fn from(value: &ManifestLicensor) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ManifestLicensor {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
impl ManifestLicensor {
    pub fn builder() -> builder::ManifestLicensor {
        Default::default()
    }
}
///Link to OKH JSON-Schema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Link to OKH JSON-Schema",
///  "type": "string",
///  "enum": [
///    "https://json.schemastore.org/okh.json",
///    "https://w3id.org/oseg/schema/okh.json"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ManifestSchema {
    #[serde(rename = "https://json.schemastore.org/okh.json")]
    HttpsJsonSchemastoreOrgOkhJson,
    #[serde(rename = "https://w3id.org/oseg/schema/okh.json")]
    HttpsW3idOrgOsegSchemaOkhJson,
}
impl ::std::convert::From<&Self> for ManifestSchema {
    fn from(value: &ManifestSchema) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ManifestSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HttpsJsonSchemastoreOrgOkhJson => {
                write!(f, "https://json.schemastore.org/okh.json")
            }
            Self::HttpsW3idOrgOsegSchemaOkhJson => {
                write!(f, "https://w3id.org/oseg/schema/okh.json")
            }
        }
    }
}
impl ::std::str::FromStr for ManifestSchema {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "https://json.schemastore.org/okh.json" => {
                Ok(Self::HttpsJsonSchemastoreOrgOkhJson)
            }
            "https://w3id.org/oseg/schema/okh.json" => {
                Ok(Self::HttpsW3idOrgOsegSchemaOkhJson)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ManifestSchema {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ManifestSchema {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ManifestSchema {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///OTRL-ID representing the development stage of the OSH module; get it from: <https://w3id.org/oseg/ont/otrl>
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "OTRL-ID representing the development stage of the OSH module; get it from: <https://w3id.org/oseg/ont/otrl>",
///  "default": "OTRL-1",
///  "oneOf": [
///    {
///      "title": "Ideation",
///      "description": "Product idea; needs are identified and initial specifications are defined",
///      "const": "OTRL-1"
///    },
///    {
///      "title": "Conception",
///      "description": "Mature product concept has been formulated",
///      "const": "OTRL-2"
///    },
///    {
///      "title": "Development",
///      "description": "Product model is developed",
///      "const": "OTRL-3"
///    },
///    {
///      "title": "Prototyping and testing",
///      "description": "Full functional prototype is built and tested",
///      "const": "OTRL-4"
///    },
///    {
///      "title": "Manufacturing development",
///      "description": "Fairly reliable processes identified and characterized",
///      "const": "OTRL-5"
///    },
///    {
///      "title": "Product qualification",
///      "description": "Certificate marking conformity assessment or comparable",
///      "const": "OTRL-6"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ManifestTechnologyReadinessLevel {
    ///Product idea; needs are identified and initial specifications are defined
    #[serde(rename = "OTRL-1")]
    Otrl1,
    ///Mature product concept has been formulated
    #[serde(rename = "OTRL-2")]
    Otrl2,
    ///Product model is developed
    #[serde(rename = "OTRL-3")]
    Otrl3,
    ///Full functional prototype is built and tested
    #[serde(rename = "OTRL-4")]
    Otrl4,
    ///Fairly reliable processes identified and characterized
    #[serde(rename = "OTRL-5")]
    Otrl5,
    ///Certificate marking conformity assessment or comparable
    #[serde(rename = "OTRL-6")]
    Otrl6,
}
impl ::std::convert::From<&Self> for ManifestTechnologyReadinessLevel {
    fn from(value: &ManifestTechnologyReadinessLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ManifestTechnologyReadinessLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Otrl1 => write!(f, "OTRL-1"),
            Self::Otrl2 => write!(f, "OTRL-2"),
            Self::Otrl3 => write!(f, "OTRL-3"),
            Self::Otrl4 => write!(f, "OTRL-4"),
            Self::Otrl5 => write!(f, "OTRL-5"),
            Self::Otrl6 => write!(f, "OTRL-6"),
        }
    }
}
impl ::std::str::FromStr for ManifestTechnologyReadinessLevel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "OTRL-1" => Ok(Self::Otrl1),
            "OTRL-2" => Ok(Self::Otrl2),
            "OTRL-3" => Ok(Self::Otrl3),
            "OTRL-4" => Ok(Self::Otrl4),
            "OTRL-5" => Ok(Self::Otrl5),
            "OTRL-6" => Ok(Self::Otrl6),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ManifestTechnologyReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ManifestTechnologyReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ManifestTechnologyReadinessLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ManifestTechnologyReadinessLevel {
    fn default() -> Self {
        ManifestTechnologyReadinessLevel::Otrl1
    }
}
///`Mass`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Mass of the part in g (grams).",
///  "type": "number",
///  "exclusiveMinimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Mass(pub f64);
impl ::std::ops::Deref for Mass {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Mass> for f64 {
    fn from(value: Mass) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Mass> for Mass {
    fn from(value: &Mass) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for Mass {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Mass {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Mass {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Mass {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Mass {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Mass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///working title of the OSH Module or Part
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "working title of the OSH Module or Part",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct Name(pub ::std::string::String);
impl ::std::ops::Deref for Name {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Name> for ::std::string::String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Name> for Name {
    fn from(value: &Name) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Name {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Name {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Name {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///An organization such as a school, NGO, corporation, club, etc.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An organization such as a school, NGO, corporation, club, etc.",
///  "type": "object",
///  "properties": {
///    "email": {
///      "description": "E-Mail of the organization",
///      "$ref": "#/$defs/email"
///    },
///    "name": {
///      "description": "Name of the organization",
///      "type": "string"
///    },
///    "url": {
///      "description": "Home-page of the organization",
///      "$ref": "#/$defs/webUrl"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Organization {
    ///E-Mail of the organization
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<Email>,
    ///Name of the organization
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Home-page of the organization
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<WebUrl>,
}
impl ::std::convert::From<&Organization> for Organization {
    fn from(value: &Organization) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Organization {
    fn default() -> Self {
        Self {
            email: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl Organization {
    pub fn builder() -> builder::Organization {
        Default::default()
    }
}
///`OrganizationMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "examples": [
///    "Free Software Foundation",
///    "Open Source Initiative",
///    "Open Source Hardware Association",
///    "Open Source Ecology",
///    "Open Source Ecology Germany",
///    [
///      "Free Software Foundation",
///      "Open Source Initiative"
///    ],
///    {
///      "name": "Free Software Foundation",
///      "url": "https://www.fsf.org"
///    },
///    [
///      {
///        "name": "Free Software Foundation",
///        "url": "https://www.fsf.org"
///      },
///      {
///        "name": "Open Source Ecology Germany",
///        "url": "https://ose-germany.de"
///      }
///    ]
///  ],
///  "anyOf": [
///    {
///      "$ref": "#/$defs/stringMulti"
///    },
///    {
///      "$ref": "#/$defs/organization"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/organization"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrganizationMulti {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<StringMulti>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<Organization>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::vec::Vec<Organization>>,
}
impl ::std::convert::From<&OrganizationMulti> for OrganizationMulti {
    fn from(value: &OrganizationMulti) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OrganizationMulti {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
impl OrganizationMulti {
    pub fn builder() -> builder::OrganizationMulti {
        Default::default()
    }
}
///Outer dimensions of the OSH module or part in mm (millimeters), which completely encompass the product.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Outer dimensions of the OSH module or part in mm (millimeters), which completely encompass the product.",
///  "type": "object",
///  "required": [
///    "depth",
///    "height",
///    "width"
///  ],
///  "properties": {
///    "depth": {
///      "type": "number",
///      "exclusiveMinimum": 0.0
///    },
///    "height": {
///      "type": "number",
///      "exclusiveMinimum": 0.0
///    },
///    "width": {
///      "type": "number",
///      "exclusiveMinimum": 0.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OuterDimensions {
    pub depth: f64,
    pub height: f64,
    pub width: f64,
}
impl ::std::convert::From<&OuterDimensions> for OuterDimensions {
    fn from(value: &OuterDimensions) -> Self {
        value.clone()
    }
}
impl OuterDimensions {
    pub fn builder() -> builder::OuterDimensions {
        Default::default()
    }
}
///a physical component of an open source hardware module, for which technical documentation (design files etc.) is available under a free/open license
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "a physical component of an open source hardware module, for which technical documentation (design files etc.) is available under a free/open license",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "auxiliary": {
///      "$ref": "#/$defs/auxiliary"
///    },
///    "export": {
///      "$ref": "#/$defs/export"
///    },
///    "image": {
///      "$ref": "#/$defs/imageMulti"
///    },
///    "mass": {
///      "$ref": "#/$defs/mass"
///    },
///    "name": {
///      "$ref": "#/$defs/name"
///    },
///    "outer-dimensions": {
///      "$ref": "#/$defs/outerDimensions"
///    },
///    "source": {
///      "$ref": "#/$defs/source"
///    },
///    "tsdc": {
///      "$ref": "#/$defs/tsdc"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Part {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auxiliary: ::std::option::Option<Auxiliary>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub export: ::std::option::Option<Export>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<ImageMulti>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mass: ::std::option::Option<Mass>,
    pub name: Name,
    #[serde(
        rename = "outer-dimensions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub outer_dimensions: ::std::option::Option<OuterDimensions>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<Source>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tsdc: ::std::option::Option<Tsdc>,
}
impl ::std::convert::From<&Part> for Part {
    fn from(value: &Part) -> Self {
        value.clone()
    }
}
impl Part {
    pub fn builder() -> builder::Part {
        Default::default()
    }
}
///A person (alive, dead, undead, or fictional)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A person (alive, dead, undead, or fictional)",
///  "type": "object",
///  "properties": {
///    "email": {
///      "description": "E-Mail of the person",
///      "$ref": "#/$defs/email"
///    },
///    "name": {
///      "description": "Full name of the person",
///      "type": "string"
///    },
///    "url": {
///      "description": "Home-page of the person",
///      "$ref": "#/$defs/webUrl"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Person {
    ///E-Mail of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<Email>,
    ///Full name of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Home-page of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<WebUrl>,
}
impl ::std::convert::From<&Person> for Person {
    fn from(value: &Person) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Person {
    fn default() -> Self {
        Self {
            email: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl Person {
    pub fn builder() -> builder::Person {
        Default::default()
    }
}
///`RdfNamespace`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "namespace"
///  ],
///  "properties": {
///    "namespace": {
///      "examples": [
///        "http://w3id.org/my-org/projects/proj-x#",
///        "http://w3id.org/my-comp/projects/proj-x#"
///      ],
///      "type": "string"
///    },
///    "prefix": {
///      "examples": [
///        "prjx",
///        "myorg-prjx",
///        "mycomp-prjx"
///      ],
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RdfNamespace {
    pub namespace: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RdfNamespace> for RdfNamespace {
    fn from(value: &RdfNamespace) -> Self {
        value.clone()
    }
}
impl RdfNamespace {
    pub fn builder() -> builder::RdfNamespace {
        Default::default()
    }
}
///`RelPath`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "$comment": "A relative file-path to a dir or file below the root - as much as one can check for that with a regex"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct RelPath(pub ::std::string::String);
impl ::std::ops::Deref for RelPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RelPath> for ::std::string::String {
    fn from(value: RelPath) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RelPath> for RelPath {
    fn from(value: &RelPath) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for RelPath {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for RelPath {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for RelPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`RelPathOrWebUrl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "anyOf": [
///    {
///      "$ref": "#/$defs/webUrl"
///    },
///    {
///      "$ref": "#/$defs/relPath"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RelPathOrWebUrl {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<WebUrl>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<RelPath>,
}
impl ::std::convert::From<&RelPathOrWebUrl> for RelPathOrWebUrl {
    fn from(value: &RelPathOrWebUrl) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RelPathOrWebUrl {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl RelPathOrWebUrl {
    pub fn builder() -> builder::RelPathOrWebUrl {
        Default::default()
    }
}
///`RelPathOrWebUrlMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/relPathOrWebUrl"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RelPathOrWebUrlMulti {
    Variant0(RelPathOrWebUrl),
    Variant1(::std::vec::Vec<RelPathOrWebUrl>),
}
impl ::std::convert::From<&Self> for RelPathOrWebUrlMulti {
    fn from(value: &RelPathOrWebUrlMulti) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrl> for RelPathOrWebUrlMulti {
    fn from(value: RelPathOrWebUrl) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<RelPathOrWebUrl>> for RelPathOrWebUrlMulti {
    fn from(value: ::std::vec::Vec<RelPathOrWebUrl>) -> Self {
        Self::Variant1(value)
    }
}
///`Software`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "release"
///  ],
///  "properties": {
///    "installation-guide": {
///      "description": "unambiguous reference to the installation guide for the corresponding software release",
///      "examples": [
///        "https://github.com/arduino/ArduinoCore-mbed/blob/a2c06d768f5ebb6821ae6505b2032ee58f4ef70d/README.md"
///      ],
///      "$ref": "#/$defs/relPathOrWebUrl"
///    },
///    "release": {
///      "description": "unambiguous reference to the software release used for this version of the OSH module",
///      "examples": [
///        "https://github.com/arduino/ArduinoCore-mbed/releases/tag/1.3.2"
///      ],
///      "$ref": "#/$defs/relPathOrWebUrl"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Software {
    ///unambiguous reference to the installation guide for the corresponding software release
    #[serde(
        rename = "installation-guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub installation_guide: ::std::option::Option<RelPathOrWebUrl>,
    ///unambiguous reference to the software release used for this version of the OSH module
    pub release: RelPathOrWebUrl,
}
impl ::std::convert::From<&Software> for Software {
    fn from(value: &Software) -> Self {
        value.clone()
    }
}
impl Software {
    pub fn builder() -> builder::Software {
        Default::default()
    }
}
/**relative or absolute path to source file (e.g. native CAD file);\
multiple inputs possible (with one entry each)*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "relative or absolute path to source file (e.g. native CAD file);\\\nmultiple inputs possible (with one entry each)",
///  "examples": [
///    "3D-parts/assembly.asm",
///    "pcb/main.pro",
///    "pcb/main.kicad_pcb",
///    "cad/part-x/model.fcstd",
///    [
///      "pcb/main.kicad_pcb",
///      "cad/part-x/model.fcstd"
///    ]
///  ],
///  "$ref": "#/$defs/relPathOrWebUrlMulti"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Source(pub RelPathOrWebUrlMulti);
impl ::std::ops::Deref for Source {
    type Target = RelPathOrWebUrlMulti;
    fn deref(&self) -> &RelPathOrWebUrlMulti {
        &self.0
    }
}
impl ::std::convert::From<Source> for RelPathOrWebUrlMulti {
    fn from(value: Source) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Source> for Source {
    fn from(value: &Source) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RelPathOrWebUrlMulti> for Source {
    fn from(value: RelPathOrWebUrlMulti) -> Self {
        Self(value)
    }
}
///A valid SPDX license expression
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A valid SPDX license expression",
///  "type": "string",
///  "$comment": "TODO We would have to define this in an extra schema, generated -> do it in the SPDX-Identifiers-generator repo! -> NOPE -> already exists! see https://github.com/spdx/spdx-spec/issues/484#issuecomment-720817111    .. sounds like it needs to be improved, though -> .. ahh nope, it does not solve our issue, but checks SPDX documents for validity"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct SpdxLicenseExpression(pub ::std::string::String);
impl ::std::ops::Deref for SpdxLicenseExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SpdxLicenseExpression> for ::std::string::String {
    fn from(value: SpdxLicenseExpression) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpdxLicenseExpression> for SpdxLicenseExpression {
    fn from(value: &SpdxLicenseExpression) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for SpdxLicenseExpression {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SpdxLicenseExpression {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SpdxLicenseExpression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`StringMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum StringMulti {
    Variant0(::std::string::String),
    Variant1(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<&Self> for StringMulti {
    fn from(value: &StringMulti) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for StringMulti {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant1(value)
    }
}
///identifier of the applying Technology-specific Documentation Criteria (TsDC) according to DIN SPEC 3105-1 - get it from: <https://w3id.org/oseg/ont/tsdc/core> - multiple inputs possible (with one entry each)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "identifier of the applying Technology-specific Documentation Criteria (TsDC) according to DIN SPEC 3105-1 - get it from: <https://w3id.org/oseg/ont/tsdc/core> - multiple inputs possible (with one entry each)",
///  "examples": [
///    "ASM",
///    "MEC",
///    "CIR",
///    "PCB",
///    "WEL",
///    "3DP",
///    "LAS",
///    "CNC",
///    [
///      "ASM",
///      "MEC",
///      "CIR"
///    ]
///  ],
///  "$ref": "#/$defs/stringMulti",
///  "$comment": "autocomplete"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Tsdc(pub StringMulti);
impl ::std::ops::Deref for Tsdc {
    type Target = StringMulti;
    fn deref(&self) -> &StringMulti {
        &self.0
    }
}
impl ::std::convert::From<Tsdc> for StringMulti {
    fn from(value: Tsdc) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Tsdc> for Tsdc {
    fn from(value: &Tsdc) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<StringMulti> for Tsdc {
    fn from(value: StringMulti) -> Self {
        Self(value)
    }
}
///`WebUrl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "uri",
///  "pattern": "^https?://",
///  "$comment": "A full web URL, meaning an URL with the schema HTTP or HTTPS"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct WebUrl(pub ::std::string::String);
impl ::std::ops::Deref for WebUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WebUrl> for ::std::string::String {
    fn from(value: WebUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUrl> for WebUrl {
    fn from(value: &WebUrl) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for WebUrl {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for WebUrl {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for WebUrl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`WebUrlMulti`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/webUrl"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/webUrl"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum WebUrlMulti {
    Variant0(WebUrl),
    Variant1(::std::vec::Vec<WebUrl>),
}
impl ::std::convert::From<&Self> for WebUrlMulti {
    fn from(value: &WebUrlMulti) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<WebUrl> for WebUrlMulti {
    fn from(value: WebUrl) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<WebUrl>> for WebUrlMulti {
    fn from(value: ::std::vec::Vec<WebUrl>) -> Self {
        Self::Variant1(value)
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Agent {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::Person>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::Organization>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Agent {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl Agent {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Person>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Organization>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Agent> for super::Agent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Agent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::Agent> for Agent {
        fn from(value: super::Agent) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DoiOrWebUrl {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::Doi>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::WebUrl>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DoiOrWebUrl {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl DoiOrWebUrl {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Doi>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DoiOrWebUrl> for super::DoiOrWebUrl {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DoiOrWebUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::DoiOrWebUrl> for DoiOrWebUrl {
        fn from(value: super::DoiOrWebUrl) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DoiOrWebUrlMulti {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::DoiOrWebUrl>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::DoiOrWebUrl>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DoiOrWebUrlMulti {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl DoiOrWebUrlMulti {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DoiOrWebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::DoiOrWebUrl>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DoiOrWebUrlMulti> for super::DoiOrWebUrlMulti {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DoiOrWebUrlMulti,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::DoiOrWebUrlMulti> for DoiOrWebUrlMulti {
        fn from(value: super::DoiOrWebUrlMulti) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageMulti {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::Image>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Image>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ImageMulti {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ImageMulti {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Image>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::Image>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ImageMulti> for super::ImageMulti {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageMulti,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::ImageMulti> for ImageMulti {
        fn from(value: super::ImageMulti) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageObject {
        depicts: ::std::result::Result<
            ::std::option::Option<super::ImageObjectDepicts>,
            ::std::string::String,
        >,
        location: ::std::result::Result<super::RelPathOrWebUrl, ::std::string::String>,
        slots: ::std::result::Result<
            ::std::vec::Vec<super::ImageObjectSlotsItem>,
            ::std::string::String,
        >,
        tags: ::std::result::Result<
            ::std::vec::Vec<super::ImageObjectTagsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ImageObject {
        fn default() -> Self {
            Self {
                depicts: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
                slots: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl ImageObject {
        pub fn depicts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageObjectDepicts>>,
            T::Error: ::std::fmt::Display,
        {
            self.depicts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for depicts: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RelPathOrWebUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn slots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ImageObjectSlotsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.slots = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for slots: {}", e)
                });
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ImageObjectTagsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ImageObject> for super::ImageObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                depicts: value.depicts?,
                location: value.location?,
                slots: value.slots?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::ImageObject> for ImageObject {
        fn from(value: super::ImageObject) -> Self {
            Self {
                depicts: Ok(value.depicts),
                location: Ok(value.location),
                slots: Ok(value.slots),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LangText {
        language: ::std::result::Result<super::Language, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LangText {
        fn default() -> Self {
            Self {
                language: Err("no value supplied for language".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl LangText {
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Language>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LangText> for super::LangText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LangText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language: value.language?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::LangText> for LangText {
        fn from(value: super::LangText) -> Self {
            Self {
                language: Ok(value.language),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Manifest {
        attestation: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrlMulti>,
            ::std::string::String,
        >,
        auxiliary: ::std::result::Result<
            ::std::option::Option<super::Auxiliary>,
            ::std::string::String,
        >,
        bom: ::std::result::Result<
            ::std::option::Option<super::Bom>,
            ::std::string::String,
        >,
        contribution_guide: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrl>,
            ::std::string::String,
        >,
        cpc_patent_class: ::std::result::Result<
            ::std::option::Option<super::CpcId>,
            ::std::string::String,
        >,
        documentation_language: ::std::result::Result<
            ::std::option::Option<super::LanguageMulti>,
            ::std::string::String,
        >,
        documentation_readiness_level: ::std::result::Result<
            super::ManifestDocumentationReadinessLevel,
            ::std::string::String,
        >,
        export: ::std::result::Result<
            ::std::option::Option<super::Export>,
            ::std::string::String,
        >,
        fork_of: ::std::result::Result<
            ::std::option::Option<super::WebUrl>,
            ::std::string::String,
        >,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        image: ::std::result::Result<
            ::std::option::Option<super::ImageMulti>,
            ::std::string::String,
        >,
        license: ::std::result::Result<
            super::SpdxLicenseExpression,
            ::std::string::String,
        >,
        licensor: ::std::result::Result<super::ManifestLicensor, ::std::string::String>,
        manufacturing_instructions: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrlMulti>,
            ::std::string::String,
        >,
        mass: ::std::result::Result<
            ::std::option::Option<super::Mass>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::Name, ::std::string::String>,
        okhv: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        organization: ::std::result::Result<
            ::std::option::Option<super::OrganizationMulti>,
            ::std::string::String,
        >,
        outer_dimensions: ::std::result::Result<
            ::std::option::Option<super::OuterDimensions>,
            ::std::string::String,
        >,
        part: ::std::result::Result<::std::vec::Vec<super::Part>, ::std::string::String>,
        publication: ::std::result::Result<
            ::std::option::Option<super::DoiOrWebUrlMulti>,
            ::std::string::String,
        >,
        rdf: ::std::result::Result<
            ::std::option::Option<super::RdfNamespace>,
            ::std::string::String,
        >,
        readme: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrlMulti>,
            ::std::string::String,
        >,
        release: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrl>,
            ::std::string::String,
        >,
        repo: ::std::result::Result<super::WebUrl, ::std::string::String>,
        schema: ::std::result::Result<
            ::std::option::Option<super::ManifestSchema>,
            ::std::string::String,
        >,
        software: ::std::result::Result<
            ::std::vec::Vec<super::Software>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            ::std::option::Option<super::Source>,
            ::std::string::String,
        >,
        standard_compliance: ::std::result::Result<
            ::std::option::Option<super::StringMulti>,
            ::std::string::String,
        >,
        technology_readiness_level: ::std::result::Result<
            super::ManifestTechnologyReadinessLevel,
            ::std::string::String,
        >,
        tsdc: ::std::result::Result<
            ::std::option::Option<super::Tsdc>,
            ::std::string::String,
        >,
        user_manual: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrlMulti>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Manifest {
        fn default() -> Self {
            Self {
                attestation: Ok(Default::default()),
                auxiliary: Ok(Default::default()),
                bom: Ok(Default::default()),
                contribution_guide: Ok(Default::default()),
                cpc_patent_class: Ok(Default::default()),
                documentation_language: Ok(Default::default()),
                documentation_readiness_level: Ok(
                    super::defaults::manifest_documentation_readiness_level(),
                ),
                export: Ok(Default::default()),
                fork_of: Ok(Default::default()),
                function: Err("no value supplied for function".to_string()),
                image: Ok(Default::default()),
                license: Err("no value supplied for license".to_string()),
                licensor: Err("no value supplied for licensor".to_string()),
                manufacturing_instructions: Ok(Default::default()),
                mass: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                okhv: Err("no value supplied for okhv".to_string()),
                organization: Ok(Default::default()),
                outer_dimensions: Ok(Default::default()),
                part: Ok(Default::default()),
                publication: Ok(Default::default()),
                rdf: Ok(Default::default()),
                readme: Ok(Default::default()),
                release: Ok(Default::default()),
                repo: Err("no value supplied for repo".to_string()),
                schema: Ok(Default::default()),
                software: Ok(Default::default()),
                source: Ok(Default::default()),
                standard_compliance: Ok(Default::default()),
                technology_readiness_level: Ok(
                    super::defaults::manifest_technology_readiness_level(),
                ),
                tsdc: Ok(Default::default()),
                user_manual: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl Manifest {
        pub fn attestation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RelPathOrWebUrlMulti>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attestation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attestation: {}", e)
                });
            self
        }
        pub fn auxiliary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Auxiliary>>,
            T::Error: ::std::fmt::Display,
        {
            self.auxiliary = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for auxiliary: {}", e)
                });
            self
        }
        pub fn bom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Bom>>,
            T::Error: ::std::fmt::Display,
        {
            self.bom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bom: {}", e));
            self
        }
        pub fn contribution_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RelPathOrWebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.contribution_guide = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for contribution_guide: {}", e
                    )
                });
            self
        }
        pub fn cpc_patent_class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CpcId>>,
            T::Error: ::std::fmt::Display,
        {
            self.cpc_patent_class = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for cpc_patent_class: {}", e
                    )
                });
            self
        }
        pub fn documentation_language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.documentation_language = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for documentation_language: {}",
                        e
                    )
                });
            self
        }
        pub fn documentation_readiness_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ManifestDocumentationReadinessLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.documentation_readiness_level = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for documentation_readiness_level: {}",
                        e
                    )
                });
            self
        }
        pub fn export<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Export>>,
            T::Error: ::std::fmt::Display,
        {
            self.export = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for export: {}", e)
                });
            self
        }
        pub fn fork_of<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fork_of: {}", e)
                });
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for function: {}", e)
                });
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for image: {}", e)
                });
            self
        }
        pub fn license<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpdxLicenseExpression>,
            T::Error: ::std::fmt::Display,
        {
            self.license = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license: {}", e)
                });
            self
        }
        pub fn licensor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ManifestLicensor>,
            T::Error: ::std::fmt::Display,
        {
            self.licensor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for licensor: {}", e)
                });
            self
        }
        pub fn manufacturing_instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RelPathOrWebUrlMulti>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.manufacturing_instructions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for manufacturing_instructions: {}",
                        e
                    )
                });
            self
        }
        pub fn mass<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mass>>,
            T::Error: ::std::fmt::Display,
        {
            self.mass = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mass: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Name>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn okhv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.okhv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for okhv: {}", e));
            self
        }
        pub fn organization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OrganizationMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.organization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
            self
        }
        pub fn outer_dimensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OuterDimensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.outer_dimensions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for outer_dimensions: {}", e
                    )
                });
            self
        }
        pub fn part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Part>>,
            T::Error: ::std::fmt::Display,
        {
            self.part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part: {}", e));
            self
        }
        pub fn publication<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DoiOrWebUrlMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.publication = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for publication: {}", e)
                });
            self
        }
        pub fn rdf<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RdfNamespace>>,
            T::Error: ::std::fmt::Display,
        {
            self.rdf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf: {}", e));
            self
        }
        pub fn readme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RelPathOrWebUrlMulti>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.readme = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for readme: {}", e)
                });
            self
        }
        pub fn release<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RelPathOrWebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.release = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for release: {}", e)
                });
            self
        }
        pub fn repo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.repo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ManifestSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for schema: {}", e)
                });
            self
        }
        pub fn software<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Software>>,
            T::Error: ::std::fmt::Display,
        {
            self.software = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for software: {}", e)
                });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Source>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
        pub fn standard_compliance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.standard_compliance = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for standard_compliance: {}", e
                    )
                });
            self
        }
        pub fn technology_readiness_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ManifestTechnologyReadinessLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.technology_readiness_level = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for technology_readiness_level: {}",
                        e
                    )
                });
            self
        }
        pub fn tsdc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tsdc>>,
            T::Error: ::std::fmt::Display,
        {
            self.tsdc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tsdc: {}", e));
            self
        }
        pub fn user_manual<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RelPathOrWebUrlMulti>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.user_manual = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for user_manual: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Manifest> for super::Manifest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Manifest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attestation: value.attestation?,
                auxiliary: value.auxiliary?,
                bom: value.bom?,
                contribution_guide: value.contribution_guide?,
                cpc_patent_class: value.cpc_patent_class?,
                documentation_language: value.documentation_language?,
                documentation_readiness_level: value.documentation_readiness_level?,
                export: value.export?,
                fork_of: value.fork_of?,
                function: value.function?,
                image: value.image?,
                license: value.license?,
                licensor: value.licensor?,
                manufacturing_instructions: value.manufacturing_instructions?,
                mass: value.mass?,
                name: value.name?,
                okhv: value.okhv?,
                organization: value.organization?,
                outer_dimensions: value.outer_dimensions?,
                part: value.part?,
                publication: value.publication?,
                rdf: value.rdf?,
                readme: value.readme?,
                release: value.release?,
                repo: value.repo?,
                schema: value.schema?,
                software: value.software?,
                source: value.source?,
                standard_compliance: value.standard_compliance?,
                technology_readiness_level: value.technology_readiness_level?,
                tsdc: value.tsdc?,
                user_manual: value.user_manual?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Manifest> for Manifest {
        fn from(value: super::Manifest) -> Self {
            Self {
                attestation: Ok(value.attestation),
                auxiliary: Ok(value.auxiliary),
                bom: Ok(value.bom),
                contribution_guide: Ok(value.contribution_guide),
                cpc_patent_class: Ok(value.cpc_patent_class),
                documentation_language: Ok(value.documentation_language),
                documentation_readiness_level: Ok(value.documentation_readiness_level),
                export: Ok(value.export),
                fork_of: Ok(value.fork_of),
                function: Ok(value.function),
                image: Ok(value.image),
                license: Ok(value.license),
                licensor: Ok(value.licensor),
                manufacturing_instructions: Ok(value.manufacturing_instructions),
                mass: Ok(value.mass),
                name: Ok(value.name),
                okhv: Ok(value.okhv),
                organization: Ok(value.organization),
                outer_dimensions: Ok(value.outer_dimensions),
                part: Ok(value.part),
                publication: Ok(value.publication),
                rdf: Ok(value.rdf),
                readme: Ok(value.readme),
                release: Ok(value.release),
                repo: Ok(value.repo),
                schema: Ok(value.schema),
                software: Ok(value.software),
                source: Ok(value.source),
                standard_compliance: Ok(value.standard_compliance),
                technology_readiness_level: Ok(value.technology_readiness_level),
                tsdc: Ok(value.tsdc),
                user_manual: Ok(value.user_manual),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ManifestLicensor {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::StringMulti>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Agent>>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::Agent>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ManifestLicensor {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl ManifestLicensor {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::Agent>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Agent>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_2: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ManifestLicensor> for super::ManifestLicensor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ManifestLicensor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::ManifestLicensor> for ManifestLicensor {
        fn from(value: super::ManifestLicensor) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Organization {
        email: ::std::result::Result<
            ::std::option::Option<super::Email>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<super::WebUrl>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Organization {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Organization {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Email>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Organization> for super::Organization {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Organization,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Organization> for Organization {
        fn from(value: super::Organization) -> Self {
            Self {
                email: Ok(value.email),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationMulti {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::StringMulti>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::Organization>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Organization>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationMulti {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl OrganizationMulti {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Organization>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::Organization>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_2: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationMulti> for super::OrganizationMulti {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationMulti,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationMulti> for OrganizationMulti {
        fn from(value: super::OrganizationMulti) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OuterDimensions {
        depth: ::std::result::Result<f64, ::std::string::String>,
        height: ::std::result::Result<f64, ::std::string::String>,
        width: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for OuterDimensions {
        fn default() -> Self {
            Self {
                depth: Err("no value supplied for depth".to_string()),
                height: Err("no value supplied for height".to_string()),
                width: Err("no value supplied for width".to_string()),
            }
        }
    }
    impl OuterDimensions {
        pub fn depth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.depth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for depth: {}", e)
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<OuterDimensions> for super::OuterDimensions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OuterDimensions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                depth: value.depth?,
                height: value.height?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::OuterDimensions> for OuterDimensions {
        fn from(value: super::OuterDimensions) -> Self {
            Self {
                depth: Ok(value.depth),
                height: Ok(value.height),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Part {
        auxiliary: ::std::result::Result<
            ::std::option::Option<super::Auxiliary>,
            ::std::string::String,
        >,
        export: ::std::result::Result<
            ::std::option::Option<super::Export>,
            ::std::string::String,
        >,
        image: ::std::result::Result<
            ::std::option::Option<super::ImageMulti>,
            ::std::string::String,
        >,
        mass: ::std::result::Result<
            ::std::option::Option<super::Mass>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::Name, ::std::string::String>,
        outer_dimensions: ::std::result::Result<
            ::std::option::Option<super::OuterDimensions>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            ::std::option::Option<super::Source>,
            ::std::string::String,
        >,
        tsdc: ::std::result::Result<
            ::std::option::Option<super::Tsdc>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Part {
        fn default() -> Self {
            Self {
                auxiliary: Ok(Default::default()),
                export: Ok(Default::default()),
                image: Ok(Default::default()),
                mass: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                outer_dimensions: Ok(Default::default()),
                source: Ok(Default::default()),
                tsdc: Ok(Default::default()),
            }
        }
    }
    impl Part {
        pub fn auxiliary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Auxiliary>>,
            T::Error: ::std::fmt::Display,
        {
            self.auxiliary = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for auxiliary: {}", e)
                });
            self
        }
        pub fn export<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Export>>,
            T::Error: ::std::fmt::Display,
        {
            self.export = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for export: {}", e)
                });
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageMulti>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for image: {}", e)
                });
            self
        }
        pub fn mass<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mass>>,
            T::Error: ::std::fmt::Display,
        {
            self.mass = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mass: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Name>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn outer_dimensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OuterDimensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.outer_dimensions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for outer_dimensions: {}", e
                    )
                });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Source>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
        pub fn tsdc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tsdc>>,
            T::Error: ::std::fmt::Display,
        {
            self.tsdc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tsdc: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Part> for super::Part {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Part,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auxiliary: value.auxiliary?,
                export: value.export?,
                image: value.image?,
                mass: value.mass?,
                name: value.name?,
                outer_dimensions: value.outer_dimensions?,
                source: value.source?,
                tsdc: value.tsdc?,
            })
        }
    }
    impl ::std::convert::From<super::Part> for Part {
        fn from(value: super::Part) -> Self {
            Self {
                auxiliary: Ok(value.auxiliary),
                export: Ok(value.export),
                image: Ok(value.image),
                mass: Ok(value.mass),
                name: Ok(value.name),
                outer_dimensions: Ok(value.outer_dimensions),
                source: Ok(value.source),
                tsdc: Ok(value.tsdc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Person {
        email: ::std::result::Result<
            ::std::option::Option<super::Email>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<super::WebUrl>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Person {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Person {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Email>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Person> for super::Person {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Person,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Person> for Person {
        fn from(value: super::Person) -> Self {
            Self {
                email: Ok(value.email),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RdfNamespace {
        namespace: ::std::result::Result<::std::string::String, ::std::string::String>,
        prefix: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RdfNamespace {
        fn default() -> Self {
            Self {
                namespace: Err("no value supplied for namespace".to_string()),
                prefix: Ok(Default::default()),
            }
        }
    }
    impl RdfNamespace {
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for namespace: {}", e)
                });
            self
        }
        pub fn prefix<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prefix = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prefix: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RdfNamespace> for super::RdfNamespace {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RdfNamespace,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                namespace: value.namespace?,
                prefix: value.prefix?,
            })
        }
    }
    impl ::std::convert::From<super::RdfNamespace> for RdfNamespace {
        fn from(value: super::RdfNamespace) -> Self {
            Self {
                namespace: Ok(value.namespace),
                prefix: Ok(value.prefix),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RelPathOrWebUrl {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::WebUrl>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::RelPath>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RelPathOrWebUrl {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl RelPathOrWebUrl {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RelPath>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RelPathOrWebUrl> for super::RelPathOrWebUrl {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RelPathOrWebUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::RelPathOrWebUrl> for RelPathOrWebUrl {
        fn from(value: super::RelPathOrWebUrl) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Software {
        installation_guide: ::std::result::Result<
            ::std::option::Option<super::RelPathOrWebUrl>,
            ::std::string::String,
        >,
        release: ::std::result::Result<super::RelPathOrWebUrl, ::std::string::String>,
    }
    impl ::std::default::Default for Software {
        fn default() -> Self {
            Self {
                installation_guide: Ok(Default::default()),
                release: Err("no value supplied for release".to_string()),
            }
        }
    }
    impl Software {
        pub fn installation_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RelPathOrWebUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.installation_guide = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for installation_guide: {}", e
                    )
                });
            self
        }
        pub fn release<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RelPathOrWebUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.release = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for release: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Software> for super::Software {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Software,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                installation_guide: value.installation_guide?,
                release: value.release?,
            })
        }
    }
    impl ::std::convert::From<super::Software> for Software {
        fn from(value: super::Software) -> Self {
            Self {
                installation_guide: Ok(value.installation_guide),
                release: Ok(value.release),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn manifest_documentation_readiness_level() -> super::ManifestDocumentationReadinessLevel {
        super::ManifestDocumentationReadinessLevel::Odrlx1
    }
    pub(super) fn manifest_technology_readiness_level() -> super::ManifestTechnologyReadinessLevel {
        super::ManifestTechnologyReadinessLevel::Otrl1
    }
}
