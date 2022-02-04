# What it does
Przyczajony-tygrys hides a file in alpha channel of a png image.

# Example

## Encoding

We want to hide this text:

```
Who’s this crab, Ferris?
Ferris is the unofficial mascot of the Rust Community.
Many Rust programmers call themselves “Rustaceans,” a play on the word “crustacean".
We refer to Ferris with any pronouns “she,” “he,” “they,” “it,” etc.

Ferris is a name playing off of the adjective, “ferrous,” meaning of or pertaining to iron.
Since Rust often forms on iron, it seemed like a fun origin for our mascot’s name!
```

inside this image:

![Ferris](https://i.imgur.com/u2HeqH9.png)

We can run:

``` cargo run -- -i ferris.png -f text.txt -o encoded.png ```

and get:

![Ferris encoded](https://i.imgur.com/hiH08sA.png)

If the file is smaller than images capacity the pattern gets repeated.

## Decoding

We have the encoded image from previous example. We can run:

``` cargo run -- --decode -i encoded.png -o decoded.txt ```
