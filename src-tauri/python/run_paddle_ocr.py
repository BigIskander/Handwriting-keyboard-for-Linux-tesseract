import sys
from paddleocr import PaddleOCR

img_bytes = sys.stdin.buffer.read()
lang = sys.argv[1]

ocr = PaddleOCR(use_angle_cls=True, lang=lang) # need to run only once to load model into memory
result = ocr.ocr(img_bytes, det=False, cls=True)
for idx in range(len(result)):
    res = result[idx]
    for line in res:
        # make somewhat simmilar output, so same regex can be used
        print(" ppocr INFO: ('" + line[0] + "', " + str(line[1]) + ") ")