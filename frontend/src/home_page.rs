// src/components/home_page.rs
use yew::prelude::*;
use crate::form::Form;
use crate::login;


#[function_component(LoginButton)]
fn login_button() -> Html {
    let show_login = use_state(|| false);
    let toggle = {
        let show_login = show_login.clone();
        Callback::from(move |_| show_login.set(!*show_login))
    };

    html! {
        <>
            <button class="loginbutton" onclick={toggle}>{"Connexion"}</button>
            if *show_login {
                <login::Login />
            }
        </>
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // État pour gérer la vue : "home" ou "register"
    let current_view = use_state(|| "home".to_string());

    let is_logged_in = use_state(|| {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|storage| storage.get_item("auth_token").ok())
            .flatten()
            .is_some()
    });

    let on_auth_change = {
        let is_logged_in = is_logged_in.clone();
        Callback::from(move |_| {
            let logged_in = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|storage| storage.get_item("auth_token").ok())
                .flatten()
                .is_some();
            is_logged_in.set(logged_in);
        })
    };

    let on_logout = {
        let on_auth_change = on_auth_change.clone();
        Callback::from(move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(maybe_storage) = window.local_storage() {
                    if let Some(storage) = maybe_storage {
                        let _ = storage.remove_item("auth_token");
                    }
                }
            }
            on_auth_change.emit(());
        })
    };

    // Passer à la vue formulaire
    let go_to_register = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set("register".to_string());
        })
    };

    // Retour à la homepage
    let go_to_home = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set("home".to_string());
        })
    };

    // Affichage conditionnel
    if *current_view == "register" {
        // Vue formulaire seul
        html! {
            <div class="page">
                <button class="backbutton" onclick={go_to_home}>{"← Retour"}</button>
                <Form />
            </div>
        }
    } else {
        // Vue homepage
        html! {
            <ContextProvider<Callback<()>> context={on_auth_change}>
                <div class="page">
                    <div class="titrecontainer">
                        <h1 class="titretext">{"TONTINE EN LIGNE"}</h1>
                    </div>

                    <div class="inscriptioncontainer">
                        <button class="inscriptionbutton" onclick={go_to_register}>
                            {"S'inscrire"}
                        </button>
                    </div>

                    <div class="logincontainer">
                        if *is_logged_in {
                            <button class="logoutbutton" onclick={on_logout}>{"Déconnexion"}</button>
                        } else {
                            <LoginButton />
                        }
                    </div>
                </div>
            </ContextProvider<Callback<()>>>
        }
    }
}