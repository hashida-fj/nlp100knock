use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Token {
    surface: String,
    base: String,
    pos: String,
    pos1: String,
}

type Sentence = Vec<Token>;
type Document = Vec<Sentence>;

trait MecabSentence {
    fn surfaces(&self) -> Vec<String>;
    fn bases(&self) -> Vec<String>;
}

impl MecabSentence for Sentence {
    fn surfaces(&self) -> Vec<String> {
        self.iter()
            .filter(|token| token.pos == "動詞")
            .map(|token| token.surface.clone())
            .collect()
    }

    fn bases(&self) -> Vec<String> {
        self.iter()
            .filter(|token| token.pos == "動詞")
            .map(|token| token.base.clone())
            .collect()
    }
}

pub fn load_mecab_file(path: &str) -> std::io::Result<Document> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sentence: Sentence = Vec::new();
    let mut sentences: Document = Vec::new();

    for line in contents.lines() {
        let mecab_elems: Vec<_> = line.split("\t").collect();

        if mecab_elems[0] == "EOS" {
            if sentence.len() > 0 {
                sentences.push(sentence);
                sentence = Vec::new();
            }
        } else {
            let elems: Vec<_> = mecab_elems[1].split(",").collect();
            sentence.push(Token {
                surface: mecab_elems[0].to_string(),
                base: elems[6].to_string(),
                pos: elems[0].to_string(),
                pos1: elems[1].to_string(),
            });
        }
    }
    return Ok(sentences);
}

pub fn print_a_no_b(doc: &Document) {
    for sentence in doc {
        for words in sentence.windows(3) {
            match words {
                [a, no, b]
                    if no.surface == "の"
                        && no.pos == "助詞"
                        && a.pos == "名詞"
                        && b.pos == "名詞" =>
                {
                    println!("{:?},{:?},{:?}", a.surface, no.surface, b.surface)
                }
                _ => continue, // slice の個数は言語的に制限できないってことか
            }
        }
    }
}

pub fn print_noun_concatenation(doc: &Document) {
    for sentence in doc {
        sentence
            .split(|token| token.pos != "名詞")
            .for_each(|nouns| {
                if nouns.len() >= 2 {
                    println!(
                        "{}",
                        nouns
                            .into_iter()
                            .map(|n| &n.base[..])
                            .collect::<Vec<_>>()
                            .join("")
                    )
                }
            })
    }
}

pub fn word_frequency(doc: &Document) {
    let mut ret: HashMap<&str, usize> = HashMap::new();
    for sentence in doc {
        for tok in sentence {
            if tok.pos == "記号" {
                continue;
            };
            ret.insert(&tok.base[..], ret.get(&tok.base[..]).unwrap_or(&0) + 1);
        }
    }

    let mut tmp = ret.iter().collect::<Vec<_>>();
    tmp.sort_by(|a, b| b.1.cmp(&a.1));
    for v in tmp {
        println!("{:?}", v)
    }
}

pub fn run_section4() {
    let document = load_mecab_file("./assets/neko.txt.mecab").unwrap();

    // for sentence in &document {
    //     println!("{:?}", sentence.surfaces())
    // }

    // for sentence in &document {
    //     println!("{:?}", sentence.bases())
    // }

    // print_a_no_b(&document);
    // print_noun_concatenation(&document);
    word_frequency(&document);
}
