use crate::insrcdata as db;

pub static REFSTRS : [ &str; 6 ] = [
"𝒾ň𝗌яčḓẚᵵᶏ : 𝔢ᶆḃ℮𝚍 ᶌ𝖔ừᵳ ⅆằƫⱥ",
"hello",
"κόσμε",
"いろはにほへとちりぬるを",
"éventuellement validé",
"Да, но фальшивый экземпляр",
];

// check string comparison for various encoded unicode strings
pub fn test_strencoding() {
    for  refstr in REFSTRS {
        let mut iter = db::Strencoding::text_range(refstr, refstr);
        let r = iter.next().expect("");
        assert!(r.text()==refstr);
        assert!(iter.next().is_none());
    }
}