// src/auth/login.rs
use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures;

#[function_component(Login)]
pub fn login() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let message = use_state(|| String::new());
    let on_login_success = use_context::<Callback<()>>();

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();
        let on_login_success = on_login_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            message.set("".to_string());

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let message_async = message.clone();
            let on_login_success = on_login_success.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let body = serde_json::json!({
                    "email": email_val,
                    "password": password_val
                });

                let req = Request::post("http://localhost:8080/api/auth/login")
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .expect("Échec sérialisation");

                match req.send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            let auth: serde_json::Value = response.json().await.unwrap();
                            let token = auth["access_token"].as_str().unwrap_or("");

                            if let Some(window) = web_sys::window() {
                                if let Ok(maybe_storage) = window.local_storage() {
                                    if let Some(storage) = maybe_storage {
                                        let _ = storage.set_item("auth_token", token);
                                    }
                                }
                            }

                            message_async.set("✅ Connecté !".to_string());
                            if let Some(cb) = on_login_success {
                                cb.emit(());
                            }
                        } else {
                            let error = response.text().await.unwrap_or("Erreur inconnue".to_string());
                            message_async.set(format!("❌ {}", error));
                        }
                    }
                    Err(_) => {
                        message_async.set("❌ Impossible de joindre le serveur.".to_string());
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

    html! {
        <div class="loginformcontainer">
            <h2 class="logintitle">{"Connexion"}</h2>
            if !(*message).is_empty() {
                <p class="loginmessage">{(*message).clone()}</p>
            }
            <form onsubmit={onsubmit} class="loginform">
                <div class="loginfield">
                    <label class="loginlabel">{"Email"}</label><br/>
                    <input 
                        type="email" 
                        value={(*email).clone()} 
                        oninput={on_email_input} 
                        required=true 
                        class="logininput"
                    />
                </div>
                <div class="loginfield">
                    <label class="loginlabel">{"Mot de passe"}</label><br/>
                    <input 
                        type="password" 
                        value={(*password).clone()} 
                        oninput={on_password_input} 
                        required=true 
                        class="logininput"
                    />
                </div>
                <button type="submit" class="loginbuttonsubmit">{"Se connecter"}</button>
            </form>
        </div>
    }
}