use std::io;
#[derive(Debug)]
pub struct FilmFields {
    pub page: i32,
    pub title_name: String,
    pub title_href: String,
    pub mail_rate: String,
    pub imdb_rate: String
}

pub fn start_parsing() -> Vec<FilmFields> {
    let num_page = get_num_page();
    let type_parsing = get_type_parsing();

    let result: Vec<FilmFields>;

    if type_parsing == "s" {
        result = parsing_page(&num_page);
    } else {
        result = parsing_pages(&num_page);
    }
    
    result
}

fn get_page(num_page: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let response = reqwest::blocking::get(format!("https://kino.mail.ru/cinema/all/?page={}", num_page));

    response
}

fn get_num_page() -> String {
    println!("Enter how many pages: ");
    let mut num_page = String::new();
    io::stdin().read_line(&mut num_page).expect("Failed read to line");
    let num_page = String::from(num_page.trim());

    num_page
}

fn get_type_parsing() -> String {
    println!("Enter type parsing (s - only one page, m - it will start with 0 and end with the number you entered): ");
    let mut type_parsing = String::new();
    io::stdin().read_line(&mut type_parsing).expect("Failed read to line");
    let type_parsing = String::from(type_parsing.trim());

    match type_parsing.as_str() {
        "m" => (),
        "s" => (),
        _ => panic!(r#"Enter only "s" or "m" mode"#)
    }

    type_parsing
}

fn parsing_page(num_page: & String) -> Vec<FilmFields> {
    let response = get_page(&num_page).unwrap();
    if response.status() != 200 {
        println!("Page not found");
    }

    let response = response.text().unwrap();
    let films = parsing_html(&response, &num_page.parse::<i32>().unwrap());

    println!("{} complete!", num_page);

    films
}

fn parsing_pages(num_page: & String) -> Vec<FilmFields> {
    let mut films: Vec<FilmFields> = Vec::new();

    for i in 1..num_page.parse::<i32>().unwrap() + 1 {
        let current_num_page = format!("{i}");
        let response = get_page(&current_num_page).unwrap();
        if response.status() != 200 {
            println!("Page not found");
        }

        let response = response.text().unwrap();
        let page = parsing_html(&response, &i);
        films.extend(page);

        println!("{} complete!", i);
    }

    films
}

fn parsing_html(response: &String, n_page: & i32) -> Vec<FilmFields> {
    let mut films: Vec<FilmFields> = Vec::new();

    let document = scraper::Html::parse_document(&response);
    let cols_selector = scraper::Selector::parse(r#"div[class="cols cols_percent cols_margin"]"#).unwrap();
    let wrapper_selector = scraper::Selector::parse(r#"div[class="cols__wrapper"]"#).unwrap();
    let child_selector = scraper::Selector::parse(r#".cols__column"#).unwrap();
    let title_selector = scraper::Selector::parse(r#"span[class="text text_block text_fixed text_light_large"]"#).unwrap();
    let href_selector = scraper::Selector::parse(r#"a.link"#).unwrap();
    let mail_rate_selector = scraper::Selector::parse(".p-rate-flag__text").unwrap();
    let imdb_rate_selector = scraper::Selector::parse(".p-rate-flag__imdb-text").unwrap();

    let table = document.select(&cols_selector).next().unwrap();
    let cells = table.select(&wrapper_selector).next().unwrap();
    let childs = cells.select(&child_selector);

    let domain = "https://kino.mail.ru".to_owned();
    for child in childs {
        let title_element = child.select(&title_selector).next().unwrap();
        let title_href_part = title_element.select(&href_selector).next().unwrap().attr("href").unwrap();
        let imdb_rate = match child.select(&imdb_rate_selector).next() {
            None => String::from("None"),
            Some(v) => v.text().collect::<String>()
        };

        let film = FilmFields {
            page: *n_page,
            title_name: title_element.text().collect::<String>(),
            title_href: format!("{domain}{title_href_part}"),
            mail_rate: child.select(&mail_rate_selector).next().unwrap().text().collect::<String>(),
            imdb_rate: imdb_rate
        };
        
        films.push(film);
    }

    films
}
