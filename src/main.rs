extern crate imdb_index;

mod download;

use imdb_index::{Index, IndexBuilder, MediaEntity, Query, Rating, SearchResults, Searcher};
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    if !fs::metadata("data").is_ok() {
        println!("Downloading imdb data...");
        download::download_all("data").unwrap();
        println!("Building indices... This will take a while.");
        IndexBuilder::new().create("data", "index").unwrap();
        println!("Done building, ready to search");
    }

    loop {
        println!("Enter a title to search: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut results = search_imdb(&input).into_vec();
                //results.dedup();
                //let valid = results
                //    .into_iter()
                //    .filter(|res| res.clone().into_pair().1.rating().is_some());
                let temp = Rating {
                    id: String::from("X"),
                    rating: 0.0,
                    votes: 0,
                };

                results.sort_by(|a, b| {
                    a.value()
                        .rating()
                        .unwrap_or(&temp)
                        .votes
                        .cmp(&b.value().rating().unwrap_or(&temp).votes)
                });
                /*
                let v = results.last().unwrap().clone();
                let (rating, result) = v.into_pair();
                let title = result.title();
                let r = result.rating().unwrap_or(&temp);
                println!(
                    "{} {} {} {} https://www.imdb.com/title/{}/\n",
                    rating, title.title, title.genres, r.votes, title.id
                );
                */
                for res in results {
                    let (rating, result) = res.into_pair();
                    let title = result.title();
                    let r = result.rating().unwrap_or(&temp);
                    println!(
                        "{} {} {} {} https://www.imdb.com/title/{}/\n",
                        rating, title.title, title.genres, r.votes, title.id
                    );
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn search_imdb(query: &str) -> SearchResults<MediaEntity> {
    println!("starting search with {}", query);
    let q: Query = Query::new().name(query);
    let data_dir: &Path = Path::new("./data/");
    let index_dir: &Path = Path::new("./index/");
    let opened_index = Index::open(data_dir, index_dir).unwrap();
    let mut searcher = Searcher::new(opened_index);
    searcher.search(&q).unwrap()
}
