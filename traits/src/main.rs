pub trait Summary {
    fn summarize(&self) -> String;
    fn get_author(&self) -> String;
}

pub trait Location {
    fn get_location(&self) -> String {
        String::from("Unknown location")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    pub fn test_fn(&self, number: i16) -> String{
        if number == 1 {
            return String::from("ABC")
        }
        String::from("CBA")
    }

}

impl Location for NewsArticle {

}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::from("Weiter lesen ...")
    }

    fn get_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn print_articel(article: &impl Summary) {
    println!("The content of the article is: {}", article.summarize());
}

pub fn print_articel_information(article: &(impl Summary + Location)) {
    println!("{} wrote in {}: {}\n", article.get_author(), article.get_location(), article.summarize());
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let article = NewsArticle {
        headline: String::from("Pinguine gewinnen die Stanley-Cup-Meisterschaft!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("Die Pittsburgh Penguins sind erneut die beste \
                               Eishockeymannschaft in der NHL.",
        ),
    };

    print!("First result: {}\n", article.test_fn(2));
    print!("Second result: {}\n", article.test_fn(1));
    print!("Third result: {}\n", article.summarize());
    print!("The location of the article is: {}\n", article.get_location());
}
