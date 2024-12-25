use crate::{model::AllLlmModel, AllLlmsClient};
use alien_seed::attach::with_seed;
use alien_seed::AlienSeed;
use eterned::db::EternerDb;
use expect_test::expect;
use std::{path::PathBuf, sync::Arc};

#[test]
fn all_llms_client_works() {
    let db = &EternerDb::default();
    let cache_dir = PathBuf::from("caches/all_llms_client_works");
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let client = AllLlmsClient::new(&db, tokio_runtime, cache_dir).unwrap();
    let prompt = "Why does Soifon hate Urahara?";
    let seed = AlienSeed::new(0);
    with_seed(seed, || {
        expect![[r#"
            Soifon's disdain for Kisuke Urahara in the anime and manga series "Bleach" can be attributed to a few reasons, primarily connected to her loyalty and sense of duty. Soifon, the captain of the 2nd Division and leader of the Stealth Force, is known for her unwavering loyalty to Yoruichi Shihouin, the former captain of the 2nd Division. When Yoruichi left the Soul Society without any explanation, it caused a deep sense of betrayal and abandonment for Soifon.

            Urahara was closely associated with Yoruichi, as he was the former captain of the 12th Division and played a significant role in the events leading up to Yoruichi's departure. Soifon views Urahara as someone who contributed to Yoruichi's decision to leave and thus holds him responsible for taking Yoruichi away from the Soul Society and, by extension, from her. This, combined with her strong sense of duty and strict adherence to the rules of the Soul Society, fuels her resentment towards Urahara.

            Additionally, Soifon may also harbor suspicion towards Urahara because of his unconventional methods and his involvement in the controversial creation of the Hōgyoku, which are seen as acts that go against the very fabric of the Soul Society's laws and order. This adds another layer of mistrust and animosity from her perspective."#]]
    .assert_eq(
        &client
            .generate_text(AllLlmModel::GPT4O, prompt.to_string())
            .unwrap(),
    );
        expect![[r#"
            Soifon's hatred for Kisuke Urahara isn't a simple, singular reason but rather a culmination of several factors:

            * **Their history at the Academy:**  While not explicitly detailed, it's heavily implied that Urahara, even during their time at the Shin'o Academy, displayed a playful and somewhat irreverent attitude toward rules and authority, something Soifon, a highly disciplined and by-the-book individual, would find deeply irritating.  His unconventional methods and disregard for tradition likely clashed intensely with her rigid personality.

            * **Urahara's perceived arrogance and trickery:** Urahara consistently outwits Soifon, often using trickery and deception.  This isn't just a matter of skill; it feels personal to Soifon because it undermines her sense of competence and control. His seeming nonchalance in the face of her fury only fuels her anger.

            * **His involvement in events leading to the Soul Society's turmoil:**  Although Urahara wasn't solely responsible for the events leading to the Arrancar invasion and the intrusion of Aizen, his past actions and apparent involvement contribute to the instability that Soifon fights so hard to prevent. This is a significant source of her resentment because it goes against everything she believes in regarding order and stability in the Soul Society.

            * **Personal rivalry and clash of personalities:** At its core, Soifon's hatred is likely fueled by a fundamental personality clash.  Urahara's relaxed, almost teasing attitude is the antithesis of Soifon's strict, serious demeanor.  Their rivalry is less about specific events and more about their inherently opposing natures.  The constant clashes only serve to deepen her resentment.

            In short, Soifon's hatred is a complex emotion built upon a foundation of personality conflict, fueled by Urahara's unorthodox methods, past actions, and continuous outsmarting of her.  It's a mixture of professional frustration, personal animosity, and a deep-seated distrust of his character.
        "#]]
    .assert_eq(
        &client
            .generate_text(AllLlmModel::GEMINI_1_5_FLASH, prompt.to_string())
            .unwrap(),
    );
    })
}
