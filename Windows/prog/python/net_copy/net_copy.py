#!python
import sys
import time
import requests
import pyperclip
import pyautogui as pya

if len(sys.argv) == 1:
    sys.exit()

copy_or_paste = sys.argv[1]

def copy_clipboard():
    pya.hotkey('ctrl', 'c')
    time.sleep(.01)
    return pyperclip.paste()

def paste_clipboard(s):
    pyperclip.copy(s)
    pya.hotkey('ctrl', 'v')

if copy_or_paste == "copy":
    json_s = {"text": copy_clipboard()}
    resp = requests.post("http://192.168.1.15:22555/copy", json=json_s)
    
else:
    resp = requests.get("http://192.168.1.15:22555")
    paste_clipboard(resp.text)

