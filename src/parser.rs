use std::io;
#[derive(Debug)]
struct FilmFields {
    title_name: String,
    title_href: String,
    mail_rate: String,
    imdb_rate: String
}

impl FilmFields {
    fn get_values(&self) -> Vec<&String> {
        let mut result = Vec::new();
        let fields = [
            &self.title_href, &self.title_name, &self.mail_rate, &self.imdb_rate
        ];

        for name in fields {
            result.push(name);
        }

        result
    }
}

pub fn start_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let num_page = get_num_page();

    for i in 0..num_page.parse::<i32>().unwrap(){
        let current_num_page = format!("{i}");
        let response = get_page(&current_num_page)?;
        if response.status() != 200 {
            println!("Page not found");
        }

        let response = response.text().unwrap();
        let films = parsing_html(&response);
        println!("{:?}", films);

        println!("{} complete!", i+1);
    }

    Ok(())
}


fn get_page(num_page: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let response = reqwest::blocking::get(format!("https://kino.mail.ru/cinema/all/?page={}", num_page));

    response
}


fn get_writer(num_page: &str) -> csv::Writer<std::fs::File> {
    let filename = format!("results_{}.csv", num_page);
    let path = std::path::Path::new(&filename);
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["url", "name", "mail_rate", "imdb_rate"])
        .unwrap();

    writer
}


fn get_num_page() -> String {
    println!("Enter how many pages: ");
    let mut num_page = String::new();
    io::stdin().read_line(&mut num_page).expect("Failed read to line");
    let num_page = String::from(num_page.trim());

    num_page
}


fn parsing_html(response: &String) -> Vec<FilmFields> {
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
            title_name: title_element.text().collect::<String>(),
            title_href: format!("{domain}{title_href_part}"),
            mail_rate: child.select(&mail_rate_selector).next().unwrap().text().collect::<String>(),
            imdb_rate: imdb_rate
        };
        
        films.push(film);
    }

    films
}
