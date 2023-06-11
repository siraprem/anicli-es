extern crate getopts;
use dialoguer::Input;

mod structs;
use structs::{Anime, AnimeEpisode, AnimeEpisodeView};

mod mainfunctions;
use mainfunctions::{search_query, query_results, choose_anime, choose_episode, get_episodes, color_menu, mpv};

fn main() {
    loop {
        let query: String = Input::new()
            .with_prompt(
                color_menu("Buscar animé")
            )
            .interact()
            .unwrap();

        let animes: Vec<Anime> = query_results(
            search_query(query.trim().to_string())
        );

        if animes.is_empty() {
            println!("No se encontró ningun animé con ese nombre.");
            continue;
        }

        let anime_seleccion: usize = choose_anime(&animes) as usize;

        let episodes: Vec<String> = get_episodes(animes[anime_seleccion].link.to_string());

        if episodes.is_empty() {
            println!("No se encontrarón episodios, intenta con otro animé.");
            continue;
        }

        mpv(
            &animes[anime_seleccion],
            &episodes,
            choose_episode(&episodes),
            false
       )
    }
}
