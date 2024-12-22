use crate::{model::AllLlmModel, AllLlmsClient};
use eterned::db::EternerDb;
use expect_test::expect;
use std::path::PathBuf;

#[test]
fn all_llms_client_works() {
    let db = &EternerDb::default();
    let cache_dir = PathBuf::from("caches/all_llms_client_works");
    let client = AllLlmsClient::new(&db, cache_dir).unwrap();
    let prompt = "Why does Soifon hate Urahara?";
    expect![[r#"
        Soifon, a character from the anime and manga series "Bleach," has a complicated relationship with Kisuke Urahara, which can be characterized by a certain level of animosity. This primarily stems from her loyalty to Yoruichi Shihouin, who was once the commander of the Stealth Force and captain of the 2nd Division, roles that Soifon later assumes. Yoruichi and Urahara share a close friendship and history, which includes Yoruichi assisting Urahara when he was exiled from the Soul Society.

        Soifon resents Urahara because she feels his actions led to Yoruichi abandoning the Soul Society to help him. This abandonment was a source of great personal pain and betrayal for Soifon, as she idolized Yoruichi and was deeply devoted to her. Therefore, Soifon's distrust and dislike of Urahara are tied to her perception that he took Yoruichi away from her and caused a rift in her loyalty and sense of duty. At the core, Soifon's feelings towards Urahara are intertwined with her unresolved emotions regarding Yoruichi's departure and the changes it brought to her life."#]]
    .assert_eq(
        &client
            .generate_text(AllLlmModel::GPT4O, prompt.to_string())
            .unwrap(),
    );
    expect![[r#"
        Soifon's hatred for Kisuke Urahara stems from a combination of factors, primarily revolving around perceived disrespect and a history of humiliating defeats:

        * **Urahara's constant teasing and disrespect:** Urahara consistently treats Soifon with a playful, almost mocking demeanor, often ignoring or belittling her authority and seriousness.  This clashes sharply with Soifon's strict personality and high rank, fueling her resentment. He frequently uses her short stature and volatile temper as a source of amusement,  pushing her buttons deliberately.

        * **Strategic outmaneuvering:**  Urahara's superior intellect and strategic thinking consistently put him one step ahead of Soifon, often leading to her defeat or humiliation in their encounters. These defeats aren't just tactical failures; they highlight her perceived inferiority in intellect and planning compared to Urahara.  He often seems to anticipate her actions and counters them effortlessly.

        * **Past events (unspecified):** While not explicitly detailed, there are hints of past interactions between them that further fueled her animosity. The exact nature of these events isn't fully explored in the series, but they contribute to the ongoing tension and hatred.

        Essentially, Soifon's hatred is a complex mix of personal frustration (due to Urahara's personality), professional rivalry (stemming from his strategic superiority), and possibly undisclosed past grievances. It's a combination of being consistently outmatched and consistently insulted, both professionally and personally.
    "#]]
    .assert_eq(
        &client
            .generate_text(AllLlmModel::GEMINI_1_5_FLASH, prompt.to_string())
            .unwrap(),
    );
}
