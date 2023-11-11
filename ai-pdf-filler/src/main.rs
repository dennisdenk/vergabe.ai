// for pdf_forms
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derive_error;

use std::env::args;
use std::error::Error;

mod pdf_forms;
use self::pdf_forms::{Form, FieldType};

use chatgpt::prelude::ChatGPT;
use chatgpt::types::Role;

const CHATGPT_SYSTEM_MESSAGE: &'static str = r#"
Du bist VergabeGPT. Dein Ziel ist es dem Nutzer bei der Ausfüllung von Vergabeunterlagen zu helfen.
Du bekommst immer einen von zwei Befehlen vom Nutzer:
INFO <name> <text ...>
FILL <name>
Der INFO Befehl gibt dir Informationen über den Nutzer. Hierrauf musst du nicht antworten.
Bei dem FILL Befehl sollst du die erhaltenen Informationen Nutzen um ein Textfeld auszufüllen.

Du antwortest immer mit einem von zwei Antworten
OK
ENTER <text ...>
MISSING <name> <description>
Wenn du einen INFO Befehl bekommst antwortest du mit OK
Wenn du einen FILL Befehl bekommst den du beantworten kannst antwortest du mit ENTER 
Wenn dir Infos fehlen um FILL korrekt zu beantworten antwortest du mit MISSING. Damit fragst du den Nutzer nach einer Info mit Namen und Beschreibung welche Information du brauchst.

Oft heißen die Felder in FILL anders als du sie zuvor in INFO bekommen hast.
INFO sind vom Menschen gegebene Informationen. FILL sind Textfelder einer PDF Datei.
Deine Aufgabe ist es anhand der gegebenen INFOs so gut möglich die Felder aus FILL auszufüllen und möglichst selten MISSING zu benutzen.

Es ist immer in Befehl pro Zeile.
"#;

const CHATGPT_USER_INFORMATION: &'static str = r#"
INFO mitarbeiter 7
INFO name Huber Heizungsbau GmbH
INFO ceo Hans Huber
INFO gruendung 17.10.2007
INFO ort Ingolstadt
INFO datum-heute 10.11.2023
INFO kompetenzen Heizungsbau, Rohrverlegung, Wärmepumpem, Gasheizungen, Ölheizungen, Dämmung, Gerüstbau
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = args().nth(1).ok_or("no ChatGPT key given")?;
    let filename = args().nth(2).ok_or("no PDF filename given")?;

    let client = ChatGPT::new(key)?;
    let mut convo = client.new_conversation();
    let mut form = Form::load(filename)?;

    convo.send_role_message(Role::System, CHATGPT_SYSTEM_MESSAGE).await?;
    let r = convo.send_role_message(Role::User, CHATGPT_USER_INFORMATION).await?;
    println!("r = {:?}", r.message().content);

    for i in 0..form.len() {
        if form.get_type(i) != FieldType::Text {
            continue;
        }

        let maybe_text = form.get_description(i).or(form.get_name(i));
        if let Some(text) = maybe_text {
            let resp = convo.send_message(format!("FILL {}", text)).await?;
            let msg = resp.message().content.clone();
            if msg.starts_with("ENTER") {
                form.set_text(i, msg.as_str()[6..].to_string())?;
            }
            else if msg.starts_with("MISSING") {
                println!("Missing information! {:?}", msg);
            }
        }
    }

    form.save("./filled.pdf")?;

    Ok(())
}
