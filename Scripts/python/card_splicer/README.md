# Summary
This is a tool I wrote for a friend to cut cards out of a pdf. It's very specific and I highly doubt that it will be useful to anyone else ever, but hey, you never know. 😂 Generated the base with ChatGPT o1, and tweaked it from there

# Dependencies
This tool depends on 
    - pdf2image
    - pillow
    - poppler

# Building
1. Create a venv:
`python -m venv ./`

2. Install deps:
`pip install pdf2image pillow`

(platform dependent extra steps):
macos: 
`brew install poppler`
Windows:
1. Download [this zip file](https://github.com/oschwartz10612/poppler-windows/releases/)
2. Move the extracted dir to `C:\Program Files`
3. Add the bin dir to windows path.



