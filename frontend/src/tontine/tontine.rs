// src/tontine/tontine.rs
use yew::prelude::*;
use wasm_bindgen_futures;
use web_sys;
use uuid::Uuid;
use crate::tontine::types::{CreateTontine, UpdateTontine, Tontine, TontineWithCreator};
use crate::tontine::api;
use crate::tontine::components::{CreateForm, IdForm, UpdateForm, TontineList, TontineDetails};

#[function_component(TontinePage)]
pub fn tontine_page() -> Html {
    let tontines = use_state(Vec::<Tontine>::new);
    let selected_tontine = use_state(|| Option::<TontineWithCreator>::None);
    let message = use_state(|| String::new());

    {
        let tontines = tontines.clone();
        let message = message.clone();
        use_effect_with(
            (),
            move |_| {
                let tontines = tontines.clone();
                let message = message.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api::fetch_all_tontines().await {
                        Ok(list) => tontines.set(list),
                        Err(e) => message.set(e),
                    }
                });
                || ()
            },
        );
    }

    let on_create = {
        let tontines = tontines.clone();
        let message = message.clone();
        Callback::from(move |tontine_data: CreateTontine| {
            let tontines = tontines.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_tontine(tontine_data).await {
                    Ok(()) => {
                        message.set("✅ Tontine créée avec succès !".to_string());
                        match api::fetch_all_tontines().await {
                            Ok(list) => tontines.set(list),
                            Err(e) => message.set(e),
                        }
                    }
                    Err(e) => message.set(e),
                }
            });
        })
    };

    // Créer DEUX callbacks distincts pour les deux utilisations
    let on_get_by_id_1 = {
        let selected_tontine = selected_tontine.clone();
        let message = message.clone();
        Callback::from(move |id: Uuid| {
            let selected_tontine = selected_tontine.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_tontine_details(id).await {
                    Ok(tontine) => selected_tontine.set(Some(tontine)),
                    Err(e) => message.set(e),
                }
            });
        })
    };

    let on_get_by_id_2 = {
        let selected_tontine = selected_tontine.clone();
        let message = message.clone();
        Callback::from(move |id: Uuid| {
            let selected_tontine = selected_tontine.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_tontine_details(id).await {
                    Ok(tontine) => selected_tontine.set(Some(tontine)),
                    Err(e) => message.set(e),
                }
            });
        })
    };

    let on_update = {
        let tontines = tontines.clone();
        let message = message.clone();
        Callback::from(move |(id, tontine_data): (Uuid, UpdateTontine)| {
            let tontines = tontines.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::update_tontine(id, tontine_data).await {
                    Ok(()) => {
                        message.set("✅ Tontine mise à jour !".to_string());
                        match api::fetch_all_tontines().await {
                            Ok(list) => tontines.set(list),
                            Err(e) => message.set(e),
                        }
                    }
                    Err(e) => message.set(e),
                }
            });
        })
    };

    let on_delete = {
        let tontines = tontines.clone();
        let message = message.clone();
        Callback::from(move |id: Uuid| {
            if let Some(window) = web_sys::window() {
                if let Ok(confirmed) = window.confirm_with_message("Êtes-vous sûr de vouloir supprimer cette tontine ?") {
                    if confirmed {
                        let tontines = tontines.clone();
                        let message = message.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match api::delete_tontine(id).await {
                                Ok(()) => {
                                    message.set("✅ Tontine supprimée avec succès !".to_string());
                                    match api::fetch_all_tontines().await {
                                        Ok(list) => tontines.set(list),
                                        Err(e) => message.set(e),
                                    }
                                }
                                Err(e) => message.set(e),
                            }
                        });
                    }
                }
            }
        })
    };

    let on_increment = {
        let tontines = tontines.clone();
        let message = message.clone();
        Callback::from(move |id: Uuid| {
            let tontines = tontines.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::increment_round(id).await {
                    Ok(()) => {
                        message.set("✅ Round incrémenté !".to_string());
                        match api::fetch_all_tontines().await {
                            Ok(list) => tontines.set(list),
                            Err(e) => message.set(e),
                        }
                    }
                    Err(e) => message.set(e),
                }
            });
        })
    };

    let on_close_details = {
        let selected_tontine = selected_tontine.clone();
        Callback::from(move |_| selected_tontine.set(None))
    };

    html! {
        <div class="tontinepage">
            <h1 class="tontinetitle">{"Gestion des Tontines - Interface Complète"}</h1>

            if !(*message).is_empty() {
                <p class="tontinemessage">{(*message).clone()}</p>
            }

            <CreateForm on_submit={on_create} />
            <IdForm 
                title={"2. Lire une Tontine par ID"}
                button_text={"Charger"}
                button_class={"details"}
                on_submit={on_get_by_id_1} // ← Premier usage
            />
            <UpdateForm on_submit={on_update} />
            <IdForm 
                title={"4. Supprimer une Tontine"}
                button_text={"Supprimer"}
                button_class={"delete"}
                on_submit={on_delete}
            />
            <IdForm 
                title={"5. Incrémenter le Round"}
                button_text={"+ Round"}
                button_class={"round"}
                on_submit={on_increment}
            />
            <IdForm 
                title={"6. Voir Détails + Créateur"}
                button_text={"Voir Détails"}
                button_class={"details"}
                on_submit={on_get_by_id_2} // ← Deuxième usage
            />

            <TontineList tontines={(*tontines).clone()} />

            if let Some(tontine) = &*selected_tontine {
                <TontineDetails 
                    tontine={tontine.clone()} 
                    on_close={Callback::from(move |_: MouseEvent| on_close_details.emit(()))} 
                />
            }
        </div>
    }
}

// ✅ Export direct de la fonction (pas de `tontine_page`)
