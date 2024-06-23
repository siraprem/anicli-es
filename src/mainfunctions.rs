use dialoguer::Input;
use regex::Regex;
use std::ops::Index;
use std::process::Command;
use json;

mod auxfunctions;

pub fn color_menu(text: &str) -> String {
    format!("\x1b[93m{}\x1b[0m", text)
}

pub fn color_category(category: &String) -> String {
    if category == "Anime" {
        return format!("\x1b[96m{}\x1b[0m", category);
    } else if category == "Película" {
        return format!("\x1b[91m{}\x1b[0m", category);
    } else if category == "OVA" {
        return format!("\x1b[38;5;208m{}\x1b[0m", category);
    } else if category == "Especial" {
        return format!("\x1b[38;5;13m{}\x1b[0m", category);
    }

    format!("{}", category)
}

// recibe un largo(usize) en el cual elegir un índice y un String que dice a que corresponde el
// índice a elegir y entrega el input del usuario como entero i32
pub fn choose_index(lenght: usize, que: &str) -> u16 {
    if lenght == 1 {
        return 0
    }

    loop {
        let index: String = Input::new()
            .with_prompt(
                color_menu(
                    format!("Elige un {}", que).as_str()
                )
            )
            .interact()
            .unwrap();

        let index: u16 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Porfavor escribe un número.");
                continue;
            }
        };

        if index > lenght.try_into().unwrap() {
            println!("El índice seleccionado es invalido.");
            continue;
        } else if index < 1 {
            println!("El índice seleccionado es invalido.");
            continue;
        } else {
            return index - 1;
        }
    }
}

// recibe un String como busqueda y devuelve el código fuente del resultado
pub fn search_query(query: String) -> String {
    let url = format!("https://www3.animeflv.net/browse?q={}", query);

    auxfunctions::get_source(url).unwrap()
}

pub fn query_results(source: String) -> Vec<super::Anime> {
    let mut animes: Vec<super::Anime> = vec![];

    let get_titles = Regex::new("<h3 class=\"Title\">([^<]*)</h3>").unwrap();
    let get_categories = Regex::new("<p><span class=\"Type [^<]*\">([^<]*)</span>").unwrap();
    let get_links = Regex::new("<a class=\"Button Vrnmlk\" href=\"([^<]*)\">VER ANIME</a>").unwrap();

    let mut titles: Vec<String> = Vec::new();
    let mut categories: Vec<String> = Vec::new();
    let mut links: Vec<String> = Vec::new();

    for t in get_titles.captures_iter(&source) {
        titles.push(
            t.get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("&#039;", "'")
            .replace("&amp;", "&")
        );
    }

    for c in get_categories.captures_iter(&source) {
        categories.push(
            c.get(1)
            .unwrap()
            .as_str()
            .to_string()
        );
    }

    for l in get_links.captures_iter(&source) {
        links.push(
            format!("https://www3.animeflv.net{}",  l.get(1).unwrap().as_str())
        );
    }

    for (i, title) in titles.iter().enumerate() {
        animes.push(super::Anime {
            title   : title.to_string(),
            category: categories[i].to_string(),
            link    : links[i].to_string(),
        })
    }

    animes
}

// printea los animés encontrados a la pantalla con indices a la izquierda
pub fn choose_anime(animelist: &Vec<super::Anime>) -> u16 {
    if animelist.len() == 1 {
        return 0;
    } else {
        for (i, anime) in animelist.iter().enumerate() {
            println!("[{}] {} {}", i+1, color_category(&anime.category), anime.title);
        }
    }

    let animes: String = format!("animé [1-{}]", animelist.len());
    choose_index(animelist.len(), animes.as_str())
}

pub fn choose_episode(episodes: &Vec<String>) -> u16 {
    choose_index(episodes.len(), format!("episodio [1-{}]", episodes.len()).as_str())
}

fn choose_lang(episode_lamgs: &Vec<super::AnimeEpisode>) -> u16 {
    if episode_lamgs.len() == 1 {
        return 0;
    } else {
        for (i, item) in episode_lamgs.iter().enumerate() {
            println!("[{}] {}", i+1, item.lang);
        }
    }

    let langs: String = format!("lenguaje [1-{}]", episode_lamgs.len());
    choose_index(episode_lamgs.len(), langs.as_str())
}

fn choose_server(servers: &Vec<super::AnimeEpisodeView>) -> u16 {
    if servers.len() == 1 {
        return 0;
    } else {
        for (i, server) in servers.iter().enumerate() {
            println!("[{}] {}", i+1, server.title);
        }
    }

    let server_menu: String = format!("Servidor [1-{}]", servers.len());
    choose_index(servers.len(), server_menu.as_str())
}

pub fn get_episodes(url: String) -> Vec<String> {
    let source = auxfunctions::get_source(url).unwrap();
    let url = String::from("https://www3.animeflv.net");

    let mut episodes: Vec<String> = vec![];

    let get_episodes = Regex::new("\\[([0-9]+),").unwrap();

    // Obtener parte del path de la url
    let get_episodes_path = Regex::new("var anime_info = (.*);").unwrap().captures(&source).unwrap().get(1).unwrap().as_str();
    let episodes_path_vec = json::parse(get_episodes_path).unwrap();
    let episodes_path = episodes_path_vec[2].to_string();

    for n in get_episodes.captures_iter(&source) {
        episodes.push(
            format!("{0}/ver/{1}-{2}", url, episodes_path, n.get(1).unwrap().as_str())
        );
    }

    episodes.reverse();

    episodes
}

pub fn episode_link_scrapper(url: String) -> Vec<super::AnimeEpisode> {
    let episode_source: String = auxfunctions::get_source(url).unwrap();

    let get_episode_servers = Regex::new("var videos = (.*);").unwrap().captures(&episode_source).unwrap();
    let episode_servers = get_episode_servers.get(1).unwrap().as_str();
    let scrapper_list = json::parse(episode_servers).unwrap();

    let mut list: Vec<super::AnimeEpisode> = vec![];

    for (lang, servers) in scrapper_list.entries() {
        if servers.is_array() && servers.len() > 0 {
            let mut anime_episode = super::AnimeEpisode {
                lang: lang.to_string(),
                servers: vec![],
            };

            for i in servers.members() {
                if ["YourUpload", "Maru", "Stape"].contains(&i["title"].to_string().as_str()) {
                    anime_episode.servers.push(
                        super::AnimeEpisodeView {
                            title: i["title"].to_string(),
                            url: if ! i["url"].is_null() { i["url"].to_string() } else { "".to_string() },
                            code: i["code"].to_string(),
                        }
                    )
                }
            }

            list.push(anime_episode);
        }
    }

    list
}

fn getargs(server: &super::AnimeEpisodeView) -> String {
    if server.title == "YourUpload" {
        return format!("mpv \"{}\"", server.code);
    }
    if server.title == "Maru" {
        return format!("mpv \"{}\"", server.code);
    }
//    if server.title == "Stape" {
//        let source = auxfunctions::get_source(server.url.clone()).unwrap();
////            println!("source: {:#?}", source);
////            let get_path_video_auth = Regex::new("streamtape\\.com\\/get_video\\?(.*)<\\/div>\\n<div id").unwrap().captures(&source).unwrap();
////            let url = format!("https://tapewithadblock.com/get_video?{}", get_path_video_auth.get(1).unwrap().as_str());
//        let get_path_video_auth = Regex::new("streamtape\\.com\\/get_video\\?(.*)<\\/div>\\n<div id").unwrap().captures(&source).unwrap();
//        let url = format!("https://streamtape.com/get_video?{}", get_path_video_auth.get(1).unwrap().as_str());
//        println!("get_path_video_auth: {:#?}", url);
//        return format!("iina -no-stdin --keep-running \"{}\"", url);
//    }

    String::new()
}

pub fn mpv(anime: &super::Anime, episodes: &Vec<String>, episode_selection: u16, failed_server: bool) {
    let episode_langs: Vec<super::AnimeEpisode> = episode_link_scrapper(episodes[episode_selection as usize].to_string());

    let episode_lang_selection: u16 = choose_lang(&episode_langs);

    let episode = episode_langs.index(episode_lang_selection as usize);

    let episode_server_selection: u16 = if failed_server { choose_server(&episode.servers) } else { 0 };

    let server = episode.servers.index(episode_server_selection as usize);

    let args: String = getargs(&server);

    if !args.is_empty() {
        let mpv_command = Command::new("sh")
            .arg("-c")
            .arg(&args)
            .spawn();

        drop(mpv_command);

        if episode.servers.len() == 1 {
            println!("\nViendo \"{}\".\nServidor: {}.\n", anime.title, server.title);
        } else {
            println!("\nViendo \"{}\", episodio {}.\nServidor: {}.\n", anime.title, episode_selection + 1, server.title);
        }
        controller(episode_selection, episodes.to_vec(), anime);
    } else {
        println!("No se encontró ningun servidor útil.");
        mpv(anime, &episodes, choose_episode(&episodes), failed_server);
    };
}

fn controller(episode_selection: u16, episodes: Vec<String>, anime: &super::Anime) {
    let mut case: u8 = 0;
    let linkslen = episodes.len();

    loop {
        let mut prompt = String::from("");

        if episode_selection == 0 && episode_selection + 1 == linkslen as u16 {
        } else if episode_selection == 0 {
            prompt.push_str("[s] Siguiente\n");
            case = 1;
        } else if episode_selection + 1 == linkslen as u16 {
            prompt.push_str("[a] Anterior\n");
            case = 2;
        } else {
            prompt.push_str("[a] Anterior\n[s] Siguiente\n");
            case = 3;
        }

        prompt.push_str("[f] Probar otro servidor\n[l] Lista de episodios\n[b] Buscar anime\n[q] Salir\nEscoge una opción");

        let option: String = Input::new()
            .with_prompt(
                color_menu(&prompt)
            )
            .interact()
            .unwrap();

        let option = option.trim().to_lowercase();

        if option == "q" {
            std::process::exit(0);
        } else if option == "b" {
            break();
        } else if option == "f" {
            mpv(anime, &episodes, episode_selection, true);
        } else if option == "l" {
            mpv(anime, &episodes, choose_episode(&episodes), false);
        } else if [1, 3].contains(&case) && option == "s" {
            mpv(anime, &episodes, episode_selection + 1, false);
        } else if [2, 3].contains(&case) && option == "a" {
            mpv(anime, &episodes, episode_selection - 1, false);
        } else {
            println!("Escoge una opción valida.\n");
            continue;
        }
    }
}
