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

You can post to Misskey (and its fork; using same api endpoint and schema) and X.

![](https://firebasestorage.googleapis.com/v0/b/kdatabase-1088a.appspot.com/o/nply%2FScreenshot%20from%202023-11-02%2009-41-33.png?alt=media)

# Milestone

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
MISSKEY_API_TOKEN=
MISSKEY_HOST=
```

Fill the right side of the equals.
If you do not want to post to X (or Misskey), you can leave its API key empty.

## 3. run

```
cargo run
```

enjoy!
