#!/usr/bin/python3
#!/usr/bin/env python3
import sys
from paddleocr import PaddleOCR

print(sys.argv)
print(sys.argv[0])

print("python launched successfully")
print("ok ok")

ocr = PaddleOCR(use_angle_cls=True, lang='en') # need to run only once to load model into memory
img_path = 'PaddleOCR/doc/imgs_words_en/word_10.png'
result = ocr.ocr(img_path, det=False, cls=True)
for idx in range(len(result)):
    res = result[idx]
    for line in res:
        print(line)