use std::collections::{BTreeMap, HashMap};
use xxhash_rust::xxh3::xxh3_64;
fn main() {
    println!("Hello, world!");
    consistent_hashing()
}

fn generate_arc() -> BTreeMap<u64, String> {
    let amount_of_virtual_nodes = 100;
    let servers = vec!["Serveur A", "Serveur B", "Serveur C", "Serveur D"];

    servers
        .iter()
        .flat_map(|&server| {
            let server_formatted = (0..amount_of_virtual_nodes).map(move |i| {
                let server_virtual = format!("{}-{}", server, i);
                let formatted_sever = server_virtual.as_bytes();
                return (xxh3_64(formatted_sever), server.to_string());
            });
            return server_formatted;
        })
        .collect()
}

fn get_server(arc: &BTreeMap<u64, String>, key: &str) -> String {
    let hash = xxh3_64(key.as_bytes());
    let mut iterator = arc.range(hash..);
    match iterator.next() {
        Some((_position, server)) => server.clone(),
        None => {
            let (_position, server) = arc.iter().next().unwrap();
            server.clone()
        }
    }
}

fn consistent_hashing() {
    let anneau = generate_arc();

    // Notre liste de fausses données en entrée
    let requetes_entrantes = vec![
        "video_vacances.mp4",
        "avatar_user_42.png",
        "config_system.json",
        "index.html",
        "database_backup.sql",
        "image_chat_1.jpg",
        "image_chat_2.jpg",
        "fnrjfrjn.jpg",
    ];

    println!("--- Début du Routage ---");

    for donnee in requetes_entrantes {
        let serveur_cible = get_server(&anneau, donnee);

        // On calcule le hash juste pour l'affichage et comprendre
        let h = xxh3_64(donnee.as_bytes());

        println!("Donnée: '{}' (Hash: {})", donnee, h);
        println!("  => Envoyée vers: {}", serveur_cible);
        println!("-----------------------");
    }
}
