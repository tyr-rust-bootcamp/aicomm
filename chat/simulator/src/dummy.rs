use fake::Dummy;
use rand::seq::SliceRandom as _;

pub struct AppVersion;
pub struct SystemOs;
pub struct SystemArch;
pub struct SystemLocale;
pub struct SystemTimezone;
// pub struct IPv4;
// pub struct UserAgent;
// pub struct CountryName;
pub struct RegionName;
// pub struct CityName;

pub struct MessageType;

impl Dummy<AppVersion> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &AppVersion, rng: &mut R) -> Self {
        // generate x.y.z
        let major = rng.gen_range(1..=4);
        let minor = rng.gen_range(0..=99);
        let patch = rng.gen_range(0..=99);
        format!("{}.{}.{}", major, minor, patch)
    }
}

impl Dummy<SystemOs> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemOs, rng: &mut R) -> Self {
        let os = ["macOS", "Linux", "Windows", "iOS", "Android"]
            .choose(rng)
            .unwrap();
        os.to_string()
    }
}

impl Dummy<SystemArch> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemArch, rng: &mut R) -> Self {
        let arch = ["x86_64", "aarch64"].choose(rng).unwrap();
        arch.to_string()
    }
}

impl Dummy<SystemLocale> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemLocale, rng: &mut R) -> Self {
        let locale = [
            "en_US", "en_GB", "fr_FR", "ru_RU", "zh_CN", "ja_JP", "ko_KR", "zh_TW", "zh_HK",
        ]
        .choose(rng)
        .unwrap();
        locale.to_string()
    }
}
impl Dummy<SystemTimezone> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemTimezone, rng: &mut R) -> Self {
        let timezone = [
            "America/New_York",
            "America/Los_Angeles",
            "America/Chicago",
            "America/Denver",
            "America/Phoenix",
            "America/Anchorage",
            "America/Adak",
            "America/New_York",
            "Europe/London",
            "Europe/Paris",
            "Europe/Berlin",
            "Europe/Madrid",
            "Asia/Shanghai",
            "Asia/Tokyo",
            "Asia/Seoul",
            "Asia/Hong_Kong",
            "Asia/Singapore",
            "Asia/Dubai",
            "Asia/Istanbul",
            "Asia/Kolkata",
            "Asia/Kuala_Lumpur",
            "Asia/Taipei",
            "Asia/Seoul",
            "Asia/Shanghai",
            "Asia/Tokyo",
            "Asia/Hong_Kong",
            "Asia/Singapore",
            "Asia/Dubai",
            "Asia/Istanbul",
            "Asia/Kolkata",
            "Asia/Kuala_Lumpur",
            "Asia/Taipei",
        ]
        .choose(rng)
        .unwrap();
        timezone.to_string()
    }
}

// impl Dummy<IPv4> for String {
//     fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &IPv4, rng: &mut R) -> Self {
//         let u = Uniform::new_inclusive(u8::MIN, u8::MAX);
//         format!(
//             "{}.{}.{}.{}",
//             u.sample(rng),
//             u.sample(rng),
//             u.sample(rng),
//             u.sample(rng),
//         )
//     }
// }
// impl Dummy<UserAgent> for String {
//     fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UserAgent, rng: &mut R) -> Self {
//         // list all major browser user agents
//         let user_agents = [
//             // macos
//             "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
//             "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Firefox/123.0.0.0 Safari/537.36",
//             "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36",
//             // windows
//             "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
//             "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Firefox/123.0.0.0 Safari/537.36",
//             // ios
//             "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
//             // android
//             "Mozilla/5.0 (Linux; Android 14; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Mobile Safari/537.36",
//         ]
//         .choose(rng).unwrap();
//         user_agents.to_string()
//     }
// }

// impl Dummy<CountryName> for String {
//     fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &CountryName, rng: &mut R) -> Self {
//         // iso 3166-1 alpha-2
//         let country = [
//             "US", "CN", "GB", "FR", "DE", "JP", "KR", "HK", "SG", "AE", "IN", "MY", "AU", "NZ",
//             "CA", "MX", "BR", "ZA", "EG", "NG", "KE", "UG", "GH", "ZM", "ZW", "EG", "NG", "KE",
//             "UG", "GH", "ZM", "ZW",
//         ]
//         .choose(rng)
//         .unwrap();
//         country.to_string()
//     }
// }

impl Dummy<RegionName> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &RegionName, rng: &mut R) -> Self {
        let region = ["California", "New York", "Texas", "Florida"]
            .choose(rng)
            .unwrap();
        region.to_string()
    }
}
// impl Dummy<CityName> for String {
//     fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &CityName, rng: &mut R) -> Self {
//         let city = [
//             // US
//             "New York",
//             "Los Angeles",
//             "Chicago",
//             "Houston",
//             "Miami",
//             // china
//             "Beijing",
//             "Shanghai",
//             "Guangzhou",
//             "Shenzhen",
//             "Chengdu",
//             // japan
//             "Tokyo",
//             "Osaka",
//             "Kyoto",
//             "Hiroshima",
//             "Nagoya",
//             // south korea
//             "Seoul",
//             "Busan",
//             "Incheon",
//             "Daegu",
//             "Gwangju",
//             // hong kong
//             "Hong Kong",
//             // singapore
//             "Singapore",
//             // australia
//             "Sydney",
//             "Melbourne",
//             "Brisbane",
//             "Perth",
//             "Adelaide",
//             // new zealand
//             "Auckland",
//             "Wellington",
//             "Christchurch",
//             "Queenstown",
//             "Wanaka",
//             // canada
//             "Toronto",
//             "Montreal",
//             "Vancouver",
//             "Calgary",
//             "Edmonton",
//             // germany
//             "Berlin",
//             "Hamburg",
//             "Munich",
//             "Frankfurt",
//             "Cologne",
//         ]
//         .choose(rng)
//         .unwrap();
//         city.to_string()
//     }
// }

impl Dummy<MessageType> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &MessageType, rng: &mut R) -> Self {
        let message_type = ["text", "image", "audio", "video"].choose(rng).unwrap();
        message_type.to_string()
    }
}
