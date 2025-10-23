// src/form.rs
use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures;

#[function_component(Form)]
pub fn form() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let phone = use_state(|| String::new());
    let message = use_state(|| String::new());

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let full_name = full_name.clone();
        let phone = phone.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            message.set("".to_string());

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let full_name_val = (*full_name).clone();
            let phone_val = (*phone).clone();
            let message_async = message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://localhost:8080/api/users";
                let body = serde_json::json!({
                    "email": email_val,
                    "password": password_val,
                    "full_name": full_name_val,
                    "phone": phone_val,
                });

                let request = Request::post(url)
                    .header("Content-Type", "application/json");

                let req = match request.json(&body) {
                    Ok(r) => r,
                    Err(_) => {
                        message_async.set("❌ Erreur : impossible de préparer la requête.".to_string());
                        return;
                    }
                };

                match req.send().await {
                    Ok(response) => {
                        if response.status() == 200 || response.status() == 201 {
                            message_async.set("✅ Utilisateur créé avec succès !".to_string());
                        } else {
                            let error_text = response.text().await.unwrap_or_else(|_| "Erreur inconnue".to_string());
                            message_async.set(format!("❌ Erreur {} : {}", response.status(), error_text));
                        }
                    }
                    Err(_) => {
                        message_async.set("❌ Impossible de joindre le serveur backend.".to_string());
                    }
                }
            });
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_full_name_input = {
        let full_name = full_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            full_name.set(input.value());
        })
    };

    let on_phone_input = {
        let phone = phone.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            phone.set(input.value());
        })
    };

    html! {
        <div class="formcontainer">
            <h2 class="formtitle">{"Créer un compte"}</h2>

            if !(*message).is_empty() {
                <p class={if (*message).contains("✅") { "formsuccess" } else { "formerror" }}>
                    {(*message).clone()}
                </p>
            }

            <form onsubmit={onsubmit} class="form">
                <div class="formgroup">
                    <label for="email" class="formlabel">{"Email *"}</label>
                    <input
                        type="email"
                        id="email"
                        value={(*email).clone()}
                        oninput={on_email_input}
                        required=true
                        class="forminput"
                    />
                </div>

                <div class="formgroup">
                    <label for="password" class="formlabel">{"Mot de passe * (min. 6 caractères)"}</label>
                    <input
                        type="password"
                        id="password"
                        value={(*password).clone()}
                        oninput={on_password_input}
                        required=true
                        class="forminput"
                    />
                </div>

                <div class="formgroup">
                    <label for="full_name" class="formlabel">{"Nom complet *"}</label>
                    <input
                        type="text"
                        id="full_name"
                        value={(*full_name).clone()}
                        oninput={on_full_name_input}
                        required=true
                        class="forminput"
                    />
                </div>

                <div class="formgroup">
                    <label for="phone" class="formlabel">{"Téléphone *"}</label>
                    <input
                        type="tel"
                        id="phone"
                        value={(*phone).clone()}
                        oninput={on_phone_input}
                        required=true
                        class="forminput"
                    />
                </div>

                <button type="submit" class="formbutton">{"S'inscrire"}</button>
            </form>
        </div>
    }
}