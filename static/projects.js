const projects = [
    {
        "type": "Flora",
        "array": [
            {
                name: "aster",
                desc: "redefining the way to collaborate with people on youtube channels.",
                tech: ["solid start"],
                link: "https://flora.tf"
            },
            {
                name: "orchid",
                desc: "quick, easy to use and optimised meme // profile picture editor.",
                tech: ["react", "flask"],
                link: "https://orchid.rex.wf"
            },
            {
                name: "sakura",
                desc: "beautiful, fast and uniquely generated avatars as a microservice",
                tech: ["golang"],
                link: "https://github.com/floraorg/sakura"
            },
            {
                name: "faux",
                desc: "minimal, fast and eyecandy placeholders as a microservice",
                tech: ["golang"],
                link: "https://github.com/floraorg/faux"
            },
        ]
    },
    {
        "type": "Good Projects",
        "array": [
            {
                name: "ascendant",
                desc: "wip 2d club penguin card jutsu style game made with rayilb",
                tech: ["zig", "raylib"],
            },
            {
                name: "holmes",
                desc: "0 js, 100% golang and templ batteries included starter kit for  crypt hunts",
                tech: ["templ", "golang"],
            },
            {
                name: "me",
                desc: "my own personal static site generator written in rust. for this site",
                tech: ["rust", "actix"],
            },
            {
                name: "pixie",
                desc: "wasm based small lightroom like image editor. only canvas and rust",
                tech: ["rust", "next"
                ],
            },
            {
                name: "biotrack",
                desc: "an online personal health diary to keep track of you life with ai assistance",
                tech: ["golang", "js", "gemini"],
            },
            {
                name: "pound",
                desc: "terminal text editor written entirely in C (with vim motions).",
                tech: ["c"],
            },
            {
                name: "prism",
                desc: "neovim plugin to easily have custom colorschemes with caching for speed",
                tech: ["neovim", "lua"],
            },
        ]
    },
    {
        "type": "Decent Projects",
        "array": [
            {
                name: "webby",
                desc: "web server written entirely from scratch in c. (with a basic todo app)",
                tech: ["c"],
            },
            {
                name: "lockin",
                desc: "24x7 lofi radio plus general productivity website",
                tech: ["next", "tailwind"],
                link: "https://cafe.namishh.me",
            },
            {
                name: "lovbyte",
                desc: "Dating app for programmers rich with features, minimal by design. ",
                tech: ["remix", "tailwind"],
            },
            {
                name: "neuing",
                desc: "a neural network written entirely in golang without external modules",
                tech: ["golang", "maths"],
            },
            {
                name: "shawty",
                desc: "a url shortener written entirely in pure C and HTMX",
                tech: ["c", "htmx"],
            },
            {
                name: "techfestweb",
                desc: "a beautiful, modern, sleek and responsive website template for techfests",
                tech: ["remix", "tailwind"],
                link: "https://techfestweb.vercel.app",
            },
            {
                name: "hacknio",
                desc: "almost redid the entire ui for this hackernews frontend",
                tech: ["next", "tailwind"],
                link: "https://hacknio.vercel.app",
            },
            {
                name: "cyquest",
                desc: "website for the cyquest 2023, with a complete desktop like interface",
                tech: ["react", "tailwind"],
            },
            {
                name: "bubble",
                desc: "a personal diary app with database and authentication!",
                tech: ["html", "flask"],
            },
            {
                name: "zenote",
                desc: "desktop app to create markdown notes locally without any uneeded bs",
                tech: ["tauri", "react"],
            },
        ]
    },
    {
        "type": "Discord Bots",
        "array": [
            {
                name: "scuffword",
                desc: "password game implmentation in c in the form of discord bot",
                tech: ["c"],
            },
            {
                name: "linear",
                desc: "discord bot that can be used to host cryptic hunt events. (with hints)",
                tech: ["rust"],
            },
            {
                name: "remellow",
                desc: "general purpose discord bot with discord.js",
                tech: ["javascript"],
            },
        ]
    },
    {
        "type": "Configs",
        "array": [
            {
                name: "crystal",
                desc: "nix dotfiles for my daily driver. comes with awesome as the window manager",
                tech: ["nix", "ricing"],
            },
            {
                name: "kodo",
                desc: "neovim configuration that is speedy, usable, and very good looking",
                tech: ["neovim", "lua"],
            },
        ]
    },
]

document.addEventListener('DOMContentLoaded', () => {
    const container = document.getElementById('projects-container');

    projects.forEach(category => {
        // Create category section with title
        const categoryHTML = `
                    <div class="">
                        <h2 class="text-xl font-bold mb-4">${category.type}</h2>
                        <div class="flex flex-wrap">
                            ${category.array.map((project, i) => `
                                <a href="${project.link ? project.link : "https://github.com/namishh/" + project.name}" class="w-full md:w-1/2 md:py-0 py-2 ${i % 2 == 0 ? "md:pr-2 pr-0" : "pl-0 md:pl-2"} mb-0 md:mb-4">
                                    <div class="border-neutral-300 dark:border-neutral-800 border-[1px] p-4 h-full">
                                        <h3 class="text-n font-bold dark:text-white text-neutral-800 mb-2">${project.name}</h3>
                                        <p class="normal-text  text-sm mb-4">${project.desc}</p>
                                        <div class="flex flex-wrap gap-2">
                                            ${project.tech.map(tech => `
                                                <span class="font-mono dark:text-neutral-200 text-neutral-600 bg-neutral-200 dark:bg-neutral-800 text-xs px-2 py-1 rounded">${tech}</span>
                                            `).join('')}
                                        </div>
                                    </div>
                                </a>
                            `).join('')}
                        </div>
                    </div>
                `;

        // Add to container
        container.insertAdjacentHTML('beforeend', categoryHTML);
    });
}
);