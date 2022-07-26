import requests
import pyperclip

#resp = requests.get("http://192.168.1.50:22555")
#print(resp.text)

def copy_clipboard():
    return pyperclip.paste()

json_s = {"text": copy_clipboard()}
resp = requests.post("http://192.168.1.50:22555/copy", json=json_s)
print(resp)

