import os
import segno
from PIL import Image

qrcode = segno.make_qr("Hello, World")
qrcode.save("qr.ppm")
os.system("convert qr.ppm -compress none qr.ppm")
