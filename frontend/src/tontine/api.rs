// src/tontine/api.rs
use gloo_net::http::Request;
use wasm_bindgen_futures;
use crate::tontine::types::{CreateTontine, UpdateTontine, Tontine, TontineWithCreator};
use uuid::Uuid;

pub async fn fetch_all_tontines() -> Result<Vec<Tontine>, String> {
    match Request::get("http://localhost:8080/api/tontines").send().await {
        Ok(resp) => {
            if resp.status() == 200 {
                resp.json::<Vec<Tontine>>().await.map_err(|e| e.to_string())
            } else {
                Err(format!("Erreur {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn fetch_tontine_details(id: Uuid) -> Result<TontineWithCreator, String> {
    match Request::get(&format!("http://localhost:8080/api/tontines/{}/details", id)).send().await {
        Ok(resp) => {
            if resp.status() == 200 {
                resp.json::<TontineWithCreator>().await.map_err(|e| e.to_string())
            } else {
                Err("Tontine non trouvée".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_tontine(data: CreateTontine) -> Result<(), String> {
    match Request::post("http://localhost:8080/api/tontines")
        .header("Content-Type", "application/json")
        .json(&data)
        .expect("Échec sérialisation")
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status() == 201 {
                Ok(())
            } else {
                Err(format!("Erreur {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn update_tontine(id: Uuid, data: UpdateTontine) -> Result<(), String> {
    match Request::put(&format!("http://localhost:8080/api/tontines/{}", id))
        .header("Content-Type", "application/json")
        .json(&data)
        .expect("Échec sérialisation")
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status() == 200 {
                Ok(())
            } else {
                Err("Erreur lors de la mise à jour".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete_tontine(id: Uuid) -> Result<(), String> {
    match Request::delete(&format!("http://localhost:8080/api/tontines/{}", id))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status() == 204 {
                Ok(())
            } else {
                Err("Erreur lors de la suppression".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn increment_round(id: Uuid) -> Result<(), String> {
    match Request::put(&format!("http://localhost:8080/api/tontines/{}/increment-round", id))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status() == 200 {
                Ok(())
            } else {
                Err("Erreur lors de l'incrémentation".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}