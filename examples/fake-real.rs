use press_news::PressNews;
#[derive(PressNews)]
struct FakeNews{
    #[news] fake_news : String,
}

#[derive(PressNews)]
struct RealNews {
    #[news] real_news : String,
}

/* It will panic for this enum */
/*
#[derive(PressNews)]
enum News {
    Real(RealNews),
    Fake(FakeNews)
} */
fn main() {
    let r = RealNews{real_news : "This is a cruel world".to_string()};
    let f = FakeNews {fake_news : "This is a beautiful world".to_string()};
    println!("Time for real news");
    r.hello();
    r.press_news();
    println!("Time for fake news");
    f.hello();
    f.press_news();
}