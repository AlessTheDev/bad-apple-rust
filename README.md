# Bad Apple with Rust Keywords
This is a program that converts black and white frames to rust keywords, then take a screenshot of your screen and store it in the rendered folder \
[![Youtube thumbnail](https://github.com/AlessTheDev/bad-apple-rust/assets/96922088/e579366f-6d94-4394-92d2-7780b67b8416)](https://www.youtube.com/watch?v=iGrrya5BxBM)


## How to render bad apple
1. Clone the repository \
`git clone ...`\
`cd bad-apple-rust` 
2. Download the bad apple intro
3. use ffmpeg to extract frames\
`ffmpeg -i input.mp4 frames/frame_%04d.png`
4. Open a program which renders text when a file changes (like vs code)
5. Run the program: `cargo run`
> If you want to visualize wihout the screenhots comment these 2 lines
```rs
thread::sleep(time::Duration::from_millis(200));
take_screenshot(frame);
```
6. Wait until the program renders everything
7. Put frames together: \
`cd ..` \
`ffmpeg -framerate 30 -pattern_type glob -i './rendered/*.png'   -c:v libx264 -pix_fmt yuv420p out2.mp4`

I reccomend reading the code to change it to your specific needs
