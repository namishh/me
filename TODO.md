## Frontend
- [x] add tailwind support (ASAP)
- [x] light mode and dark mode
- [x] a very cool pixely canvas effect -- see where i can add gol
- [x] table of contents on the right 
  - [x] generate it automatically 
  - [x] make the table of contents a timeline like animated lines
- [x] meta
  - [x] metadata
  - [x] images
- [x] 404 page
- [x] projects page
- [x] search bar

## Backend
- [x] fix resume redirects
- [x] meta
  - [x] generate images for blogs, notes, poems (imageproc)
  - [x] generate image for main page
- [x] images
  - [x] generate images for a given tweet -> https://react-tweet.vercel.app/api/tweet/{tweet_id}
- [x] codeblocks
  - [x] copy to clipboard button 
  - [x] better syntax highlighting of code - try if we can have custom themes
  - [x] file name
  - [x] line numbers
  - [x] highlighting specific lines
- [x] ocd commit: move /static/*.{png,jpg} to /static/images
- [x] add a search bar
- [x] name of the blog in the side bar
- [x] implement 404 pages
- [x] put internal links to all the headings, #, ##, ###, etc
- [x] rss feed
- [x] dockerize this shit

## Fixes
- [x] indentation of folders
- [x] optmise conway's game of life
  - [x] remove the lag when changing theme
  - [x] use requestAnimationFrame
  - [x] pause the game when the tab is not active
- [ ] search
  - [x] fix the search bar
  - [ ] maybe make search results rendered markdown
- [x] make it so 0 javascript for projects page

### commands to run it locally

```
npx @tailwindcss/cli -i ./static/input.css -o ./static/style.css --watch
```

```
cargo watch -w src -w Cargo.toml -w templates -w content -w static/_priv -w static/images -x run
```