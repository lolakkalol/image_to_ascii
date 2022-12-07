# Will determin the averege pixel intensity in the symbols of the variable messages by printing them to an image and averaging them

from PIL import Image, ImageDraw, ImageFont
import numpy as np 

def bubbleSort(arr):
    n = len(arr)
    for i in range(n):
        for j in range(0, n-i-1):
            if arr[j][1] > arr[j+1][1]:
                arr[j], arr[j+1] = arr[j+1], arr[j]

width = 512 - 100
height = 512
message =  '.,-*^\/()#@"=~$£'#'!"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~'

result = [(' ', 0)]

print(result)

for i in range(0, len(message)):
    img = Image.new("RGB", (width, height), color='white')
    font = ImageFont.truetype("monospace-821.otf", 560)
    imgDraw = ImageDraw.Draw(img)
    imgDraw.text((0, -130), message[i], font=font, fill=(0, 0, 0))

    img = img.convert("L")

    avg = 0

    for y in range(0, 512):
        for x in range(0, 412):
            avg += img.getpixel((x, y)) / 255
            x += 1

    avg /= (412*512)
    avg = 1 - avg
    result.append((message[i], avg))
    
bubbleSort(result)

print(result)