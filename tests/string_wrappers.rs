use std::borrow::Cow;

use langtag::{LangTag, LangTagBuf};

#[test]
fn string_wrappers_validate_borrowed_and_owned_inputs() {
	for text in ["en-GB", "zh-Hant-TW", "x-private"] {
		let borrowed = LangTag::new(text).expect("valid borrowed tag");
		let owned = LangTagBuf::new(text.to_owned()).expect("valid owned tag");
		assert_eq!(AsRef::<str>::as_ref(borrowed), text);
		assert_eq!(owned.as_str(), text);
	}
	assert!(LangTag::new("en--GB").is_err());
	assert!(LangTagBuf::new(String::from("en--GB")).is_err());
	assert!(LangTag::new(&[0xff]).is_err());
}

#[test]
fn cow_conversions_preserve_tag_comparison() {
	let borrowed = LangTag::new("en-GB").expect("valid borrowed tag");
	let borrowed = Cow::<LangTag>::from(borrowed);
	assert!(matches!(borrowed, Cow::Borrowed(_)));
	assert!(borrowed.as_ref() == "EN-gb");

	let owned = LangTagBuf::new(String::from("en-GB")).expect("valid owned tag");
	let owned = Cow::<LangTag>::from(owned);
	assert!(matches!(owned, Cow::Owned(_)));
	assert!(owned.as_ref() == "EN-gb");
}
