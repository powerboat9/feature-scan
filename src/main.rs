fn str_escape(s: &str) -> String {
    let mut out = String::new();
    let mut quote_open = true;
    for byte in s.bytes() {
        out += match byte {
            b'`' => {
                let v = quote_open;
                quote_open = !quote_open;
                if v {
                    "%<"
                } else {
                    "%>"
                }
            },
            b'%' => "%%",
            32..=126 => {
                std::str::from_utf8(std::slice::from_ref(&byte)).unwrap()
            },
            _ => panic!("unrecognized byte")
        }
    }
    out
}

fn get_state(s: rustc_feature::State) -> String {
    fn convert_opt(s_opt: Option<&str>) -> String {
        match s_opt {
            Some(s) => "SOME (\"".to_owned() + str_escape(s).as_str() + "\")",
            None => "NONE".to_owned()
        }
    }

    use rustc_feature::State::*;

    match s {
        Accepted => "FEATURE_STATE_ACCEPTED".to_owned(),
        Active {..}=> "FEATURE_STATE_ACTIVE".to_owned(),
        Removed { reason: r } => {
            "FEATURE_STATE_REMOVED_".to_owned() + convert_opt(r).as_str()
        },
        Stabilized { reason: r } => {
            "FEATURE_STATE_STABILIZED_".to_owned() + convert_opt(r).as_str()
        }
    }
}

fn get_issue(feat: &rustc_feature::Feature) -> String {
    let issue = rustc_feature::find_feature_issue(
        feat.name, rustc_feature::GateIssue::Language
    );
    match issue {
        Some(n) => {
            "FEATURE_ISSUE_SOME (".to_owned() + n.to_string().as_str() + ")"
        },
        None => "FEATURE_ISSUE_NONE".to_owned()
    }
}

fn get_edition(edition: Option<rustc_span::edition::Edition>) -> &'static str {
    use rustc_span::edition::Edition::*;
    match edition {
        None => "FEATURE_EDITION_NONE",
        Some(Edition2015) => "FEATURE_EDITION_2015",
        Some(Edition2018) => "FEATURE_EDITION_2018",
        _ => panic!("unknown edition")
    }
}

fn process_feat(feat: &rustc_feature::Feature) {
    println!(
        "FEATURE ({}, {}, {}, \"{}\", {}, {})",
        get_state(feat.state),
        feat.name.as_str(),
        feat.name.as_str().to_ascii_uppercase().as_str(),
        feat.since,
        get_issue(feat),
        get_edition(feat.edition)
    );
}

pub fn main() {
    println!("// state, name, NAME, since, issue, edition");
    println!();

    rustc_span::with_default_session_globals(|| {
        for feat in rustc_feature::ACTIVE_FEATURES {
            process_feat(feat);
        }
        println!();
        for feat in rustc_feature::ACCEPTED_FEATURES {
            process_feat(feat);
        }
        println!();
        for feat in rustc_feature::REMOVED_FEATURES {
            process_feat(feat);
        }
    });
}
