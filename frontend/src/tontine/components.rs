// src/tontine/components.rs
use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement}; // ← HtmlSelectElement ajouté
use uuid::Uuid;
use std::str::FromStr;
use crate::tontine::types::{CreateTontine, UpdateTontine, Tontine, TontineWithCreator};

fn input_callback(state: &UseStateHandle<String>) -> Callback<InputEvent> {
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.set(input.value());
    })
}

fn select_callback(state: &UseStateHandle<String>) -> Callback<Event> {
    let state = state.clone();
    Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into(); // ← Correct
        state.set(select.value());
    })
}

fn number_callback(state: &UseStateHandle<i32>) -> Callback<InputEvent> {
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(num) = input.value().parse::<i32>() {
            state.set(num);
        }
    })
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreateFormProps {
    pub on_submit: Callback<CreateTontine>,
}

#[function_component(CreateForm)]
pub fn create_form(props: &CreateFormProps) -> Html {
    let name = use_state(|| String::new());
    let description = use_state(|| String::new());
    let amount = use_state(|| String::new());
    let frequency = use_state(|| "monthly".to_string());
    let max_members = use_state(|| 10);
    let created_by = use_state(|| String::new());

    let on_submit = {
        let name = name.clone();
        let description = description.clone();
        let amount = amount.clone();
        let frequency = frequency.clone();
        let max_members = max_members.clone();
        let created_by = created_by.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name_val = (*name).clone();
            let description_val = (*description).clone();
            let amount_val = (*amount).clone();
            let frequency_val = (*frequency).clone();
            let max_members_val = *max_members;
            let created_by_val = (*created_by).clone();

            if name_val.is_empty() || amount_val.is_empty() || created_by_val.is_empty() {
                return;
            }

            if !Uuid::from_str(&created_by_val).is_ok() {
                return;
            }

            on_submit.emit(CreateTontine {
                name: name_val,
                description: if description_val.is_empty() { None } else { Some(description_val) },
                amount_per_member: amount_val,
                frequency: frequency_val,
                max_members: max_members_val,
                created_by: created_by_val,
            });
        })
    };

    html! {
        <div class="tontineformcontainer">
            <h2 class="tontineformtitle">{"1. Créer une Tontine"}</h2>
            <form onsubmit={on_submit} class="tontineform">
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nom *"}</label>
                    <input type="text" value={(*name).clone()} oninput={input_callback(&name)} required=true class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Description"}</label>
                    <input type="text" value={(*description).clone()} oninput={input_callback(&description)} class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Montant par membre *"}</label>
                    <input type="text" value={(*amount).clone()} oninput={input_callback(&amount)} required=true class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Fréquence *"}</label>
                    <select value={(*frequency).clone()} onchange={select_callback(&frequency)} class="tontineformselect">
                        <option value="daily">{"Quotidien"}</option>
                        <option value="weekly">{"Hebdomadaire"}</option>
                        <option value="monthly">{"Mensuel"}</option>
                    </select>
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nombre max de membres *"}</label>
                    <input type="number" value={(*max_members).to_string()} oninput={number_callback(&max_members)} required=true min="2" class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"ID du créateur (UUID) *"}</label>
                    <input type="text" value={(*created_by).clone()} oninput={input_callback(&created_by)} required=true class="tontineforminput" />
                </div>
                <button type="submit" class="tontineformbutton">{"Créer"}</button>
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct IdFormProps {
    pub title: String,
    pub button_text: String,
    pub button_class: String,
    pub on_submit: Callback<Uuid>,
}

#[function_component(IdForm)]
pub fn id_form(props: &IdFormProps) -> Html {
    let id_input = use_state(|| String::new());

    let on_submit = {
        let id_input = id_input.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Ok(id) = Uuid::from_str(&id_input) {
                on_submit.emit(id);
            }
        })
    };

    html! {
        <div class="tontineformcontainer">
            <h2 class="tontineformtitle">{&props.title}</h2>
            <form onsubmit={on_submit} class="tontineform">
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"ID de la tontine *"}</label>
                    <input type="text" value={(*id_input).clone()} oninput={input_callback(&id_input)} required=true class="tontineforminput" />
                </div>
                <button type="submit" class={format!("tontineformbutton {}", props.button_class)}>{&props.button_text}</button>
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct UpdateFormProps {
    pub on_submit: Callback<(Uuid, UpdateTontine)>,
}

#[function_component(UpdateForm)]
pub fn update_form(props: &UpdateFormProps) -> Html {
    let id = use_state(|| String::new());
    let name = use_state(|| String::new());
    let description = use_state(|| String::new());
    let amount = use_state(|| String::new());
    let frequency = use_state(|| "monthly".to_string());
    let max_members = use_state(|| 0);

    let on_submit = {
        let id = id.clone();
        let name = name.clone();
        let description = description.clone();
        let amount = amount.clone();
        let frequency = frequency.clone();
        let max_members = max_members.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Ok(tontine_id) = Uuid::from_str(&id) {
                on_submit.emit((tontine_id, UpdateTontine {
                    name: if (*name).is_empty() { None } else { Some((*name).clone()) },
                    description: if (*description).is_empty() { None } else { Some((*description).clone()) },
                    amount_per_member: if (*amount).is_empty() { None } else { Some((*amount).clone()) },
                    frequency: if (*frequency).is_empty() { None } else { Some((*frequency).clone()) },
                    max_members: if *max_members == 0 { None } else { Some(*max_members) },
                    status: None,
                }));
            }
        })
    };

    html! {
        <div class="tontineformcontainer">
            <h2 class="tontineformtitle">{"3. Mettre à jour une Tontine"}</h2>
            <form onsubmit={on_submit} class="tontineform">
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"ID de la tontine *"}</label>
                    <input type="text" value={(*id).clone()} oninput={input_callback(&id)} required=true class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nouveau nom"}</label>
                    <input type="text" value={(*name).clone()} oninput={input_callback(&name)} class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nouvelle description"}</label>
                    <input type="text" value={(*description).clone()} oninput={input_callback(&description)} class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nouveau montant"}</label>
                    <input type="text" value={(*amount).clone()} oninput={input_callback(&amount)} class="tontineforminput" />
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nouvelle fréquence"}</label>
                    <select value={(*frequency).clone()} onchange={select_callback(&frequency)} class="tontineformselect">
                        <option value="daily">{"Quotidien"}</option>
                        <option value="weekly">{"Hebdomadaire"}</option>
                        <option value="monthly">{"Mensuel"}</option>
                    </select>
                </div>
                <div class="tontineformgroup">
                    <label class="tontineformlabel">{"Nouveau nombre max de membres"}</label>
                    <input type="number" value={(*max_members).to_string()} oninput={number_callback(&max_members)} min="2" class="tontineforminput" />
                </div>
                <button type="submit" class="tontineformbutton">{"Mettre à jour"}</button>
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TontineListProps {
    pub tontines: Vec<Tontine>,
}

#[function_component(TontineList)]
pub fn tontine_list(props: &TontineListProps) -> Html {
    html! {
        <div class="tontinelistcontainer">
            <h2 class="tontinelisttitle">{"7. Liste de Toutes les Tontines"}</h2>
            if props.tontines.is_empty() {
                <p class="tontinelistingempty">{"Aucune tontine trouvée."}</p>
            } else {
                <div class="tontinelist">
                    { for props.tontines.iter().map(|tontine| {
                        html! {
                            <div class="tontineitem">
                                <div class="tontineitemheader">
                                    <h3 class="tontineitemname">{&tontine.name}</h3>
                                    <span class={format!("tontineitemstatus status-{}", tontine.status)}>{&tontine.status}</span>
                                </div>
                                <p class="tontineitemamount">{"Montant: "}{&tontine.amount_per_member}</p>
                                <p class="tontineitemmembers">{"Membres: "}{tontine.max_members}</p>
                                <p class="tontineitemround">{"Round: "}{tontine.current_round}</p>
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TontineDetailsProps {
    pub tontine: TontineWithCreator,
    pub on_close: Callback<MouseEvent>, // ← MouseEvent ici
}

#[function_component(TontineDetails)]
pub fn tontine_details(props: &TontineDetailsProps) -> Html {
    html! {
        <div class="tontinedetailscontainer">
            <h2 class="tontinedetailstitle">{"Détails de la Tontine"}</h2>
            <div class="tontinedetails">
                <p><strong>{"Nom:"}</strong> {&props.tontine.name}</p>
                <p><strong>{"Description:"}</strong> {props.tontine.description.as_ref().unwrap_or(&"Aucune".to_string())}</p>
                <p><strong>{"Montant par membre:"}</strong> {&props.tontine.amount_per_member}</p>
                <p><strong>{"Fréquence:"}</strong> {&props.tontine.frequency}</p>
                <p><strong>{"Membres max:"}</strong> {props.tontine.max_members}</p>
                <p><strong>{"Round actuel:"}</strong> {props.tontine.current_round}</p>
                <p><strong>{"Statut:"}</strong> <span class={format!("tontinedetailsstatus status-{}", props.tontine.status)}>{&props.tontine.status}</span></p>
                <p><strong>{"Créée par:"}</strong> {format!("{} ({})", &props.tontine.creator_name, &props.tontine.creator_email)}</p>
                <p><strong>{"ID Créateur:"}</strong> {props.tontine.created_by.to_string()}</p>
                <p><strong>{"Créée le:"}</strong> {&props.tontine.created_at}</p>
                <p><strong>{"Modifiée le:"}</strong> {&props.tontine.updated_at}</p>
            </div>
            <button onclick={props.on_close.clone()} class="tontinedetailsback">
                {"Fermer"}
            </button>
        </div>
    }
}