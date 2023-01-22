This requires an older tensorflow version so we have to use `pyenv`

1. Install `pyenv` https://github.com/pyenv/pyenv
2. `pyenv install 3.8.12`
3. `pyenv global 3.8.12`
4. Run `python -m venv .env`
5. Run `source .env/bin/activate`
6. Run `pip install -r requirements.txt`
7. Run `wget https://github.com/matterport/Mask_RCNN/releases/download/v2.0/mask_rcnn_coco.h5`
8. In another directory run `git clone https://github.com/mrk1992/mask-rcnn-tf2-us` and copy the `mrcnn` folder to the root folder of this project

### Development

1. `maturin develop`
2. `PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=python python3 model.py`

### Release build

1. `maturin build --release`
2. `pip install <path>/<to>/<wheel>.whl --force-reinstall`
3. `PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=python python3 -O model.py`

You may be prompted to install a library related to graphics, like `xcb` in order to display the image. Follow the instructions.

Ref: https://blog.paperspace.com/mask-r-cnn-in-tensorflow-2-0/
