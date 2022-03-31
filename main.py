import pyautogui
from PIL import Image
import mss
import time
import pyperclip
# yellow (252, 209, 0)


sct = mss.mss()

box = {"left": 340, "top": 333, "width": 1, "height": 1}
hintBox = {"left": 385, "top": 645, "width": 220, "height": 50}
time.sleep(2)




while True:
    img = sct.grab(box)
    mss.tools.to_png(img.rgb, img.size, output="test.png")
    if Image.open("test.png").getpixel((0, 0)) == (252, 209, 0):
        pyautogui.write(".hint")
        pyautogui.press("return")
        time.sleep(2)
        pyautogui.moveTo(384, 660)
        pyautogui.dragTo(790, 660, button="left")
        pyautogui.hotkey("ctrl", "c", interval=0.1)
        data = pyperclip.paste()
        try:
            new_data = data.split("is")[1]
        except:
            pass
        pyperclip.copy(new_data)
        pyautogui.hotkey("alt", "p", interval=0.1)
        time.sleep(1)
        data = pyperclip.paste()
        new_data = data.replace("p!c", ".c")
        pyperclip.copy(new_data)
        pyautogui.hotkey("ctrl", "v", interval=0.1)
        pyautogui.press("return")

    else:
        time.sleep(0.2)
        if Image.open("test.png").getpixel((0, 0)) != (252, 209, 0):
            pyautogui.write("X")
            pyautogui.press("return")
            time.sleep(1.5)