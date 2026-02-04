use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::sleep;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Lancement du navigateur
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head() 
            .build()?
    ).await?;

    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            let _ = h;
        }
    });

    println!("🌍 Ouverture de Google Flights...");
    let page = browser.new_page("https://www.google.com/travel/flights/search?tfs=CBwQAhooEgoyMDI2LTA1LTA0agwIAhIIL20vMDVxdGpyDAgDEggvbS8waHNxZhooEgoyMDI2LTA1LTI1agwIAxIIL20vMGhzcWZyDAgCEggvbS8wNXF0akABSAFwAYIBCwj___________8BmAEB&hl=fr&gl=FR").await?;
    
    // 2. Automatisation du clic sur les Cookies
    sleep(Duration::from_secs(4)).await;
    
    // On cherche tous les boutons pour trouver celui qui contient le texte d'acceptation
    let buttons = page.find_elements("button").await?;
    let mut clicked = false;

    for btn in buttons {
        if let Ok(Some(txt)) = btn.inner_text().await {
            let t = txt.to_lowercase();
            if t.contains("accepter") || t.contains("accept all") || t.contains("agree") {
                btn.click().await?;
                println!("✅ Cookies acceptés automatiquement par le bot !");
                clicked = true;
                break;
            }
        }
    }

    if !clicked {
        println!("⚠️ Bouton cookies non trouvé, vérification visuelle nécessaire.");
    }

    // 3. Attente du chargement dynamique des prix
    println!("⏳ Analyse des résultats en cours...");
    sleep(Duration::from_secs(8)).await;

    // 4. Extraction propre sans doublons
    let mut unique_prices = HashSet::new();
    let elements = page.find_elements("span").await?;

    println!("\n--- RÉSULTATS DES VOLS ---");
    for el in elements {
        if let Ok(Some(txt)) = el.inner_text().await {
            let clean_txt = txt.trim().to_string();
            
            // Filtre : doit contenir €, être court, et ne pas être déjà vu
            if clean_txt.contains('€') && clean_txt.len() < 12 {
                if unique_prices.insert(clean_txt.clone()) {
                    println!("💰 Prix trouvé : {}", clean_txt);
                }
            }
        }
    }

    if unique_prices.is_empty() {
        println!("❌ Aucun prix n'a pu être extrait. Tentative de capture d'écran...");
        let screenshot_bytes = page.screenshot(chromiumoxide::page::ScreenshotParams::default()).await?;
        std::fs::write("debug.png", screenshot_bytes)?;
    }

    // 5. Fin
    println!("\nTerminé. Fermeture dans 10s.");
    sleep(Duration::from_secs(10)).await;
    browser.close().await?;

    Ok(())
}