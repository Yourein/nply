<center>
    <h1 style="
        font-weight:bold;
        background:linear-gradient(to right, #1db954 25%, #dea584 60%);
        -webkit-background-clip:text;
        -webkit-text-fill-color: transparent;
    ">
        nply
    </h1>
</center>

# What is this?

This is a Rust based cli application that allows you to share your \#nowplaying on Spotify.  

This application is unfortunately under in reconstruction since $\mathbb{X}$ (Twitter) restricting its API.  
You can not execute this program without `TWITTER_API_KEY` and `TWITTER_API_SECRET` in your `.env` file.

# Milestone

- Support Misskey API (High Priority)
  - This change will allow you to share your \#nowplaying to the Fediverse via a misskey server.
- Change the code to not panic if the `TWITTER_API_KEY` and `TWITTER_API_SECRET` are missing (High Priority)
- Change the code to embed your API keys to a executable by setting file (Mid Priority)
  - This change will allow you to use this application without `cargo run`, just `cargo install` once.

# Install

Currently, this application **does not** have a option to use `cargo install` since this application uses `dotenv` crate and `.env` file.

## 1. Clone the repo

```
git clone git@github.com:Yourein/nply.git
```

## 2. Setup your `.env` file

```
touch .env
vim ./.env
```

Then copy this to your `.env` file.

```
SPOTIFY_API_KEY=
SPOTIFY_API_SECRET=
TWITTER_API_KEY=
TWITTER_API_SECRET=
```

Fill the right side of the equals.

## 3. run

```
cargo run
```

enjoy!