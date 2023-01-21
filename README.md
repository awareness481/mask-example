This requires an older tensorflow version so we have to use `pyenv`

1. Install `pyenv` https://github.com/pyenv/pyenv
2. `pyenv install 3.7`
3. `pyenv global 3.7`
4. `pip install tensorflow==1.13.1 'h5py==2.10.0' keras==2.0.8 opencv-python-headless numpy scipy Pillow matplotlib scikit-image cython imgaug IPython`
5. Download `https://github.com/matterport/Mask_RCNN/releases/download/v2.0/mask_rcnn_coco.h5` and save it in the root folder of this project
6. In another directory run `git clone https://github.com/matterport/Mask_RCNN.git` and copy the `mrcnn` folder to the root folder of this project
7. `PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=python python3 i.py` (may not need the env based on your setup)

You may be prompted to install a library related to graphics, like `xcb` in order to display the image. Follow the instructions.

Ref: https://blog.paperspace.com/mask-r-cnn-in-tensorflow-2-0/
