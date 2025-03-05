import sys
from paddleocr import PaddleOCR

print(sys.argv)
print(sys.argv[0])

print("python launched successfully")
print("ok ok")

img_bytes = sys.stdin.buffer.read()
# print(type(img_bytes))

ocr = PaddleOCR(use_angle_cls=True, lang='ch') # need to run only once to load model into memory
# img_path = '/tmp/temp_image.png'
result = ocr.ocr(img_bytes, det=False, cls=True)
for idx in range(len(result)):
    res = result[idx]
    for line in res:
        # make somewhat simmilar output, so same regex can be used
        print(" ppocr INFO: ('" + line[0] + "', " + str(line[1]) + ") ")