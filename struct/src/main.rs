#[derive(Debug)]
#[derive(PartialEq)]
enum Lang {
  English,
  Spanish,
  French,
  German,
  Chinese,
  Texan,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Hallo WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);

  let query_lang = Lang::Texan;
  for e in v {
    if e.lang == query_lang {
      println!("{:?} {}", e.lang, e.message);
    }
  }
}
