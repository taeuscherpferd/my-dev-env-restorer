try:
    from PIL import Image
    from PIL import ImageGrab

except ImportError:
    import Image

import pytesseract
import pyperclip


im = ImageGrab.grabclipboard()

text = ""
try:
    text = pytesseract.image_to_string(im)
except:
    text = "No text found, or no image in clipboard"

pyperclip.copy(text)