use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub desc: String,
    pub tech: Vec<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectCategory {
    pub r#type: String,
    pub array: Vec<Project>,
}

pub fn get_projects() -> Vec<ProjectCategory> {
    vec![
        ProjectCategory {
            r#type: "Flora".to_string(),
            array: vec![
                Project {
                    name: "aster".to_string(),
                    desc: "redefining the way to collaborate with people on youtube channels.".to_string(),
                    tech: vec!["solid start".to_string()],
                    link: Some("https://flora.tf".to_string()),
                },
                Project {
                    name: "orchid".to_string(),
                    desc: "quick, easy to use and optimised meme // profile picture editor.".to_string(),
                    tech: vec!["react".to_string(), "flask".to_string()],
                    link: Some("https://orchid.rex.wf".to_string()),
                },
                Project {
                    name: "sakura".to_string(),
                    desc: "beautiful, fast and uniquely generated avatars as a microservice".to_string(),
                    tech: vec!["golang".to_string()],
                    link: Some("https://github.com/floraorg/sakura".to_string()),
                },
                Project {
                    name: "faux".to_string(),
                    desc: "minimal, fast and eyecandy placeholders as a microservice".to_string(),
                    tech: vec!["golang".to_string()],
                    link: Some("https://github.com/floraorg/faux".to_string()),
                },
            ],
        },
        ProjectCategory {
            r#type: "Good Projects".to_string(),
            array: vec![
                Project {
                    name: "ascendant".to_string(),
                    desc: "wip 2d club penguin card jutsu style game made with rayilb".to_string(),
                    tech: vec!["zig".to_string(), "raylib".to_string()],
                    link: None,
                },
                Project {
                    name: "holmes".to_string(),
                    desc: "0 js, 100% golang and templ batteries included starter kit for  crypt hunts".to_string(),
                    tech: vec!["templ".to_string(), "golang".to_string()],
                    link: None,
                },
                Project {
                    name: "me".to_string(),
                    desc: "my own personal blazingly fast website written in rust".to_string(),
                    tech: vec!["rust".to_string(), "actix".to_string()],
                    link: None,
                },
                Project {
                    name: "pixie".to_string(),
                    desc: "wasm based small lightroom like image editor. only canvas and rust".to_string(),
                    tech: vec!["rust".to_string(), "next".to_string()],
                    link: None,
                },
                Project {
                    name: "biotrack".to_string(),
                    desc: "an online personal health diary to keep track of you life with ai assistance".to_string(),
                    tech: vec!["golang".to_string(), "js".to_string(), "gemini".to_string()],
                    link: None,
                },
                Project {
                    name: "pound".to_string(),
                    desc: "terminal text editor written entirely in C (with vim motions).".to_string(),
                    tech: vec!["c".to_string()],
                    link: None,
                },
                Project {
                    name: "prism".to_string(),
                    desc: "neovim plugin to easily have custom colorschemes with caching for speed".to_string(),
                    tech: vec!["neovim".to_string(), "lua".to_string()],
                    link: None,
                },
            ],
        },
        ProjectCategory {
            r#type: "Decent Projects".to_string(),
            array: vec![
                Project {
                    name: "webby".to_string(),
                    desc: "web server written entirely from scratch in c. (with a basic todo app)".to_string(),
                    tech: vec!["c".to_string()],
                    link: None,
                },
                Project {
                    name: "lockin".to_string(),
                    desc: "24x7 lofi radio plus general productivity website".to_string(),
                    tech: vec!["next".to_string(), "tailwind".to_string()],
                    link: Some("https://cafe.namishh.me".to_string()),
                },
                Project {
                    name: "lovbyte".to_string(),
                    desc: "Dating app for programmers rich with features, minimal by design. ".to_string(),
                    tech: vec!["remix".to_string(), "tailwind".to_string()],
                    link: None,
                },
                Project {
                    name: "neuing".to_string(),
                    desc: "a neural network written entirely in golang without external modules".to_string(),
                    tech: vec!["golang".to_string(), "maths".to_string()],
                    link: None,
                },
                Project {
                    name: "shawty".to_string(),
                    desc: "a url shortener written entirely in pure C and HTMX".to_string(),
                    tech: vec!["c".to_string(), "htmx".to_string()],
                    link: None,
                },
                Project {
                    name: "techfestweb".to_string(),
                    desc: "a beautiful, modern, sleek and responsive website template for techfests".to_string(),
                    tech: vec!["remix".to_string(), "tailwind".to_string()],
                    link: Some("https://techfestweb.vercel.app".to_string()),
                },
                Project {
                    name: "hacknio".to_string(),
                    desc: "almost redid the entire ui for this hackernews frontend".to_string(),
                    tech: vec!["next".to_string(), "tailwind".to_string()],
                    link: Some("https://hacknio.vercel.app".to_string()),
                },
                Project {
                    name: "cyquest".to_string(),
                    desc: "website for the cyquest 2023, with a complete desktop like interface".to_string(),
                    tech: vec!["react".to_string(), "tailwind".to_string()],
                    link: None,
                },
                Project {
                    name: "bubble".to_string(),
                    desc: "a personal diary app with database and authentication!".to_string(),
                    tech: vec!["html".to_string(), "flask".to_string()],
                    link: None,
                },
                Project {
                    name: "zenote".to_string(),
                    desc: "desktop app to create markdown notes locally without any uneeded bs".to_string(),
                    tech: vec!["tauri".to_string(), "react".to_string()],
                    link: None,
                },
            ],
        },
        ProjectCategory {
            r#type: "Discord Bots".to_string(),
            array: vec![
                Project {
                    name: "scuffword".to_string(),
                    desc: "password game implmentation in c in the form of discord bot".to_string(),
                    tech: vec!["c".to_string()],
                    link: None,
                },
                Project {
                    name: "linear".to_string(),
                    desc: "discord bot that can be used to host cryptic hunt events. (with hints)".to_string(),
                    tech: vec!["rust".to_string()],
                    link: None,
                },
                Project {
                    name: "remellow".to_string(),
                    desc: "general purpose discord bot with discord.js".to_string(),
                    tech: vec!["javascript".to_string()],
                    link: None,
                },
            ],
        },
        ProjectCategory {
            r#type: "Configs".to_string(),
            array: vec![
                Project {
                    name: "crystal".to_string(),
                    desc: "nix dotfiles for my daily driver. comes with awesome as the window manager".to_string(),
                    tech: vec!["nix".to_string(), "ricing".to_string()],
                    link: None,
                },
                Project {
                    name: "kodo".to_string(),
                    desc: "neovim configuration that is speedy, usable, and very good looking".to_string(),
                    tech: vec!["neovim".to_string(), "lua".to_string()],
                    link: None,
                },
            ],
        },
    ]
}