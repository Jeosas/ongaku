<div align="center">
    <h1>Ongaku</h1>
</div>

This little utility tool helps you manage your music library when using [yt-dlp](https://github.com/yt-dlp/yt-dlp).

Status: WIP

### Disclaimer

This tool has been developed mainly for practice purposes and may seem like a huge work for a small problem.

It should be _overkill_ for most cases.

A simple solution would be to add a bunch of URLs in a file and run

```console
$ cat url_list.txt > xargs yt-dlp \
    -q \
    -ciw \
    -f "bestaudio/best" \
    --extract-audio \
    -o "%(artist)s/%(title)s.%(ext)s"
```

This tool was created from several frustration with this method:

- Downloads are sequential and thus slower than it could be with a good internet down-link,
- I'd like to have more control on how my library is sorted.

## Introduction

todo!()

## Dependencies

- `yt-dlp`
- `ffmpeg` (might be optional, used to extract audio from videos)

## License

Ongaku is MIT-licensed. For more information check the [LICENSE](./LICENSE) file.
